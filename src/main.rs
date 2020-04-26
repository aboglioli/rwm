mod core;
use crate::core::window_manager::WindowManager;

fn main() -> Result<(), String> {
    let mut wm = WindowManager::new()?;
    wm.scan()?;
    wm.run()?;
    wm.cleanup();

    Ok(())
}
