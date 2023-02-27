use std::ops::Deref;
use bevy::prelude::*;
use crate::util::animation::AnimationState;

#[derive(Component)]
pub struct Controllable;

pub fn player_move(
    key: Res<Input<KeyCode>>,
    mut player_query: Query<(&Controllable, &mut Transform, &mut AnimationState)>
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