use crate::core::node;

pub struct Container {
    windows: Vec<dyn Window>;
}

impl Container {
    fn new() -> Self {
        Container {
            windows: Vec::new(),
        }
    }
}
