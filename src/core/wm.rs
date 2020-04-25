use crate::core::{event, x};

pub struct WindowManager {
    display: x::Display,
}

impl WindowManager {
    pub fn new() -> Result<WindowManager, String> {
        Ok(WindowManager {
            display: x::Display::open()?,
        })
    }

    pub fn run(&self) -> Result<(), String> {
        self.display.hook()?;
        self.display.grab_button(
            1,
            0,
            self.display.root,
            1,
            event::BUTTON_PRESS_MASK | event::BUTTON_RELEASE_MASK | event::POINTER_MOTION_MASK,
            event::GRAB_MODE_ASYNC,
            event::GRAB_MODE_ASYNC,
            0,
            0,
        );

        loop {
            match self.display.next_event() {
                event::Event::ButtonPress(button) => {
                    println!("{:?}", button);
                }
                event::Event::Motion(button, motion) => {
                    println!("{:?}", button);
                    println!("{:?}", motion);
                }
                _ => (),
            }
        }
    }
}
