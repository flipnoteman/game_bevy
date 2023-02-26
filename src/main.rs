use bevy::prelude::*;
use crate::entities::enemy::EnemyBundle;
use crate::entities::player::PlayerBundle;
mod entities;
mod util;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(entities::camera::spawn_camera)
        .add_startup_system(PlayerBundle::spawn_player)
        .add_startup_system(EnemyBundle::spawn_enemy)
        .add_system(util::animation::animate_sprites)
        .add_system(entities::player_movement::player_move)
        .run();
}
