mod components;
mod map;
mod movement;

use bevy_prototype_lyon::prelude::*;
use components::*;
use map::Map;
use movement::*;
use prelude::MoveMode;

mod prelude {
    pub use crate::components::*;
    pub use crate::map::*;
    pub use bevy::prelude::*;
    pub use bevy_prototype_lyon::prelude::*;
    pub use hex2d::*;

    //Map config
    pub const MAP_HEX_RADIUS: usize = 5;
}

use bevy::{
    core::FixedTimestep,
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 8 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(DefaultLinesPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    map_setup(&mut commands);
    game_setup(&mut commands);
}

fn map_setup(commands: &mut Commands) {
    let map = Map::new();
    map.render(commands);
}

fn game_setup(commands: &mut Commands) {
    
}

pub struct DefaultLinesPlugin;

impl Plugin for DefaultLinesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system_to_stage(CoreStage::PreUpdate, cursor_tracking_system.system());
    }
}
