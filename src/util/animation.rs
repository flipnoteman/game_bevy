use bevy::prelude::*;
use crate::game_objects::entities::player::*;

/// Provides entities with specific framerate specifier
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

/// Animates all sprites in the world, we handle sprite changing separately
pub fn animate_sprite_system(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite)>,
) {
    for (mut timer, mut sprite) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index = (sprite.index + 1) % 8; // modulo ensures it doesn't exceed 10 frames
        }
    }
}