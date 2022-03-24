//! Coordinate system names:
//! * content: size of contents (generally large; that's why we want scroll bars)
//! * outer: size of scroll area including scroll bar(s)
//! * inner: excluding scroll bar(s). The area we clip the contents to.

#![allow(clippy::needless_range_loop)]

use eframe::egui::*;

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct State {
    /// Positive offset means scrolling down/right
    pub offset: Vec2,

    show_scroll: [bool; 2],

    /// Momentum, used for kinetic scrolling
    #[cfg_attr(feature = "serde", serde(skip))]
    pub vel: Vec2,

    /// Mouse offset relative to the top of the handle when started moving the handle.
    scroll_start_offset_from_top_left: [Option<f32>; 2],

    /// Is the scroll sticky. This is true while scroll handle is in the end position
    /// and remains that way until the user moves the scroll_handle. Once unstuck (false)
    /// it remains false until the scroll touches the end position, which reenables stickiness.
    scroll_stuck_to_end: [bool; 2],
}

impl Default for State {
    fn default() -> Self {
        Self {
            offset: Vec2::ZERO,
            show_scroll: [false; 2],
            vel: Vec2::ZERO,
            scroll_start_offset_from_top_left: [None; 2],
            scroll_stuck_to_end: [true; 2],
        }
    }
}

impl State {
    pub fn load(ctx: &Context, id: Id) -> Option<Self> {
        ctx.data().get_persisted(id)
    }

    pub fn store(self, ctx: &Context, id: Id) {
        ctx.data().insert_persisted(id, self);
    }
}

pub struct ScrollAreaOutput<R> {
    /// What the user closure returned.
    pub inner: R,

    /// `Id` of the `ScrollArea`.
    pub id: Id,

    /// The current state of the scroll area.
    pub state: State,

    /// Where on the screen the content is (excludes scroll bars).
    pub inner_rect: Rect,
}

/// Add vertical and/or horizontal scrolling to a contained [`Ui`].
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// egui::ScrollArea::vertical().show(ui, |ui| {
///     // Add a lot of widgets here.
/// });
/// # });
/// ```
#[derive(Clone, Debug)]
#[must_use = "You should call .show()"]
pub struct ScrollArea {
    /// Do we have horizontal/vertical scrolling?
    has_bar: [bool; 2],
    auto_shrink: [bool; 2],
    max_size: Vec2,
    min_scrolled_size: Vec2,
    always_show_scroll: bool,
    id_source: Option<Id>,
    offset_x: Option<f32>,
    offset_y: Option<f32>,
    /// If false, we ignore scroll events.
    scrolling_enabled: bool,

    /// If true for vertical or horizontal the scroll wheel will stick to the
    /// end position until user manually changes position. It will become true
    /// again once scroll handle makes contact with end.
    stick_to_end: [bool; 2],
}

impl ScrollArea {
    /// Create a vertical scroll area.
    pub fn vertical() -> Self {
        Self::new([false, true])
    }

    /// Create a scroll area where you decide which axis has scrolling enabled.
    /// For instance, `ScrollAre::new([true, false])` enable horizontal scrolling.
    pub fn new(has_bar: [bool; 2]) -> Self {
        Self {
            has_bar,
            auto_shrink: [true; 2],
            max_size: Vec2::INFINITY,
            min_scrolled_size: Vec2::splat(64.0),
            always_show_scroll: false,
            id_source: None,
            offset_x: None,
            offset_y: None,
            scrolling_enabled: true,
            stick_to_end: [false; 2],
        }
    }

    /// A source for the unique `Id`, e.g. `.id_source("second_scroll_area")` or `.id_source(loop_index)`.
    pub fn id_source(mut self, id_source: impl std::hash::Hash) -> Self {
        self.id_source = Some(Id::new(id_source));
        self
    }

    /// Set the vertical scroll offset position.
    ///
    /// See also: [`Self::scroll_offset`], [`Ui::scroll_to_cursor`](crate::ui::Ui::scroll_to_cursor) and
    /// [`Response::scroll_to_me`](crate::Response::scroll_to_me)
    pub fn vertical_scroll_offset(mut self, offset: f32) -> Self {
        self.offset_y = Some(offset);
        self
    }
}

struct Prepared {
    id: Id,
    state: State,
    has_bar: [bool; 2],
    auto_shrink: [bool; 2],
    /// How much horizontal and vertical space are used up by the
    /// width of the vertical bar, and the height of the horizontal bar?
    current_bar_use: Vec2,
    always_show_scroll: bool,
    /// Where on the screen the content is (excludes scroll bars).
    inner_rect: Rect,
    content_ui: Ui,
    /// Relative coordinates: the offset and size of the view of the inner UI.
    /// `viewport.min == ZERO` means we scrolled to the top.
    viewport: Rect,
    scrolling_enabled: bool,
    stick_to_end: [bool; 2],
}

impl ScrollArea {
    fn begin(self, ui: &mut Ui) -> Prepared {
        let Self {
            has_bar,
            auto_shrink,
            max_size,
            min_scrolled_size,
            always_show_scroll,
            id_source,
            offset_x,
            offset_y,
            scrolling_enabled,
            stick_to_end,
        } = self;

        let ctx = ui.ctx().clone();

        let id_source = id_source.unwrap_or_else(|| Id::new("scroll_area"));
        let id = ui.make_persistent_id(id_source);
        let mut state = State::load(&ctx, id).unwrap_or_default();

        state.offset.x = offset_x.unwrap_or(state.offset.x);
        state.offset.y = offset_y.unwrap_or(state.offset.y);



        let available_outer = ui.available_rect_before_wrap();

        let outer_size = available_outer.size().at_most(max_size);

        let inner_size = {
            let mut inner_size = outer_size;

            // Don't go so far that we shrink to zero.
            // In particular, if we put a `ScrollArea` inside of a `ScrollArea`, the inner
            // one shouldn't collapse into nothingness.
            // See https://github.com/emilk/egui/issues/1097
            for d in 0..2 {
                if has_bar[d] {
                    inner_size[d] = inner_size[d].max(min_scrolled_size[d]);
                }
            }
            inner_size
        };

        let inner_rect = Rect::from_min_size(available_outer.min, inner_size);

        let mut content_max_size = inner_size;

        if true {
            // Tell the inner Ui to *try* to fit the content without needing to scroll,
            // i.e. better to wrap text and shrink images than showing a horizontal scrollbar!
        } else {
            // Tell the inner Ui to use as much space as possible, we can scroll to see it!
            for d in 0..2 {
                if has_bar[d] {
                    content_max_size[d] = f32::INFINITY;
                }
            }
        }

        let content_max_rect = Rect::from_min_size(inner_rect.min - state.offset, content_max_size);
        let mut content_ui = ui.child_ui(content_max_rect, *ui.layout());
        let mut content_clip_rect = inner_rect.expand(ui.visuals().clip_rect_margin);
        content_clip_rect = content_clip_rect.intersect(ui.clip_rect());
        // Nice handling of forced resizing beyond the possible:
        for d in 0..2 {
            if !has_bar[d] {
                content_clip_rect.max[d] = ui.clip_rect().max[d];
            }
        }
        content_ui.set_clip_rect(content_clip_rect);

        let viewport = Rect::from_min_size(Pos2::ZERO + state.offset, inner_size);

        Prepared {
            id,
            state,
            has_bar,
            auto_shrink,
            current_bar_use: Vec2::default(),
            always_show_scroll,
            inner_rect,
            content_ui,
            viewport,
            scrolling_enabled,
            stick_to_end,
        }
    }

    /// Show the `ScrollArea`, and add the contents to the viewport.
    ///
    /// If the inner area can be very long, consider using [`Self::show_rows`] instead.
    pub fn show<R>(
        self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> ScrollAreaOutput<R> {
        self.show_viewport_dyn(ui, Box::new(|ui, _viewport| add_contents(ui)))
    }

    fn show_viewport_dyn<'c, R>(
        self,
        ui: &mut Ui,
        add_contents: Box<dyn FnOnce(&mut Ui, Rect) -> R + 'c>,
    ) -> ScrollAreaOutput<R> {
        let mut prepared = self.begin(ui);
        let id = prepared.id;
        let inner_rect = prepared.inner_rect;
        let inner = add_contents(&mut prepared.content_ui, prepared.viewport);
        let state = prepared.end(ui);
        ScrollAreaOutput {
            inner,
            id,
            state,
            inner_rect,
        }
    }
}

impl Prepared {
    fn end(self, ui: &mut Ui) -> State {
        let Prepared {
            id,
            mut state,
            inner_rect,
            has_bar,
            auto_shrink,
            mut current_bar_use,
            always_show_scroll,
            content_ui,
            viewport: _,
            scrolling_enabled,
            stick_to_end,
        } = self;

        let content_size = content_ui.min_size();

        let inner_rect = {
            // At this point this is the available size for the inner rect.
            let mut inner_size = inner_rect.size();

            for d in 0..2 {
                inner_size[d] = match (has_bar[d], auto_shrink[d]) {
                    (true, true) => inner_size[d].min(content_size[d]), // shrink scroll area if content is small
                    (true, false) => inner_size[d], // let scroll area be larger than content; fill with blank space
                    (false, true) => content_size[d], // Follow the content (expand/contract to fit it).
                    (false, false) => inner_size[d].max(content_size[d]), // Expand to fit content
                };
            }

            let mut inner_rect = Rect::from_min_size(inner_rect.min, inner_size);

            // The window that egui sits in can't be expanded by egui, so we need to respect it:
            for d in 0..2 {
                if !has_bar[d] {
                    // HACK for when we have a vertical-only scroll area in a top level panel,
                    // and that panel is not wide enough for the contents.
                    // This code ensures we still see the scroll bar!
                    let max = ui.input().screen_rect().max[d]
                        - current_bar_use[d]
                        - ui.spacing().item_spacing[d];
                    inner_rect.max[d] = inner_rect.max[d].at_most(max);
                    // TODO: maybe auto-enable horizontal/vertical scrolling if this limit is reached
                }
            }

            inner_rect
        };

        let outer_rect = Rect::from_min_size(inner_rect.min, inner_rect.size() + current_bar_use);

        let content_is_too_small = [
            content_size.x > inner_rect.width(),
            content_size.y > inner_rect.height(),
        ];

        if content_is_too_small[0] || content_is_too_small[1] {
            // Drag contents to scroll (for touch screens mostly):
            let sense = if self.scrolling_enabled {
                Sense::drag()
            } else {
                Sense::hover()
            };
            let content_response = ui.interact(inner_rect, id.with("area"), sense);

            if content_response.dragged() {
                for d in 0..2 {
                    if has_bar[d] {
                        state.offset[d] -= ui.input().pointer.delta()[d];
                        state.vel[d] = ui.input().pointer.velocity()[d];
                        state.scroll_stuck_to_end[d] = false;
                    } else {
                        state.vel[d] = 0.0;
                    }
                }
            } else {
                let stop_speed = 20.0; // Pixels per second.
                let friction_coeff = 1000.0; // Pixels per second squared.
                let dt = ui.input().unstable_dt;

                let friction = friction_coeff * dt;
                if friction > state.vel.length() || state.vel.length() < stop_speed {
                    state.vel = Vec2::ZERO;
                } else {
                    state.vel -= friction * state.vel.normalized();
                    // Offset has an inverted coordinate system compared to
                    // the velocity, so we subtract it instead of adding it
                    state.offset -= state.vel * dt;
                    ui.ctx().request_repaint();
                }
            }
        }

        let max_offset = content_size - inner_rect.size();
        

        let show_scroll_this_frame = [
            content_is_too_small[0] || always_show_scroll,
            content_is_too_small[1] || always_show_scroll,
        ];

        let max_scroll_bar_width = max_scroll_bar_width_with_margin(ui);

        // Avoid frame delay; start showing scroll bar right away:
        if show_scroll_this_frame[0] && current_bar_use.y <= 0.0 {
            current_bar_use.y = max_scroll_bar_width * ui.ctx().animate_bool(id.with("h"), true);
        }
        if show_scroll_this_frame[1] && current_bar_use.x <= 0.0 {
            current_bar_use.x = max_scroll_bar_width * ui.ctx().animate_bool(id.with("v"), true);
        }

        for d in 0..2 {
            let animation_t = current_bar_use[1 - d] / max_scroll_bar_width;

            if animation_t == 0.0 {
                continue;
            }

            // margin between contents and scroll bar
            let margin = animation_t * ui.spacing().item_spacing.x;
            let min_cross = inner_rect.max[1 - d] + margin; // left of vertical scroll (d == 1)
            let max_cross = outer_rect.max[1 - d]; // right of vertical scroll (d == 1)
            let min_main = inner_rect.min[d]; // top of vertical scroll (d == 1)
            let max_main = inner_rect.max[d]; // bottom of vertical scroll (d == 1)

            let outer_scroll_rect = if d == 0 {
                Rect::from_min_max(
                    pos2(inner_rect.left(), min_cross),
                    pos2(inner_rect.right(), max_cross),
                )
            } else {
                Rect::from_min_max(
                    pos2(min_cross, inner_rect.top()),
                    pos2(max_cross, inner_rect.bottom()),
                )
            };

            // maybe force increase in offset to keep scroll stuck to end position
            if stick_to_end[d] && state.scroll_stuck_to_end[d] {
                state.offset[d] = content_size[d] - inner_rect.size()[d];
            }

            let from_content =
                |content| remap_clamp(content, 0.0..=content_size[d], min_main..=max_main);

            let handle_rect = if d == 0 {
                Rect::from_min_max(
                    pos2(from_content(state.offset.x), min_cross),
                    pos2(from_content(state.offset.x + inner_rect.width()), max_cross),
                )
            } else {
                Rect::from_min_max(
                    pos2(min_cross, from_content(state.offset.y)),
                    pos2(
                        max_cross,
                        from_content(state.offset.y + inner_rect.height()),
                    ),
                )
            };

            let interact_id = id.with(d);
            let sense = if self.scrolling_enabled {
                Sense::click_and_drag()
            } else {
                Sense::hover()
            };
            let response = ui.interact(outer_scroll_rect, interact_id, sense);

            if let Some(pointer_pos) = response.interact_pointer_pos() {
                let scroll_start_offset_from_top_left = state.scroll_start_offset_from_top_left[d]
                    .get_or_insert_with(|| {
                        if handle_rect.contains(pointer_pos) {
                            pointer_pos[d] - handle_rect.min[d]
                        } else {
                            let handle_top_pos_at_bottom = max_main - handle_rect.size()[d];
                            // Calculate the new handle top position, centering the handle on the mouse.
                            let new_handle_top_pos = (pointer_pos[d] - handle_rect.size()[d] / 2.0)
                                .clamp(min_main, handle_top_pos_at_bottom);
                            pointer_pos[d] - new_handle_top_pos
                        }
                    });

                let new_handle_top = pointer_pos[d] - *scroll_start_offset_from_top_left;
                state.offset[d] = remap(new_handle_top, min_main..=max_main, 0.0..=content_size[d]);

                // some manual action taken, scroll not stuck
                state.scroll_stuck_to_end[d] = false;
            } else {
                state.scroll_start_offset_from_top_left[d] = None;
            }

            let unbounded_offset = state.offset[d];
            state.offset[d] = state.offset[d].max(0.0);
            state.offset[d] = state.offset[d].min(max_offset[d]);

            if state.offset[d] != unbounded_offset {
                state.vel[d] = 0.0;
            }

            if ui.is_rect_visible(outer_scroll_rect) {
                // Avoid frame-delay by calculating a new handle rect:
                let mut handle_rect = if d == 0 {
                    Rect::from_min_max(
                        pos2(from_content(state.offset.x), min_cross),
                        pos2(from_content(state.offset.x + inner_rect.width()), max_cross),
                    )
                } else {
                    Rect::from_min_max(
                        pos2(min_cross, from_content(state.offset.y)),
                        pos2(
                            max_cross,
                            from_content(state.offset.y + inner_rect.height()),
                        ),
                    )
                };
                let min_handle_size = ui.spacing().scroll_bar_width;
                if handle_rect.size()[d] < min_handle_size {
                    handle_rect = Rect::from_center_size(
                        handle_rect.center(),
                        if d == 0 {
                            vec2(min_handle_size, handle_rect.size().y)
                        } else {
                            vec2(handle_rect.size().x, min_handle_size)
                        },
                    );
                }

                let visuals = if scrolling_enabled {
                    ui.style().interact(&response)
                } else {
                    &ui.style().visuals.widgets.inactive
                };

                ui.painter().add(epaint::Shape::rect_filled(
                    outer_scroll_rect,
                    visuals.rounding,
                    ui.visuals().extreme_bg_color,
                ));

                ui.painter().add(epaint::Shape::rect_filled(
                    handle_rect,
                    visuals.rounding,
                    visuals.bg_fill,
                ));
            }
        }


        if show_scroll_this_frame != state.show_scroll {
            ui.ctx().request_repaint();
        }

        let available_offset = content_size - inner_rect.size();
        state.offset = state.offset.min(available_offset);
        state.offset = state.offset.max(Vec2::ZERO);

        // Is scroll handle at end of content, or is there no scrollbar
        // yet (not enough content), but sticking is requested? If so, enter sticky mode.
        // Only has an effect if stick_to_end is enabled but we save in
        // state anyway so that entering sticky mode at an arbitrary time
        // has appropriate effect.
        state.scroll_stuck_to_end = [
            (state.offset[0] == available_offset[0])
                || (self.stick_to_end[0] && available_offset[0] < 0.),
            (state.offset[1] == available_offset[1])
                || (self.stick_to_end[1] && available_offset[1] < 0.),
        ];

        state.show_scroll = show_scroll_this_frame;

        state.store(ui.ctx(), id);

        state
    }
}

/// Width of a vertical scrollbar, or height of a horizontal scroll bar
fn max_scroll_bar_width_with_margin(ui: &Ui) -> f32 {
    ui.spacing().item_spacing.x + ui.spacing().scroll_bar_width
}
