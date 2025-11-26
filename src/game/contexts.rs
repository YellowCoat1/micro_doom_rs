/// Makes this crate a lot more portable by abstracting away different contexts.

use crate::game::drawing::Drawer;

pub struct GraphicsContext<'a, T: Drawer> {
    pub width: u32,
    pub height: u32,
    pub drawer: &'a mut T,
}

pub struct KeysDown {
    pub up: bool,
    pub down: bool,
    pub left: bool, 
    pub right: bool,
    pub w: bool,
    pub s: bool,
}
