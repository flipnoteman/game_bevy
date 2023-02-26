use bevy::prelude::*;
mod game_objects;
use crate::game_objects::entities::*;
mod util;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(camera_controller::spawn_camera)
        .add_startup_system(player::PlayerBundle::spawn_player)
        .add_startup_system(enemy::EnemyBundle::spawn_enemy)
        .add_system(camera_controller::move_camera)
        .add_system(util::animation::animate_sprites)
        .add_system(player_movement::player_move)
        .run();
}
