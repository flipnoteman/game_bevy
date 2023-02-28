use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use crate::util::animation::AnimationTimer;

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

/// Uses bevy_asset_loaders asset macros to load and store Texture Atlases for the player
/// ***May move this to asset_handling***
#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    // With the new crate I loaded, you can call this series of macros and it will create and add a
    // TextureAtlas with the given asset into the TextureAtlas resources
    #[asset(texture_atlas(tile_size_x = 48.0, tile_size_y = 48.0, columns = 10, rows = 1))]
    #[asset(path = "Player/Player Sword Idle/Player Sword Idle 48x48.png")]
    idle: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 48.0, tile_size_y = 48.0, columns = 8, rows = 1))]
    #[asset(path = "Player/Player Sword Run/Player Sword Run 48x48.png")]
    run: Handle<TextureAtlas>,
}

impl Player {
    /// Spawn our Player Bundle
    pub fn spawn_player(
        mut commands: Commands,
        player_assets: Res<PlayerAssets>,
        texture_atlases: Res<Assets<TextureAtlas>>
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