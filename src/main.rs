#![allow(unused)]
#[macro_use]
extern crate lazy_static;

use std::{sync::Arc, borrow::BorrowMut};
mod components;
mod player;
mod config;
use bevy::{prelude::*, transform};

use crate::components::{Player, Velocity};
use crate::player::PlayerPlugin;

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

