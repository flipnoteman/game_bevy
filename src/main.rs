use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use crate::game_objects::entities::camera_controller::{move_camera, spawn_camera};

mod game_objects;
use crate::util::animation::*;
use crate::util::asset_handling::*;
use crate::game_objects::entities::player::*;
use crate::game_objects::entities::player_movement::player_move;

use bevy_ecs_ldtk::prelude::*;

mod util;

fn main() {
    App::new()
        // Initialize our states, Bevy knows when to transition between states based on the Collection we gave it below
        .add_loading_state(
            // Creates a loading state that will switch over the Next state when <PlayerAssets> has been completely loaded
            LoadingState::new(AssetStates::AssetLoading)
                .continue_to_state(AssetStates::Next) // will continue upon completion of below collection
                .with_collection::<PlayerAssets>() // Collection it's checking
        )
        .add_state(AssetStates::AssetLoading) // To define the initial state
        .insert_resource(Msaa { samples: 1 }) // Adds anti aliasing
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(LdtkPlugin)
        // Here we add system sets so we can make sure we only call the sprites if the assets have been completely loaded
        // "AssetStates::Next" is the state AFTER the sprite sheets are done loading, so systems that are called "on_enter"
        // will only call once, right after the assets are successfully loaded, and systems with "on_update" are called once every
        // game update cycle, just like add_startup_system and add_system but with proper error checking
        .add_system_set(SystemSet::on_enter(AssetStates::Next).with_system(spawn_camera))
        .add_system_set(SystemSet::on_enter(AssetStates::Next).with_system(Player::spawn_player))
        .add_system_set(SystemSet::on_enter(AssetStates::Next).with_system(ldtk_setup))
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<LevelBundle>("LevelIdentifier")
        .add_system_set(SystemSet::on_update(AssetStates::Next).with_system(player_move))
        .add_system_set(SystemSet::on_update(AssetStates::Next).with_system(animate_sprite_system))
        .add_system_set(SystemSet::on_update(AssetStates::Next).with_system(Player::change_animation))
        .add_system_set(SystemSet::on_update(AssetStates::Next).with_system(move_camera))
        .run();
}

fn ldtk_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(LdtkWorldBundle {
       ldtk_handle: asset_server.load("Level/level_one.ldtk"),
        ..default()
    });
}

#[derive(Default, Component)]
struct ComponentA;

#[derive(Default, Component)]
struct ComponentB;

#[derive(Bundle, LdtkEntity)]
pub struct LevelBundle {
    a: ComponentA,
    b: ComponentB,
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle
}


