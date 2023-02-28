use bevy::ecs::bundle::Bundle;
use bevy::sprite::{SpriteSheetBundle, TextureAtlas};
use std::default::Default;
use benimator::FrameRate;
use util::animation::Animation;
use bevy::asset::{Assets, AssetServer};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;
use bevy::time::{Timer, TimerMode};
use bevy::asset::{AssetLoader, BoxedFuture, Error, LoadContext, LoadedAsset};
use player_movement::Controllable;

use crate::game_objects::game_object::Name;
use crate::game_objects::entities::entity::Health;
use crate::game_objects::entities::player_movement;
use crate::util;
use crate::util::animation::AnimationState;


/// Main Player Bundle
#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: Name,
    pub health: Health,

    #[bundle]
    pub model: SpriteSheetBundle,
}

#[derive(Component, Eq, PartialEq)]
pub enum PlayerState {
    RUNNING,
    IDLE,
}

impl PlayerBundle {
    /// Spawn our Player Bundle
    pub fn spawn_player( mut commands: Commands,
                            asset_server: ResMut<AssetServer>,
                            texture_atlas_server: ResMut<Assets<TextureAtlas>>) {

        let animation = Animation(benimator::Animation::from_indices(
           0..=7,
            FrameRate::from_fps(12.0),
        ));

        commands.spawn((
            PlayerBundle{
            name: Name(("Sam").to_string()),
            health: Health(10),
            model: SpriteSheetBundle {
                texture_atlas: util::asset_handling::load_texture(
                    asset_server,
                    texture_atlas_server,
                    "Player Sword Idle/Player Sword Idle 48x48.png".to_string(),
                    Vec2::new(48.0, 48.0),
                    8,
                    1,
                ),
                transform: Transform::from_scale(Vec3::splat(2.0)),
                ..default()
            }
        },
            Controllable,
        ))
            .insert(animation)
            .insert(AnimationState::default())
            .insert(PlayerState::IDLE);
    }
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            name: Name("Player".to_string()),
            health: Health(10),

            model: SpriteSheetBundle{
                sprite: Default::default(),
                texture_atlas: Default::default(),
                transform: Default::default(),
                global_transform: Default::default(),
                visibility: Default::default(),
                computed_visibility: Default::default(),
            },
        }
    }
}