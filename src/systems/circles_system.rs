use crate::components::{CircleController, Circles};
use crate::entities::spawn_circle;

use rand::{thread_rng, Rng};

use bevy::prelude::*;

pub fn circle_spawner_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut circle_controller: ResMut<CircleController>,
) {
    circle_controller.spawn_at -= time.delta_seconds;

    let mut angle: f64 = 0.0 + circle_controller.offset as f64;

    if circle_controller.spawn_at <= 0.0 {
        let mut rng = thread_rng();
        let col_mix = (
            rng.gen_range(0.0, 1.0),
            rng.gen_range(0.0, 1.0),
            rng.gen_range(0.0, 1.0),
        );

        //println!("Spawning");
        for _i in 0..40 {
            spawn_circle(
                &mut commands,
                &mut materials,
                0.0,
                0.0,
                angle.to_radians() as f32,
                col_mix,
            );
            angle += 360.0 / 40.0;
        }
        circle_controller.spawn_at = 1.2;
        circle_controller.offset += 5.0;
        if circle_controller.offset > 360.0 {
            circle_controller.offset = 0.0;
        }
    }
}

pub fn circles_proc_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Circles, &mut Transform)>,
) {
    for (entity, mut circle, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;

        *translation.x_mut() += ((circle.angle as f64).cos() * 1.0) as f32 * circle.speed;
        *translation.y_mut() += ((circle.angle as f64).sin() * 1.0) as f32 * circle.speed;

        circle.life_span -= 1.0;
        circle.speed += 0.01;
        let scale = &mut transform.scale;
        scale[0] += 0.1;
        scale[1] += 0.1;
        if circle.life_span <= 0.0 {
            commands.despawn(entity);
        }
    }
}
