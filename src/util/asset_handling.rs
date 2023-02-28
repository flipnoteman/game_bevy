use bevy::asset::{Asset, Assets, AssetServer, Handle, LoadState};
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::ecs::prelude::*;
use bevy::render::camera::RenderTarget::Image;
use bevy::render::render_resource::Texture;


/// @resource = https://github.com/bevyengine/bevy/blob/main/examples/2d/texture_atlas.rs#enroll-beta

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    START,
    FINISHED,
}

#[derive(Resource, Default)]
pub struct SpriteHandles {
    pub handles: Vec<HandleUntyped>
}

pub fn load_textures(asset_server: Res<AssetServer>, mut sprite_handles: ResMut<SpriteHandles>) {
    sprite_handles.handles = asset_server.load_folder("sprites").unwrap();
}

pub fn check_textures(
    mut next_state: ResMut<State<AppState>>,
    mut sprite_handles: ResMut<SpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded = asset_server
        .get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        println!("\nAssets successfully loaded\n");
        next_state.set(AppState::FINISHED).expect("Failed to switch State to Finished");
    }
}