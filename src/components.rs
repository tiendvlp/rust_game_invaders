use bevy::{
    prelude::{Component, Resource, Handle, Image},
    sprite::TextureAtlas
};

#[derive(Component)]
pub struct Velocity {
  pub x: f32,
  pub y: f32 
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Movable {
    pub auto_despawn: bool
}

#[derive(Resource)]
pub struct GameTextures {
	pub player: Handle<Image>,
	pub player_laser: Handle<Image>,
}

#[derive(Resource)]
pub struct WindowSize (pub f32, pub f32);

