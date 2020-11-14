use crate::components::{OppoShip, BULLETTYPE, OWNERTYPE};
use crate::entities::spawn_bullet;

use bevy::prelude::*;

pub fn ship_system(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(Entity, &mut OppoShip, &mut Transform)>,
) {
    for (entity, mut ship, mut transform) in query.iter_mut() {
        *transform.translation.x_mut() += ((ship.angle as f64).cos() * 1.0) as f32 * ship.speed;
        *transform.translation.y_mut() += ((ship.angle as f64).sin() * 1.0) as f32 * ship.speed;

        transform.scale[0] += 0.025;
        transform.scale[1] += 0.025;

        ship.speed += 0.001;

        ship.col_shape.update(
            transform.translation.x().clone(),
            transform.translation.y().clone(),
            (transform.scale[0].clone()) * 3.0 as f32,
            (transform.scale[1].clone()) * 3.0 as f32,
        );

        match ship.level {
            1 => {
                use rand::{thread_rng, Rng};
                let mut rng = thread_rng();
                let chance = rng.gen_range(0, 100);

                if chance >= 99 {
                    let x = transform.translation.x().clone();
                    let y = transform.translation.y().clone();
                    let angle = ship.angle;
                    spawn_bullet(
                        &mut commands,
                        &mut materials,
                        &mut asset_server,
                        x,
                        y,
                        angle,
                        OWNERTYPE::Enemy,
                        BULLETTYPE::Regular,
                    );
                }
            }
            2 => {
                use rand::{thread_rng, Rng};
                let mut rng = thread_rng();
                let chance = rng.gen_range(0, 100);

                if chance >= 98 {
                    let x = transform.translation.x().clone();
                    let y = transform.translation.y().clone();
                    let angle = ship.angle;
                    spawn_bullet(
                        &mut commands,
                        &mut materials,
                        &mut asset_server,
                        x,
                        y,
                        angle,
                        OWNERTYPE::Enemy,
                        BULLETTYPE::Regular,
                    );
                }
                ship.angle += 0.005;
                transform.rotation = Quat::from_rotation_z(ship.angle + 1.570796);
            }
            _ => {}
        }

        ship.life_span -= 1.0;
        if ship.life_span <= 0.0 {
            commands.despawn(entity);
        }
    }
}
