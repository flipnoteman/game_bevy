use bevy::prelude::{Res, Timer, Query, Time, TextureAtlas, TextureAtlasSprite, Handle, Assets, Component, Deref, DerefMut};
use bevy::sprite::SpriteSheetBundle;
use crate::game_objects::entities::entity::SpriteSheets;
use crate::game_objects::entities::player::{PlayerBundle};
use bevy::prelude::*;
use crate::game_objects::entities::player_movement::Controllable;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component,PartialEq,Eq)]
pub enum AnimationState{
    MOVING,
    IDLE,
    JUMPING,
    PRONE,
    DEAD,
    ATTACKING,
}

pub fn animate_sprites(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
        &AnimationState,
    )>,
) {
    for (mut timer, mut sprite, mut texture_atlas_handle, animation_state) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if *animation_state == AnimationState::IDLE {
                let asset: Handle<Image> = asset_server.load("Player Sword Idle/Player Sword Idle 48x48.png");
                let text = texture_atlases.get_handle(asset);
                let texture_atlas = texture_atlases.get(&text).unwrap();
                sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            }
        }
    }
}

pub fn sprite_animation_system (){

}
