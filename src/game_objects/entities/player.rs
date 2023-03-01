use bevy::prelude::*;
use crate::util::animation::AnimationTimer;
use crate::util::asset_handling::PlayerAssets;

/// Unique identifier for Querying for the player attributes
#[derive(Component)]
pub struct Player;

/// Defines the current players state, is updated upon events
#[derive(Component)]
pub enum PlayerState {
    Running,
    Walking,
    Idle,
    Attacking
}

impl Player {
    /// Spawn our Player Bundle
    pub fn spawn_player(
        mut commands: Commands,
        player_assets: Res<PlayerAssets>,
    ) {
        // Spawn our texture atlas for animation
        commands.spawn(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(0., 150., 0.),
                ..default()
            },
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: player_assets.idle.clone(),
            ..default()
        })
            .insert(AnimationTimer(Timer::from_seconds(
                0.1,
                TimerMode::Repeating
            )))
            .insert(Player)
            .insert(PlayerState::Idle);
    }

    /// React to events by changing the Player Animation
    pub fn change_animation(
        player_assets: Res<PlayerAssets>,
        key: Res<Input<KeyCode>>,
        mut query: Query<(&Player, &mut Handle<TextureAtlas>, &mut PlayerState)>
    ) {
        for (_, mut texture_handle, mut state) in &mut query {
            if key.pressed(KeyCode::D) || key.pressed(KeyCode::A) {
                *state = PlayerState::Running;
                *texture_handle = player_assets.run.clone();
            } else {
                *state = PlayerState::Idle;
                *texture_handle = player_assets.idle.clone();
            }
        }
    }
}