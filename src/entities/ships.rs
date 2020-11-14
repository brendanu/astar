use crate::components::{HiveMind, OppoShip, StructuralSystem};
use crate::geom::*;

use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub fn spawn_hive_mind(commands: &mut Commands) {
    commands.with(HiveMind::new());
}

pub fn spawn_simple(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
) {
    let tow_texture = asset_server.load("textures/sampleShip2.png");

    let mut rng = thread_rng();
    let angle = (rng.gen_range(0.0, 360.0) as f64).to_radians() as f32;

    let mut transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
    transform.rotate(Quat::from_rotation_z(angle + 1.570796));

    commands
        .spawn(SpriteComponents {
            material: materials.add(tow_texture.into()),
            transform,
            sprite: Sprite::new(Vec2::new(5.0, 5.0)),
            ..Default::default()
        })
        .with(OppoShip {
            angle,
            life_span: 450.0,
            speed: 0.5,
            col_shape: Rectangle::new(0.0, 0.0, 5.0, 5.0),
            level: 1,
            system: StructuralSystem::new(),
        });
}

pub fn spawn_rotator(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
) {
    let tow_texture = asset_server.load("textures/sampleShip3.png");

    let mut rng = thread_rng();
    let angle = (rng.gen_range(0.0, 360.0) as f64).to_radians() as f32;

    let mut transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
    transform.rotate(Quat::from_rotation_z(angle + 1.570796));

    commands
        //.spawn(Camera2dComponents::default())
        //.spawn(UiCameraComponents::default())
        .spawn(SpriteComponents {
            //material: materials.add(Color::rgb(0.4, 0.7, 0.4).into()),
            material: materials.add(tow_texture.into()),
            transform,
            sprite: Sprite::new(Vec2::new(5.0, 5.0)),
            ..Default::default()
        })
        .with(OppoShip {
            angle,
            life_span: 450.0,
            speed: 0.8,
            col_shape: Rectangle::new(0.0, 0.0, 5.0, 5.0),
            level: 2,
            system: StructuralSystem::new(),
        });
}

pub fn spawn_ai_ship(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
    level: u32,
) {
    match level {
        1 => {
            spawn_simple(commands, materials, asset_server);
        }
        2 => {
            spawn_rotator(commands, materials, asset_server);
        }
        _ => println!("Nothing to spawn"),
    }
}
