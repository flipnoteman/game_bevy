use bevy::ecs::bundle::Bundle;
use bevy::sprite::{SpriteSheetBundle, TextureAtlas};
use std::default::Default;
use bevy::asset::{Assets, AssetServer};
use bevy::math::Vec3;
use bevy::prelude::{Commands, Res, ResMut, Transform};
use bevy::time::{Timer, TimerMode};
use bevy::utils::default;
use player_movement::Controllable;
use crate::entities::entity::Name;
use crate::entities::entity::Health;
use crate::entities::player_movement;
use crate::util;


/// Main Player Bundle
#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: Name,
    pub health: Health,

    #[bundle]
    pub model: SpriteSheetBundle,
}

impl PlayerBundle {
    /// Spawn our Player Bundle
    pub fn spawn_player( mut commands: Commands,
                            asset_server: ResMut<AssetServer>,
                            texture_atlas_server: ResMut<Assets<TextureAtlas>>) {
        commands.spawn((
            PlayerBundle{
            name: Name(("Sam").to_string()),
            health: Health(10),
            model: SpriteSheetBundle {
                texture_atlas: util::asset_handling::load_texture(
                    asset_server,
                    texture_atlas_server,
                    "Player Sword Run/Player Sword Run 48x48.png".to_string()),
                transform: bevy::prelude::Transform::from_scale(Vec3::splat(2.0)),
                ..default()
            }
        },
            util::animation::AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Controllable
        ));
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