use crate::components::{PlayerBox, StructuralSystem, Weapons};
use crate::geom::Rectangle;

use bevy::prelude::*;

pub fn spawn_player(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
) {
    let tow_texture = asset_server.load("textures/sampleShip1.png");

    let x = ((0.0 as f64).cos() * 1.0) as f32 * 300.0;
    let y = ((0.0 as f64).sin() * 1.0) as f32 * 300.0;
    let mut transform = Transform::from_translation(Vec3::new(x, y, 0.0));
    transform.rotate(Quat::from_rotation_z(0.0 - 1.570796));

    commands
        .spawn(SpriteComponents {
            //material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            material: materials.add(tow_texture.into()),
            //material: materials.add(Color::rgb(0.2, 0.5, 0.9).into()),
            transform,
            sprite: Sprite::new(Vec2::new(40.0, 40.0)),

            ..Default::default()
        })
        .with(PlayerBox {
            angle: 0.0,
            speed: 0.5,
            boost: 1.0,
            col_shape: Rectangle::new(x, y, 40.0, 40.0),
            system: StructuralSystem::new(),
            weapons: Weapons::new(),
        });
}
