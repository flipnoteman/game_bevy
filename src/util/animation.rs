use bevy::prelude::{Res, Timer, Query, Time, TextureAtlas, TextureAtlasSprite, Handle, Assets, Component, Deref, DerefMut};

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub enum AnimationState{
    MOVING,
    IDLE,
    JUMPING,
    PRONE,
    DEAD,
    ATTACKING
}

pub fn animate_sprites(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &AnimationState,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle, animation_state) in &mut query {
        if animation_state == AnimationState::MOVING {
            timer.tick(time.delta());
            if timer.just_finished() {
                let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            }
        } else {

        }
    }
}