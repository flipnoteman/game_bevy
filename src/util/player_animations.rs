use bevy::asset::{Assets, AssetServer};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Res, ResMut, TextureAtlas, Transform};
use bevy::sprite::SpriteSheetBundle;
use crate::util;

pub fn load_player_animations(
    asset_server: ResMut<AssetServer>,
    texture_atlas_server: ResMut<Assets<TextureAtlas>>,
)/* -> Vec<SpriteSheetBundle> */{
//     let mut sprite_sheets:Vec<SpriteSheetBundle> = vec![];
//
//     let sprite_sheet_1 = SpriteSheetBundle {
//         texture_atlas: util::asset_handling::load_texture(
//             asset_server,
//             texture_atlas_server,
//             "Player Sword Run/Player Sword Run 48x48.png".to_string(),
//             Vec2::new(48.0, 48.0),
//             8,
//             1,
//         ),
//         transform: Transform::from_scale(Vec3::splat(2.0)),
//         ..default()
//     };
//
//     sprite_sheets.push(sprite_sheet_1);
//     sprite_sheets
}