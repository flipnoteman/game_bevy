use bevy::asset::{Assets, AssetServer, Handle};
use bevy::math::Vec2;
use bevy::prelude::{Image, ResMut};
use bevy::sprite::TextureAtlas;

pub fn load_texture(
    asset_server: ResMut<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    sheet: String,
    tile_size: Vec2,
    columns: usize,
    rows: usize,
    ) -> Handle<TextureAtlas> {
    let handle: Handle<Image> = asset_server.load(sheet);
    let atlas = TextureAtlas::from_grid(handle, tile_size, columns, rows, None, None);
    let texture_atlas_handle = texture_atlases.add(atlas);
    texture_atlas_handle
}