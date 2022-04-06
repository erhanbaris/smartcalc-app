/*
 * smartcalc-app v1.0.8
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use std::io;
fn main() -> io::Result<()> {
    #[cfg(all(windows_subsystem = "windows", not(target_arch = "wasm32")))] {
        use winres::WindowsResource;
        WindowsResource::new()
            .set_icon("./assets/smartcalc.ico")
            .compile()?;
    }
    Ok(())
}