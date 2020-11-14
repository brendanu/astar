use crate::components::Camera;

use bevy::prelude::*;

use rand::{thread_rng, Rng};

pub fn camera_system(
    time: Res<Time>,
    mut camera_res: ResMut<Camera>,
    mut query: Query<&mut Transform>,
) {
    let mut camera = query.get_mut(camera_res.camera.unwrap()).unwrap();

    let mut rng = thread_rng();

    //let x = my_world.cam_pos.0;
    //let y = my_world.cam_pos.1;

    if camera_res.time_remaining > 0.0 {
        let rx;

        match camera_res.level {
            1 => rx = 50.0,
            2 => rx = 125.0,
            3 => rx = 250.0,
            _ => rx = 0.0,
        };

        let x = (rng.gen_range(-rx, rx) as f64).to_radians() as f32;
        let y = (rng.gen_range(-rx, rx) as f64).to_radians() as f32;

        *camera.translation.x_mut() = x; // my_world.cam_pos.0;
        *camera.translation.y_mut() = y; // my_world.cam_pos.1;

        camera_res.time_remaining -= time.delta_seconds;
    }
}
