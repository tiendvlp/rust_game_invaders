use bevy::prelude::Component;

#[derive(Component)]
pub struct Velocity {
  pub x: f32,
  pub y: f32 
}

#[derive(Component)]
pub struct Player;

const SPRITE_SCALE: (f32, f32) = (0.5, 0.5);
const PLAYER_SIZE: (f32, f32) = (144., 75.);
