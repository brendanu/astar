use crate::components::{HiveMind, MyWorld};
use crate::entities::spawn_ai_ship;

use bevy::prelude::*;

pub fn hive_mind_system(
    mut commands: Commands,
    time: Res<Time>,
    mut my_world: ResMut<MyWorld>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    audio: Res<Audio>,
    mut query: Query<&mut HiveMind>,
) {
    let mut hive_mind = query.get_mut(my_world.hive_mind.unwrap()).unwrap();

    my_world.progression += time.delta_seconds;

    if my_world.progression >= 10.0 {
        my_world.progression = 0.0;
        my_world.level += 1;

        /*let sfx_to_play;

        match my_world.level {
            2 => {
                sfx_to_play = "sounds/speech/round_2.ogg";
                let sfx: bevy::prelude::Handle<bevy::prelude::AudioSource> =
                    asset_server.load(sfx_to_play);
                audio.play(sfx);
            }
            //3 => sfx_to_play = "sounds/speech/round_3.ogg",
            //4 => sfx_to_play = "sounds/speech/round_4.ogg",
            //5 => sfx_to_play = "sounds/speech/round_5.ogg",
            _ => (), //sfx_to_play = "sounds/speech/you_win.ogg",
        }*/
    }

    if my_world.progression < 9.0 {
        if hive_mind.spawn_at <= 0.0 {
            use rand::{thread_rng, Rng};
            let mut rng = thread_rng();
            let ship_level = rng.gen_range(1, 100);

            match ship_level {
                0..=80 => {
                    spawn_ai_ship(&mut commands, &mut materials, &asset_server, 1);
                }
                81..=100 => {
                    spawn_ai_ship(&mut commands, &mut materials, &asset_server, 2);
                }
                _ => {}
            }

            hive_mind.spawn_at = 0.75;
        } else {
            hive_mind.spawn_at -= time.delta_seconds
        }
    }
}
