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