mod components;
mod map;
mod movement;

use std::env;

use bevy_prototype_lyon::prelude::*;
use map::Map;
use movement::*;

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
    sprite::collide_aabb::{collide, Collision},
};
use prelude::Cursor;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 8 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(DefaultLinesPlugin)
        .run();
}

pub struct DefaultLinesPlugin;

impl Plugin for DefaultLinesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    //map_setup(&mut commands);
    //game_setup(&mut commands);
}

fn map_setup(commands: &mut Commands) {
    let map = Map::new();
    map.render(commands);
}

fn game_setup(commands: &mut Commands) {

    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(50.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn_bundle(GeometryBuilder::build_as(
        &shapes::Circle {
            radius:5.0,
            center:Vec2::new(0.0,0.0),
        },
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::BEIGE),
            outline_mode: StrokeMode::new(Color::BLACK, 2.0),
        },
        Transform::default(),
    )).insert(Cursor {has_moved:false});
}


