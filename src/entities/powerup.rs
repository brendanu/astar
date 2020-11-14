use crate::components::{PowerUp, POWERUPTYPE};
use crate::geom::*;
use bevy::prelude::*;

pub fn spawn_powerup(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
    angle: f32,
    x: f32,
    y: f32,
    power_type: POWERUPTYPE,
) {
    let tow_texture = match power_type {
        POWERUPTYPE::Health => asset_server.load("textures/bullets/laserGreen.png"),
        POWERUPTYPE::WeaponIncreaseBullets => asset_server.load("textures/bullets/laserRed.png"),
        POWERUPTYPE::WeaponMultiFire => asset_server.load("textures/bullets/laserBlue.png"),
    };

    commands
        .spawn(SpriteComponents {
            //material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            material: materials.add(tow_texture.into()),
            //material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
            sprite: Sprite::new(Vec2::new(20.0, 20.0)),

            ..Default::default()
        })
        .with(PowerUp {
            powerup_type: power_type,
            time_remaining: 100.0,
            angle,
            col_shape: Rectangle::new(x, y, 20.0, 20.0),
        });
}
