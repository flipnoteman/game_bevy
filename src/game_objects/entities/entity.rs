use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Health(pub i32);
