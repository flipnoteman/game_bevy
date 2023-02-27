use bevy::ecs::bundle::Bundle;
use bevy::sprite::{SpriteSheetBundle, TextureAtlas};
use std::default::Default;
use bevy::asset::{Assets, AssetServer};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;
use bevy::time::{Timer, TimerMode};
use bevy::utils::default;
use player_movement::Controllable;
use crate::game_objects::game_object::Name;
use crate::game_objects::entities::entity::{Health};
use crate::game_objects::entities::player_movement;
use crate::util;
use crate::util::animation::{AnimationState};
use crate::util::asset_handling::load_player_textures;
use crate::util::player_animations::load_player_animations;


/// Main Player Bundle
#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: Name,
    pub health: Health,

    #[bundle]
    pub active_animation: SpriteSheetBundle,
}

impl PlayerBundle {
    /// Spawn our Player Bundle
    pub fn spawn_player(mut commands: Commands,
        mut asset_server: ResMut<AssetServer>,
        mut texture_atlas_server: ResMut<Assets<TextureAtlas>>,
    ) {
        let asset_handle: Handle<Image> = asset_server.load("Player Sword Idle/Player Sword Idle 48x48.png");
        let texture_handle = texture_atlas_server.get_handle(asset_handle);
        commands.spawn((
            PlayerBundle{
            name: Name(("Sam").to_string()),
            health: Health(10),
            active_animation: SpriteSheetBundle {
                texture_atlas: texture_handle,
                transform: Transform::from_scale(Vec3::splat(2.0)),
                ..default()
            },
            ..default()
        },
            util::animation::AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Controllable,
            AnimationState::IDLE,
        ));
    }
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            name: Name("Player".to_string()),
            health: Health(10),
            active_animation: SpriteSheetBundle{
                ..default()
            }
        }
    }
}