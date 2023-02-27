use bevy::asset::AssetServer;
use bevy::ecs::bundle::Bundle;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Res};
use bevy::sprite::SpriteBundle;

use crate::game_objects::game_object::Name;

#[derive(Bundle)]
pub struct PropBundle{
    pub name: Name,
    /*pub position: Vec2,*/

    #[bundle]
    pub model: SpriteBundle
}

impl PropBundle {
    pub fn spawn_prop(commands: Commands){
        //TODO
    }

    pub fn default() -> PropBundle{
        PropBundle{
            name: Name("Prop".to_string()),
           /* position: Vec2::ZERO,*/
            model: Default::default()
        }
    }
}