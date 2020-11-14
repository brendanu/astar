use crate::components::{Bullet, MyWorld, PlayerBox, OWNERTYPE};

use bevy::prelude::*;

pub fn bullet_system(
    mut commands: Commands,
    my_world: ResMut<MyWorld>,
    mut query: Query<(Entity, &mut Bullet, &mut Transform)>,
    mut p_query: Query<&mut PlayerBox>,
) {
    for (entity, mut bullet, mut transform) in query.iter_mut() {
        *transform.translation.x_mut() += ((bullet.angle as f64).cos() * 1.0) as f32 * bullet.speed;
        *transform.translation.y_mut() += ((bullet.angle as f64).sin() * 1.0) as f32 * bullet.speed;

        bullet.life_span -= 1.0;

        if bullet.owner_type == OWNERTYPE::Player {
            bullet.speed -= 0.01;
            transform.scale[0] -= 0.01;
            transform.scale[1] -= 0.01;
        } else {
            bullet.speed += 0.01;
            transform.scale[0] += 0.01;
            transform.scale[1] += 0.01;
        }

        bullet.col_shape.update(
            transform.translation.x(),
            transform.translation.y(),
            transform.scale[0] as f32 * 10.0,
            transform.scale[1] as f32 * 10.0,
        );

        if bullet.life_span <= 0.0 {
            commands.despawn(entity);

            if bullet.owner_type == OWNERTYPE::Player {
                let mut player = p_query.get_mut(my_world.player.unwrap()).unwrap();
                player.weapons.current_bullets -= 1;
            }
        }
    }
}
