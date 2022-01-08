use crate::prelude::*;
use bevy::prelude::Commands;
use std::collections::HashMap;

const OFFSET_FROM_CENTER_X: f64 = -250.0;
const OFFSET_FROM_CENTER_Y: f64 = -0.0;

pub struct Map {
    pub hexes: HashMap<i32, Hex>,
}

pub struct Hex {
    pub c: Coordinate,
}

impl Map {
    pub fn new() -> Self {
        let mut hexes = HashMap::new();

        let center_coord = Coordinate::new(0, 0);
        let c = center_coord.clone();
        let center_tile = Hex { c };
        hexes.insert(coord_to_id(&center_tile.c), center_tile);

        for r in 0..MAP_HEX_RADIUS {
            for c in center_coord.ring_iter(r as i32, Spin::CCW(XY)) {
                let tile = Hex { c };
                hexes.insert(coord_to_id(&tile.c), tile);
            }
        }

        Self { hexes }
    }

    pub fn render(
        &self,
        commands: &mut Commands,
    ) {
        // Length of one hexagon edge
        let size = 50.0;

        // Hexagon shape config
        let shape = shapes::RegularPolygon {
            sides: 6,
            feature: shapes::RegularPolygonFeature::Radius(50.0),
            ..shapes::RegularPolygon::default()
        };

        // Spawn each hexagon
        for (_index, hex) in &self.hexes {
            // Create the screen position for the hex
            let pixel = hex.c.to_pixel(Spacing::PointyTop(size));
            let mut transform = Transform::from_xyz(
                (pixel.0 + OFFSET_FROM_CENTER_X) as f32,
                (pixel.1 + OFFSET_FROM_CENTER_Y) as f32,
                0.0,
            );
            transform.rotate(Quat::from_rotation_z(30.0_f32.to_radians()));

            // Render the hex
            commands.spawn_bundle(GeometryBuilder::build_as(
                &shape,
                ShapeColors::outlined(Color::BEIGE, Color::BLACK),
                DrawMode::Outlined {
                    fill_options: FillOptions::default(),
                    outline_options: StrokeOptions::default().with_line_width(2.0),
                },
                transform,
            ));
        }
    }
}

pub fn coord_to_id(c: &Coordinate) -> i32 {
    let range = (MAP_HEX_RADIUS * 2 + 1) as i32;
    c.x + (c.y * range)
}

pub fn id_to_coord(id: i32) -> Coordinate {
    let range = (MAP_HEX_RADIUS * 2 + 1) as i32;
    let x = id % range;
    let y = id / range;
    Coordinate::new(x, y)
}

pub fn nb_tiles_from_radius(radius: u32) -> u32 {
    let mut nb = 1;
    for i in 1..radius {
        nb += i * 6;
    }
    nb
}
