use bevy::prelude::*;
mod game_objects;
use crate::game_objects::entities::*;
use crate::util::asset_handling::*;
use bevy::asset::LoadState;

mod util;

fn main() {
    App::new()
        .init_resource::<SpriteHandles>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_state::<AppState>(Default::default())
        .add_startup_system(load_textures.in_schedule(OnEnter(AppState::Setup)))
        .add_startup_system(camera_controller::spawn_camera)
        .add_startup_system(player::PlayerBundle::spawn_player)
        //.add_startup_system(enemy::EnemyBundle::spawn_enemy)
        .add_system(camera_controller::move_camera)
        .add_system(util::animation::animate_sprites)
        .add_system(player_movement::player_move)
        .run();
}
