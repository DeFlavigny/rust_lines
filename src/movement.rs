use bevy::window::CursorMoved;

use crate::prelude::*;

pub fn pawn_movement_system(
    mouse_input: Res<Input<MouseButton>>,
    mut query: Query<(&HexPosition, &GameObj, &Owner)>,
) {
}

pub fn cursor_tracking_system(
    mut e_cursor_moved: EventReader<CursorMoved>,
    mut query: Query<(&mut Cursor, &mut Transform)>,
) {
    let (mut cursor, mut transform) = query.single_mut().expect("Did not find exactly one cursor");
    let event = e_cursor_moved.iter().next();
    match event {
        Some(e) => {
            cursor.has_moved = true;
            transform.translation = Vec3::new(e.position.x, e.position.y, 0.0);
        }
        None => cursor.has_moved = false,
    }
}
