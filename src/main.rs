use bevy::{prelude::*, render::pass::ClearColor};

mod components;
mod entities;
mod geom;
mod systems;

use components::{Camera, CircleController, MyWorld, ShipsSpriteHandles};
use systems::{
    bullet_system, camera_system, circle_spawner_system, circles_proc_system, collision_system,
    game_setup_system, hive_mind_system, particle_system, player_control_system, powerup_system,
    ship_system,
};

fn main() {
    App::build()
        .init_resource::<ShipsSpriteHandles>()
        .add_plugins(DefaultPlugins)
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_resource(MyWorld::new())
        .add_resource(Camera::new())
        .add_resource(CircleController::new())
        .add_startup_system(game_setup_system.system())
        //.add_system(load_atlas.system())
        .add_system(player_control_system.system())
        .add_system(bullet_system.system())
        .add_system(ship_system.system())
        .add_system(hive_mind_system.system())
        .add_system(circle_spawner_system.system())
        .add_system(circles_proc_system.system())
        .add_system(collision_system.system())
        .add_system(particle_system.system())
        .add_system(camera_system.system())
        .add_system(powerup_system.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

//https://github.com/jamadazi/bevy-cheatsheet
