use bevy::prelude::*;

#[derive(Component)]
pub struct Controllable;

pub fn player_move(
    key: Res<Input<KeyCode>>,
    mut player_query: Query<(&Controllable, &mut Transform)>
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