use crate::prelude::*;

#[derive(Component)]
pub struct HexPosition {
    pub hex_id: i32,
}
#[derive(Component)]
pub struct GameObj;
#[derive(Component)]
pub struct Hoverable;
#[derive(Component)]
pub struct Hovered;
#[derive(Component)]
pub struct Draggable;
#[derive(Component)]
pub struct Dragged;
#[derive(Component)]
pub struct Dropped;
#[derive(Component)]
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
