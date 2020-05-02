mod core;
use crate::core::{window_manager::WindowManager, x};

fn main() -> Result<(), String> {
    let display = x::Display::open()?;

    let mut wm = WindowManager::new(&display);
    wm.scan()?;
    wm.run()?;

    Ok(())
}
