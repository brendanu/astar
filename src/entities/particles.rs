use crate::components::Particle;
use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub fn spawn_particles(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    x: f32,
    y: f32,
    scale: f32,
    colour: Color,
) {
    let mut rng = thread_rng();

    for _i in 0..10 {
        let angle = (rng.gen_range(0.0, 360.0) as f64).to_radians() as f32;

        let mut transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
        transform.rotate(Quat::from_rotation_z(angle));

        commands
            .spawn(SpriteComponents {
                //material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                material: materials.add(colour.into()),
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                sprite: Sprite::new(Vec2::new(1.0 * scale, 1.0 * scale)),

                ..Default::default()
            })
            .with(Particle {
                angle,
                lifetime: 4.0,
                speed: 5.0,
            });
    }
}
