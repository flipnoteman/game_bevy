use std::ops::Deref;
use bevy::prelude::*;
use crate::game_objects::entities::player::Player;

/// Provide player movement when pressing left or right
/// ToDo: Implement other movement types (Probably after physics has been implemented)
pub fn player_move(
    key: Res<Input<KeyCode>>,
    mut player_query: Query<(&Player, &mut Transform)>
)
{
    for (_, mut transform, mut anim_state) in &mut player_query {
        *anim_state = AnimationState::IDLE;

        if key.pressed(KeyCode::D) {
            transform.translation += Vec3::new(3.0, 0.0, 0.0);
            transform.rotation = Quat::default();
            *anim_state = AnimationState::MOVING;
        }
        if key.pressed(KeyCode::A) {
            transform.translation += Vec3::new(-3.0, 0.0, 0.0);
            transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
            *anim_state = AnimationState::MOVING;
        }
    }
}