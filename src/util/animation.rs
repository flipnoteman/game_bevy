// use bevy::{
//   asset::{AssetLoader, BoxedFuture, Error, LoadContext, LoadedAsset},
//   prelude::*,
//   reflect::TypeUuid,
// };
//
// use crate::util;
//
// #[derive(Component, Deref)]
// pub struct Animation(pub benimator::Animation);
//
// #[derive(Default, Component, Deref, DerefMut)]
// pub struct AnimationState(pub benimator::State);
//
// pub fn animate_sprites(
//   time: Res<Time>,
//   asset_server: ResMut<AssetServer>,
//   texture_atlas_server: ResMut<Assets<TextureAtlas>>,
//   mut query: Query<(
//     &mut AnimationState,
//     &mut TextureAtlasSprite,
//     &mut Handle<TextureAtlas>,
//     &Animation,
//     &PlayerState,
//   )>
// ) {
//   for (mut player, mut texture, mut texture_atlas_handle, animation, player_state) in query.iter_mut() {
//
//     if *player_state == PlayerState::RUNNING {
//       *texture_atlas_handle = util::asset_handling::load_texture(
//         asset_server,
//         texture_atlas_server,
//         "Player Sword Idle/Player Sword Idle 48x48.png".to_string(),
//         Vec2::new(48.0, 48.0),
//         8,
//         1,
//       );
//     }
//
//     // Update the Player State
//     player.update(animation, time.delta());
//
//     // Update the Texture atlas
//     texture.index = player.frame_index();
//   }
// }