use crate::components::Circles;

use bevy::prelude::*;

pub fn spawn_circle(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    x: f32,
    y: f32,
    angle: f32,
    col_mix: (f32, f32, f32),
) {
    let mut transform = Transform::from_translation(Vec3::new(x, y, 0.0));
    transform.rotate(Quat::from_rotation_z(angle - 1.570796));

    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(col_mix.0, col_mix.1, col_mix.2).into()),
            transform: transform,
            sprite: Sprite::new(Vec2::new(0.5, 3.0)),

            ..Default::default()
        })
        .with(Circles {
            angle,
            life_span: 250.0,
            speed: 2.0,
        });
}
