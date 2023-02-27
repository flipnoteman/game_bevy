use bevy::asset::{Assets, AssetServer, Handle};
use bevy::math::Vec2;
use bevy::prelude::{Image, Res, ResMut};
use bevy::sprite::TextureAtlas;

pub fn load_player_textures(
    asset_server: &ResMut<AssetServer>,
    mut texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ){
    let idle_handle: Handle<Image> = asset_server.load("Player Sword Idle/Player Sword Idle 48x48.png");
    let atlas = TextureAtlas::from_grid(idle_handle, Vec2::new(48.0, 48.0), 8, 1, None, None);
    texture_atlases.add(atlas);

    let run_handle: Handle<Image> = asset_server.load("Player Sword Run/Player Sword Run 48x48.png");
    let atlas = TextureAtlas::from_grid(run_handle, Vec2::new(48.0, 48.0), 8, 1, None, None);
    texture_atlases.add(atlas);
}