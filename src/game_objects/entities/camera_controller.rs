use bevy::prelude::{Camera2dBundle, Commands};
use bevy::prelude::*;
use crate::game_objects::entities::player::Player;

/// Makes the camera Follow the Player
/// ToDo: Adding Linear Interpolation to the Camera movement so the player can look ahead
pub fn move_camera(
    mut camera_query: Query<(&Camera, &mut Transform)>,
    player_query: Query<(&Player, &Transform), Without<Camera>>
) {
    let (_, player_loc) = player_query.single();
    for (_, mut camera_loc) in &mut camera_query{
        camera_loc.translation = player_loc.translation;
    }
}

/// Provide tracking for Camera attributes
#[derive(Component)]
pub struct Camera;

/// Spawn main camera
pub fn spawn_camera(
    mut commands: Commands,
){
    let mut camera = Camera2dBundle::default();
    commands.spawn((camera, Camera));
}