use bevy::prelude::{Camera2dBundle, Commands};
use bevy::prelude::*;
use crate::game_objects::entities::player_movement::Controllable;

pub fn move_camera(
    mut camera_query: Query<(&Camera, &mut Transform)>,
    mut player_query: Query<(&Controllable, &Transform)>
) {
    let (_, player_loc) = player_query.single();
    for (_, mut camera_loc) in &mut camera_query{
        camera_loc.translation = player_loc.translation;
    }
}

#[derive(Component)]
pub struct Camera;

pub fn spawn_camera(
    mut commands: Commands,
){
    commands.spawn((Camera2dBundle::default(), Camera));
}