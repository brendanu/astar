use crate::components::{Bullet, BULLETTYPE, OWNERTYPE};
use crate::geom::*;
use bevy::prelude::*;

pub fn spawn_bullet(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
    x: f32,
    y: f32,
    angle: f32,
    owner_type: OWNERTYPE,
    bullet_type: BULLETTYPE,
) -> (u32, f32) {
    match bullet_type {
        BULLETTYPE::Regular => {
            spawn_bullet_regular(commands, materials, asset_server, x, y, angle, owner_type);
            return (1, 0.1);
        }
        BULLETTYPE::Spread => {
            spawn_bullet_regular(
                commands,
                materials,
                asset_server,
                x,
                y,
                angle - 0.236,
                owner_type,
            );
            spawn_bullet_regular(commands, materials, asset_server, x, y, angle, owner_type);
            spawn_bullet_regular(
                commands,
                materials,
                asset_server,
                x,
                y,
                angle + 0.236,
                owner_type,
            );
            return (3, 0.1);
        }
    }
}
pub fn spawn_bullet_regular(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
    x: f32,
    y: f32,
    angle: f32,
    owner_type: OWNERTYPE,
) {
    let start_size = if owner_type == OWNERTYPE::Player {
        10.0
    } else {
        5.0
    };

    let tow_texture = if owner_type == OWNERTYPE::Player {
        asset_server.load("textures/bullets/laserGreen.png")
    } else {
        asset_server.load("textures/bullets/laserRed.png")
    };

    commands
        .spawn(SpriteComponents {
            //material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            material: materials.add(tow_texture.into()),
            //material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
            sprite: Sprite::new(Vec2::new(start_size, start_size)),

            ..Default::default()
        })
        .with(Bullet {
            angle,
            life_span: 100.0,
            speed: 3.0,
            col_shape: Rectangle::new(x, y, start_size, start_size),
            owner_type,
            damage_amount: 5.0,
        });
}
