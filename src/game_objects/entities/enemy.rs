use bevy::asset::{Assets, AssetServer};
use bevy::ecs::bundle::Bundle;
use bevy::math::Vec3;
use bevy::prelude::{Commands, default, ResMut};
use bevy::sprite::{SpriteSheetBundle, TextureAtlas};
use bevy::time::{Timer, TimerMode};
use crate::entities::entity;
use crate::entities::entity::{Health, Name};
use crate::util;

#[derive(Bundle)]
pub struct EnemyBundle {
    name: entity::Name,
    health: entity::Health,

    #[bundle]
    model: SpriteSheetBundle,
}

impl EnemyBundle {
    pub fn spawn_enemy( mut commands: Commands,
                         asset_server: ResMut<AssetServer>,
                         texture_atlas_server: ResMut<Assets<TextureAtlas>>) {
        commands.spawn((
            EnemyBundle {
                name: Name(("Sam").to_string()),
                health: Health(10),
                model: SpriteSheetBundle {
                    texture_atlas: util::asset_handling::load_texture(
                        asset_server,
                        texture_atlas_server,
                        "Player Death/Player Death 64x64.png".to_string()),
                    transform: bevy::prelude::Transform::from_scale(Vec3::splat(2.0)),
                    ..default()
                }
            },
            util::animation::AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ));
    }
}

impl Default for EnemyBundle {
    fn default() -> Self {
        EnemyBundle {
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