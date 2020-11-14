use crate::components::{Bullet, Camera, MyWorld, OppoShip, PlayerBox, OWNERTYPE, POWERUPTYPE};
use crate::entities::{spawn_particles, spawn_powerup};
use bevy::prelude::*;

pub fn collision_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut my_world: ResMut<MyWorld>,
    mut camera_res: ResMut<Camera>,
    audio: Res<Audio>,
    mut query_ships: Query<(Entity, &mut OppoShip, &Transform, &Handle<ColorMaterial>)>,
    mut query_bullets: Query<(Entity, &mut Bullet)>,
    mut query_player: Query<(Entity, &mut PlayerBox)>,
) {
    let (_player_entity, mut player_box) = query_player.get_mut(my_world.player.unwrap()).unwrap();

    for (entity_bullet, bullet) in query_bullets.iter_mut() {
        if bullet.owner_type == OWNERTYPE::Player {
            for (entity_ship, mut ship, transform, material_handle) in query_ships.iter_mut() {
                //println!("bullet: ");
                //dbg!(bullet.col_shape);
                //println!("ship: ");
                //dbg!(ship.col_shape);

                if bullet.col_shape.overlaps(ship.col_shape) {
                    //println!("overlap");

                    let x = transform.translation.x();
                    let y = transform.translation.y();
                    let scale = transform.scale.x();

                    let angle = ship.angle;

                    let colour = materials.get(material_handle).unwrap().clone().color;
                    spawn_particles(&mut commands, &mut materials, x, y, scale, colour);

                    my_world.points += ship.level * 10;
                    println!("Points: {}", my_world.points);
                    if ship.system.hit(bullet.damage_amount) {
                        commands.despawn(entity_ship);
                        let sfx: bevy::prelude::Handle<bevy::prelude::AudioSource> =
                            asset_server.load("sounds/sfx/sfx_exp_shortest_hard1.ogg");
                        audio.play(sfx);
                    }
                    commands.despawn(entity_bullet);

                    use rand::{thread_rng, Rng};
                    let mut rng = thread_rng();
                    let chance = rng.gen_range(0, 100);

                    if chance >= 40 {
                        let what_type = rng.gen_range(0, 100);

                        match what_type {
                            0..=60 => spawn_powerup(
                                &mut commands,
                                &mut materials,
                                &asset_server,
                                angle,
                                x,
                                y,
                                POWERUPTYPE::Health,
                            ),
                            70..=90 => spawn_powerup(
                                &mut commands,
                                &mut materials,
                                &asset_server,
                                angle,
                                x,
                                y,
                                POWERUPTYPE::WeaponIncreaseBullets,
                            ),
                            91..=100 => spawn_powerup(
                                &mut commands,
                                &mut materials,
                                &asset_server,
                                angle,
                                x,
                                y,
                                POWERUPTYPE::WeaponMultiFire,
                            ),
                            _ => {}
                        }
                    }
                }
            }
        } else if bullet.owner_type == OWNERTYPE::Enemy {
            if bullet.col_shape.overlaps(player_box.col_shape) {
                if player_box.system.hit(bullet.damage_amount) {
                    commands.despawn(entity_bullet);
                }

                if player_box.system.health_warning() {
                    let sfx: bevy::prelude::Handle<bevy::prelude::AudioSource> =
                        asset_server.load("sounds/sfx/sfx_sound_neutral1.ogg");
                    audio.play(sfx);
                }

                let sfx: bevy::prelude::Handle<bevy::prelude::AudioSource> =
                    asset_server.load("sounds/sfx/sfx_exp_cluster3.ogg");
                audio.play(sfx);

                camera_res.level = 3;
                camera_res.time_remaining = 0.5;
            }
        }

        //if ship.life_span <= 0.0 {
        //    commands.despawn(entity);
        // }
    }
}
