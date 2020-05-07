mod core;
use crate::core::{window_manager::WindowManager, x::Display};

fn main() -> Result<(), String> {
    let display = Display::open()?;

    let mut wm = WindowManager::new(&display);
    println!("[RWM]");

    // Scan existing windows
    let windows = wm.scan()?;
    println!("- Attached to {} windows.", windows);
    println!("- Running...");

    // Wait for events
    wm.run()?;

    Ok(())
}
