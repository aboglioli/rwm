use crate::core::node;
use std::iter::Iterator;

pub trait Layout {
    fn apply(&self, windows: &mut dyn Iterator<Item = &mut Box<dyn node::Node>>);
}

pub struct ColumnLayout(pub u32, pub u32);
impl Layout for ColumnLayout {
    fn apply(&self, windows: &mut dyn Iterator<Item = &mut Box<dyn node::Node>>) {
        if let (_, Some(mut len)) = windows.size_hint() {
            len = if len > 0 { len } else { 1 };
            let (w, h) = (self.0 / len as u32, self.1);
            for (i, win) in windows.enumerate() {
                let i = i as i32;
                win.set_size(w, h);
                win.set_position(i * w as i32, 0);
            }
        }
    }
}

pub struct RowLayout(pub u32, pub u32);

impl Layout for RowLayout {
    fn apply(&self, windows: &mut dyn Iterator<Item = &mut Box<dyn node::Node>>) {
        if let (_, Some(mut len)) = windows.size_hint() {
            len = if len > 0 { len } else { 1 };
            let (w, h) = (self.0, self.1 / len as u32);
            for (i, win) in windows.enumerate() {
                let i = i as i32;
                win.set_size(w, h);
                win.set_position(0, i * h as i32);
            }
        }
    }
}
