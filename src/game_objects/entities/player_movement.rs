use bevy::prelude::*;
use crate::game_objects::entities::player::Player;

/// Provide player movement when pressing left or right
/// ToDo: Implement other movement types (Probably after physics has been implemented)
pub fn player_move(
    key: Res<Input<KeyCode>>,
    mut player_query: Query<(&Player, &mut Transform)>
)
{
    if key.pressed(KeyCode::D) {
        for (_, mut transform) in &mut player_query {
            transform.translation += Vec3::new(3.0, 0.0, 0.0);
            transform.rotation = Quat::default();
        }
    }
    if key.pressed(KeyCode::A) {
        for (_, mut transform) in &mut player_query {
            transform.translation += Vec3::new(-3.0, 0.0, 0.0);
            transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
        }
    }
}