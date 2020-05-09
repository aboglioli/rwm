pub use crate::core::window;

pub type NodeID = u64;

pub trait Node {
    fn id(&self) -> NodeID;
    fn is(&self, id: NodeID) -> bool;

    fn set_position(&mut self, x: i32, y: i32);
    fn set_size(&mut self, width: u32, height: u32);

    fn focus(&mut self);
    fn unfocus(&mut self);

    fn mark(&mut self);
    fn unmark(&mut self);

    fn map(&self);
    fn reparent(&self, parent: window::WindowID);
}
