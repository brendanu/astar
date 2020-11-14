use crate::components::*;
use crate::entities::*;

use bevy::prelude::*;

pub fn game_setup_system(
    mut commands: Commands,
    mut my_world: ResMut<MyWorld>,
    mut camera_res: ResMut<Camera>,
    materials: ResMut<Assets<ColorMaterial>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut ship_sprites_handles: ResMut<ShipsSpriteHandles>,
) {
    //sounds
    spawn_player(&mut commands, materials, &asset_server);
    let player_entity = commands.current_entity().unwrap();
    my_world.player = Some(player_entity);
    spawn_hive_mind(&mut commands);
    let hive_mind = commands.current_entity().unwrap();
    my_world.hive_mind = Some(hive_mind);

    let camera = Camera2dComponents::default();
    commands.spawn(camera);

    camera_res.camera = Some(commands.current_entity()).unwrap();
    //camera_res.position = (0.0, 0.0);
    //camera_res.level = 0;
    //camera_res.time_remaining = 0.0;

    //

    ship_sprites_handles.handles = asset_server.load_folder("textures/ships").unwrap();

    let _song1 = "sounds/music/Juhani Junkala [Retro Game Music Pack] Level 1.ogg";
    let _title = "sounds/music/Juhani Junkala [Retro Game Music Pack] Title Screen.ogg";
    audio.play(asset_server.load(_song1));

    let _start1 = "sounds/speech/round_1.ogg";
    audio.play(asset_server.load(_start1));
}

/*

use bevy::{asset::LoadState, sprite::TextureAtlasBuilder};

pub fn _load_atlas(
    commands: Commands,
    ship_sprite_handles: ResMut<ShipsSpriteHandles>,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    my_world: ResMut<MyWorld>,
) {
    if ship_sprite_handles.atlas_loaded {
        return;
    }

    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    if let LoadState::Loaded = asset_server
        .get_group_load_state(ship_sprite_handles.handles.iter().map(|handle| handle.id))
    {
        for handle in ship_sprite_handles.handles.iter() {
            let texture = textures.get(handle).unwrap();
            texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), &texture);
        }

        let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();

        /*spawn_ship_builder_ship(
            commands,
            ship_sprite_handles,
            asset_server,
            texture_atlases,
            my_world,
            texture_atlas,
            150.0,
            150.0,
        );*/
    }
}*/

pub fn _spawn_ship_builder_ship(
    mut commands: Commands,
    mut ship_sprite_handles: ResMut<ShipsSpriteHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut my_world: ResMut<MyWorld>,
    texture_atlas: TextureAtlas,
    x: f32,
    y: f32,
) {
    let arm_handle = asset_server.get_handle("textures/ships/Arm1.png");
    let arm_index = texture_atlas.get_texture_index(&arm_handle).unwrap();

    let body_handle = asset_server.get_handle("textures/ships/cockpit1Green.png");
    let body_index = texture_atlas.get_texture_index(&body_handle).unwrap();

    let cockpit_handle = asset_server.get_handle("textures/ships/cockpit3Green.png");
    let cockpit_index = texture_atlas.get_texture_index(&cockpit_handle).unwrap();

    let wing_handle = asset_server.get_handle("textures/ships/wing2Green.png");
    let wing_index = texture_atlas.get_texture_index(&wing_handle).unwrap();

    let atlas_handle = texture_atlases.add(texture_atlas);

    /*arm = shipParts.findRegion("shipparts/Arm1");
    body = shipParts.findRegion("shipparts/cockpit1" + bodyColour);
    cockpit = shipParts.findRegion("shipparts/cockpit3" + cockpitColour);
    wing = shipParts.findRegion("shipparts/wing2" + wingColour);*/

    let mut transform = Transform::from_translation(Vec3::new(x, y, 0.0));
    transform.rotate(Quat::from_rotation_z(0.0));

    let _parent = commands
        .spawn(SpriteComponents {
            transform,
            ..Default::default()
        })
        .with(ShipBuilder)
        // With that entity as a parent, run a lambda that spawns its children
        .with_children(|parent| {
            // parent is a ChildBuilder, which has a similar API to Commands
            parent.spawn(SpriteSheetComponents {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec3::splat(1.0),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(arm_index as u32),
                texture_atlas: texture_atlases.get_handle(&atlas_handle),
                ..Default::default()
            });
        })
        .with_children(|parent| {
            // parent is a ChildBuilder, which has a similar API to Commands
            parent.spawn(SpriteSheetComponents {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 10.0),
                    scale: Vec3::splat(1.0),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(body_index as u32),
                texture_atlas: texture_atlases.get_handle(&atlas_handle),
                ..Default::default()
            });
        })
        .with_children(|parent| {
            // parent is a ChildBuilder, which has a similar API to Commands
            parent.spawn(SpriteSheetComponents {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 20.0),
                    scale: Vec3::splat(1.0),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(cockpit_index as u32),
                texture_atlas: texture_atlases.get_handle(&atlas_handle),
                ..Default::default()
            });
        })
        .with_children(|parent| {
            // parent is a ChildBuilder, which has a similar API to Commands
            parent.spawn(SpriteSheetComponents {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 15.0),
                    scale: Vec3::splat(1.0),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(wing_index as u32),
                texture_atlas: texture_atlases.get_handle(&atlas_handle),
                ..Default::default()
            });
        })
        .with_children(|parent| {
            // parent is a ChildBuilder, which has a similar API to Commands
            parent.spawn(SpriteSheetComponents {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 15.0),
                    scale: Vec3::new(-1.0, 1.0, 1.0),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(wing_index as u32),
                texture_atlas: texture_atlases.get_handle(&atlas_handle),
                ..Default::default()
            });
        });
    // Store parent entity for next sections
    //.current_entity()
    //.unwrap();
    my_world.ship = Some(commands.current_entity()).unwrap();

    println!("added parent");

    /*
    .spawn(SpriteSheetComponents {
        transform: Transform {
            translation: Vec3::new(130.0, 130.0, 15.0),
            scale: Vec3::splat(1.0),
            ..Default::default()
        },
        sprite: TextureAtlasSprite::new(wing_index as u32),
        texture_atlas: texture_atlases.get_handle(&atlas_handle),
        ..Default::default()
    })
    .spawn(SpriteSheetComponents {
        transform: Transform {
            translation: Vec3::new(70.0, 130.0, 15.0),
            scale: Vec3::new(-1.0, 1.0, 1.0),
            ..Default::default()
        },
        sprite: TextureAtlasSprite::new(wing_index as u32),
        texture_atlas: texture_atlases.get_handle(&atlas_handle),
        ..Default::default()
    });*/

    ship_sprite_handles.atlas_loaded = true;
}
