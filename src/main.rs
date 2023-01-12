#![allow(unused)]

use std::{sync::Arc, borrow::BorrowMut};
mod components;
mod player;
use bevy::{prelude::*, transform};

use crate::components::{Player, Velocity};
use crate::player::PlayerPlugin;
const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const SPRITE_SCALE: (f32, f32) = (0.5, 0.5);
const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

#[derive(Resource)]
pub struct WindowSize (f32, f32);

fn main() {
  App::new()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .add_plugin(PlayerPlugin::new())
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "Rust Invader!".to_string(),
            width: 600.0,
            height: 700.0,
            ..Default::default()
        },
        ..Default::default()
    }))
    .add_startup_system(setup_systems)
    .run();
}

fn setup_systems(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>
) {
    // Add camera
    commands.spawn(Camera2dBundle::default());
    // Capture window size
    let window = windows.get_primary_mut().unwrap();
    let window_size = WindowSize(window.width(), window.height());
    commands.insert_resource(window_size);
    window.set_position(MonitorSelection::Current, IVec2::new(0, 0));
}

