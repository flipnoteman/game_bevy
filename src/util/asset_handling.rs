use bevy::asset::{Assets, AssetServer, Handle};
use bevy::math::Vec2;
use bevy::prelude::{Image, ResMut};
use bevy::sprite::TextureAtlas;

pub fn load_texture(
    asset_server: ResMut<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    sheet: String
    ) -> Handle<TextureAtlas> {
    let handle: Handle<Image> = asset_server.load(sheet);
    let atlas = TextureAtlas::from_grid(handle, Vec2::new(48.0, 48.0), 8, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(atlas);
    texture_atlas_handle
}