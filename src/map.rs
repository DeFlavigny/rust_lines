use crate::prelude::*;
use bevy::prelude::Commands;
use std::collections::HashMap;

pub struct Map {
    pub hexes: HashMap<i32, Hex>,
}

pub struct Hex {
    pub c: Coordinate,
}

impl Map {
    pub fn new() -> Self {
        let mut hexes = HashMap::new();

        let center_coord = Coordinate::new(0,0);
        let c = center_coord.clone();
        let center_tile = Hex{c};
        hexes.insert(coord_to_id(&center_tile.c), center_tile);

        for r in 0..MAP_HEX_RADIUS {
            for c in center_coord.ring_iter(r as i32, Spin::CCW(XY)) {
                
                let id = coord_to_id(&c);
                let tile = Hex{c};
                hexes.insert(coord_to_id(&tile.c), tile);
            }
        }

        Self { hexes }
    }

    pub fn render(&self, mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, texture_handle: Handle<Texture>) {
        let hue = 200;

        // Cubic coordinates calculations
        let inner_angle: f64 = 30.0;
        let size = 50.0;
        let height = 2.0 * size;
        let width = size * 2.0 * (inner_angle * 2.0).to_radians().sin();

        for (_index, hex) in &self.hexes {
            let pixel = hex.c.to_pixel(Spacing::PointyTop(size));
            let transform = Transform::from_xyz(pixel.0 as f32, pixel.1 as f32, 0.0);

            // Spawn a hex
            commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(ColorMaterial {
                    color: Color::hsla(hue as f32, 0.8, 0.5, 1.0),
                    texture: Some(texture_handle.clone()),
                }),
                transform: transform,
                sprite: Sprite {
                    size: Vec2::new(width as f32, height as f32),
                    resize_mode: SpriteResizeMode::Manual,
                    ..Default::default()
                },
                ..Default::default()
            });
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
