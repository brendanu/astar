use crate::components::Particle;

use bevy::prelude::*;

pub fn particle_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Particle, &mut Transform)>,
) {
    for (entity, mut particle, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;

        *translation.x_mut() += ((particle.angle as f64).cos() * 1.0) as f32 * particle.speed;
        *translation.y_mut() += ((particle.angle as f64).sin() * 1.0) as f32 * particle.speed;

        *transform.scale.x_mut() *= 0.95;
        *transform.scale.y_mut() *= 0.95;
        *transform.scale.z_mut() *= 0.95;

        particle.lifetime -= time.delta_seconds;

        if particle.lifetime <= 0.0 {
            commands.despawn(entity);
        }
    }
}
