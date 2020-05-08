mod core;

use crate::core::{window_manager::WindowManager, x::Display};

#[tokio::main]
async fn main() -> Result<(), String> {
    let display = Display::open()?;

    let mut wm = WindowManager::new(display);
    println!("[RWM]");

    let windows = wm.scan()?;
    println!("- Attached to {} windows.", windows);
    println!("- Running...");
    wm.run()?;

    Ok(())
}
