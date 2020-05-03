use crate::core::window;
use std::iter::Iterator;

pub trait Layout {
    fn apply<'a>(&self, windows: Box<dyn Iterator<Item = &mut window::Window> + 'a>);
}

pub struct ColumnLayout(pub u32, pub u32);
impl Layout for ColumnLayout {
    fn apply<'a>(&self, windows: Box<dyn Iterator<Item = &mut window::Window> + 'a>) {
        if let (_, Some(len)) = windows.size_hint() {
            let mut i = 0;
            let (w, h) = (self.0 / len as u32, self.1);
            for win in windows {
                win.set_size(w, h);
                win.set_position(i * w as i32, 0);
                i += 1;
            }
        }
    }
}

pub struct RowLayout(pub u32, pub u32);

impl Layout for RowLayout {
    fn apply<'a>(&self, windows: Box<dyn Iterator<Item = &mut window::Window> + 'a>) {
        if let (_, Some(len)) = windows.size_hint() {
            let mut i = 0;
            let (w, h) = (self.0, self.1 / len as u32);
            for win in windows {
                win.set_size(w, h);
                win.set_position(0, i * h as i32);
                i += 1;
            }
        }
    }
}
