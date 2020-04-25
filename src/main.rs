mod core;
use crate::core::wm::WindowManager;

fn main() -> Result<(), String> {
    let window_manager = WindowManager::new()?;
    window_manager.run()?;
    Ok(())
}
