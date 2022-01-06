mod components;
mod spawner;
mod map;

mod prelude {
    pub use bevy::prelude::*;
    pub use crate::map::*;
    pub use hex2d::*;

    //Map config
    pub const MAP_HEX_RADIUS: usize = 5;
    pub const SCREENPOS_START_X: i64 = -250;
    pub const SCREENPOS_START_Y: i64 = -250;

    pub const HEX_SPRITE_PATH: &str = "sprites/white_hex.png";
}

use bevy::{
    core::FixedTimestep,
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};

use map::Map;
use prelude::HEX_SPRITE_PATH;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup.system())
        .run();
}

enum Collider {
    Solid,
    Scorable,
}

fn setup(
    mut commands: Commands,
    materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let texture_handle = asset_server.load(HEX_SPRITE_PATH);

    let map = Map::new();
    map.render(commands, materials, texture_handle);

}

fn hue_from_coords(x: u32, y: u32) -> u32 {
    (350 + y * 3 + x * 12) % 360
}

struct Tint {
    hue: u32,
}
