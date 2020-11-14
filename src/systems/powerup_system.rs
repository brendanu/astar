use crate::components::{MyWorld, PlayerBox, PowerUp, BULLETTYPE, POWERUPTYPE};
use bevy::prelude::*;

pub fn powerup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    my_world: Res<MyWorld>,
    time: Res<Time>,
    audio: Res<Audio>,
    mut query_bullets: Query<(Entity, &mut PowerUp, &mut Transform)>,
    mut query_player: Query<(Entity, &mut PlayerBox)>,
) {
    let (_player_entity, mut player_box) = query_player.get_mut(my_world.player.unwrap()).unwrap();

    for (powerup_entity, mut powerup, mut transform) in query_bullets.iter_mut() {
        if powerup.col_shape.overlaps(player_box.col_shape) {
            println!("powerup");

            match powerup.powerup_type {
                POWERUPTYPE::Health => {
                    player_box.system.add_healthpack(10.0);
                }
                POWERUPTYPE::WeaponIncreaseBullets => {
                    player_box.weapons.max_bullets += 10;
                }
                POWERUPTYPE::WeaponMultiFire => {
                    player_box.weapons.change(BULLETTYPE::Spread);
                }
            }

            let sfx: bevy::prelude::Handle<bevy::prelude::AudioSource> =
                asset_server.load("sounds/sfx/sfx_sound_neutral7.ogg");
            audio.play(sfx);
            commands.despawn(powerup_entity);
        }

        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        let modx = rng.gen_range(-1.0, 1.0);
        let mody = rng.gen_range(-1.0, 1.0);

        *transform.translation.x_mut() += ((powerup.angle as f64).cos() * 1.0) as f32 * 2.0 + modx;
        *transform.translation.y_mut() += ((powerup.angle as f64).sin() * 1.0) as f32 * 2.0 + mody;

        powerup.col_shape.update(
            transform.translation.x().clone(),
            transform.translation.y().clone(),
            (transform.scale[0].clone()) * 1.0 as f32,
            (transform.scale[1].clone()) * 1.0 as f32,
        );

        //powerup.angle
        powerup.time_remaining -= time.delta_seconds;
        if powerup.time_remaining <= 0.0 {
            commands.despawn(powerup_entity);
        }
    }
}
