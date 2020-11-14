use crate::components::{MyWorld, PlayerBox};
use crate::entities::spawn_ai_ship;

use bevy::prelude::*;

pub fn player_control_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    //time: Res<Time>,
    my_world: ResMut<MyWorld>,
    keyboard_input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut query: Query<(&mut PlayerBox, &mut Transform)>,
) {
    let mut directionx: f32;
    let mut directiony: f32;

    let (mut player_box, mut transform) = query.get_mut(my_world.player.unwrap()).unwrap();

    if keyboard_input.pressed(KeyCode::LShift) {
        player_box.speed = 2.0;
    } else {
        player_box.speed = 0.5;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        player_box.angle -= 0.05 * player_box.speed;
        directionx = ((player_box.angle as f64).cos() * 1.0) as f32 * 300.0;
        directiony = ((player_box.angle as f64).sin() * 1.0) as f32 * 300.0;

        let translation = &mut transform.translation;
        *translation.x_mut() = directionx;
        *translation.y_mut() = directiony;

        transform.rotation = Quat::from_rotation_z(player_box.angle - 1.570796);
    }

    if keyboard_input.pressed(KeyCode::Right) {
        player_box.angle += 0.05 * player_box.speed;
        directionx = ((player_box.angle as f64).cos() * 1.0) as f32 * 300.0;
        directiony = ((player_box.angle as f64).sin() * 1.0) as f32 * 300.0;

        let translation = &mut transform.translation;
        *translation.x_mut() = directionx;
        *translation.y_mut() = directiony;

        transform.rotation = Quat::from_rotation_z(player_box.angle - 1.570796);
    }

    if keyboard_input.just_pressed(KeyCode::Z) {
        spawn_ai_ship(&mut commands, &mut materials, &asset_server, 1);
    }

    if keyboard_input.pressed(KeyCode::Space) {
        if player_box.weapons.can_fire() {
            let spawn_x = ((player_box.angle as f64).cos() * 1.0) as f32 * 280.0;
            let spawn_y = ((player_box.angle as f64).sin() * 1.0) as f32 * 280.0;
            let angle = player_box.angle - 3.141593;
            player_box.weapons.fire(
                spawn_x,
                spawn_y,
                angle,
                commands,
                materials,
                asset_server,
                audio,
            )
        } else {
        }
    }

    player_box.weapons.cooldown();

    if keyboard_input.pressed(KeyCode::Up) {
        player_box.speed += 0.01;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        player_box.speed -= 0.01;
    }

    let x = transform.translation.x();
    let y = transform.translation.y();

    player_box.col_shape.update(x, y, 40.0, 40.0);
}
