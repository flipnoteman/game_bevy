use bevy::prelude::*;
mod game_objects;
use crate::game_objects::entities::*;
use crate::util::asset_handling::*;
use bevy::asset::{Asset, LoadState};

mod util;

fn main() {
    App::new()
        .init_resource::<SpriteHandles>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_state::<AppState>(Default::default())
        .add_system(load_textures.in_schedule)
        .add_startup_system(check_textures)
        .add_startup_system(setup)
        // .add_startup_system(camera_controller::spawn_camera)
        // .add_startup_system(player::PlayerBundle::spawn_player)
        //.add_startup_system(enemy::EnemyBundle::spawn_enemy)
        .add_system(camera_controller::move_camera)
        // .add_system(util::animation::animate_sprites)
        .add_system(player_movement::player_move)
        .run();
}

fn setup(
    mut commands: Commands,
    sprite_handles: Res<SpriteHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in &sprite_handles.handles {
        let handle = handle.typed_weak();
        let Some(texture) = textures.get(&handle) else {
            warn!("{:?} did not resolve to an `Image` asset.", asset_server.get_handle_path(handle));
            continue;
        };

        texture_atlas_builder.add_texture(handle, texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let texture_atlas_texture = texture_atlas.texture.clone();
    let idle_handle = asset_server.get_handle("sprites/Player Sword Idle/Player Sword Idle 48x48.png");
    let idle_index = texture_atlas.get_texture_index(&idle_handle).unwrap();
    let atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteSheetBundle {
        transform: Transform {
            translation: Vec3::new(150.0, 0.0, 0.0),
            scale: Vec3::splat(4.0),
            ..default()
        },
        sprite: TextureAtlasSprite::new(idle_index),
        texture_atlas: atlas_handle,
        ..default()
    });

    commands.spawn(SpriteBundle {
        texture: texture_atlas_texture,
        transform: Transform::from_xyz(-300.0, 0.0, 0.0),
        ..default()
    });

}


