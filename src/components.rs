use crate::prelude::*;

pub struct HexPosition {
    pub hex_id: i32,
}

pub struct GameObj;

pub struct Hoverable;
pub struct Hovered;
pub struct Draggable;
pub struct Dragged;
pub struct Dropped;

pub struct Cursor {
    pub has_moved: bool,
}

pub enum Owner {
    Player(i32),
    Game,
}

pub enum MoveMode {
    Line,
}
