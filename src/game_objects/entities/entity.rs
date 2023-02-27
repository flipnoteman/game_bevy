use bevy::ecs::component::Component;
use bevy::prelude::Resource;
use bevy::sprite::{SpriteSheetBundle, TextureAtlas};

#[derive(Component)]
pub struct Health(pub i32);

#[derive(Resource)]
pub struct SpriteSheets(pub Vec<SpriteSheetBundle>);
