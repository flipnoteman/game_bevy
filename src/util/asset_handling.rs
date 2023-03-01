use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

/// States to track the completion of asset loading to prevent the 'Err on None()' error
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AssetStates {
    AssetLoading,
    Next,
}

/// Uses bevy_asset_loaders asset macros to load and store Texture Atlases for the player
#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    // With the new crate I loaded, you can call this series of macros and it will create and add a
    // TextureAtlas with the given asset into the TextureAtlas resources
    #[asset(texture_atlas(tile_size_x = 48.0, tile_size_y = 48.0, columns = 10, rows = 1))]
    #[asset(path = "Player/Player Sword Idle/Player Sword Idle 48x48.png")]
    pub(crate) idle: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 48.0, tile_size_y = 48.0, columns = 8, rows = 1))]
    #[asset(path = "Player/Player Sword Run/Player Sword Run 48x48.png")]
    pub(crate) run: Handle<TextureAtlas>,
}