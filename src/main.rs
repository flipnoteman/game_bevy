use bevy::prelude::*;
mod game_objects;
use crate::game_objects::entities::*;
use crate::game_objects::entities::camera_controller::spawn_camera;
use crate::util::asset_handling::load_player_textures;

mod util;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin{
            watch_for_changes: true,
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(camera_controller::move_camera)
        .add_system(util::animation::animate_sprites)
        .add_system(player_movement::player_move)
        .run();
}

fn setup(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    assets: Res<Assets<Image>>
) {
    asset_server.load_folder("Player Sword Idle").expect("Oh No!!!");
    //asset_server.load_folder("Player Sword Run").expect("Oh No!!!");
    load_player_textures(&asset_server, &mut texture_atlases);
    // for (id, _) in assets.iter() {
    //     println!("\n{:?}\n", id);
    // }
    spawn_camera;
    player::PlayerBundle::spawn_player(commands, asset_server, texture_atlases);
}

