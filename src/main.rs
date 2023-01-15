#![allow(unused)]
#[macro_use]
extern crate lazy_static;
use std::{sync::Arc, borrow::BorrowMut};
mod components;
mod player;
mod config;
use bevy::{prelude::*, transform};
use components::{GameTextures, Movable};
use config::CONFIG;

use crate::components::{Player, Velocity, WindowSize};
use crate::player::PlayerPlugin;

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
    .add_system(movement_system)
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

    // Add Game Asset Resource
    let game_resource = GameTextures {
        player: asset_server.load(CONFIG.PLAYER_SPRITE.as_str()),
        player_laser: asset_server.load(CONFIG.PLAYER_LASER_SPRITE.as_str())
    };
    commands.insert_resource(game_resource);
}

fn movement_system(
    mut commands: Commands,
    win_size: Res<WindowSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>) {
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        let mut translation = transform.translation.borrow_mut();
        translation.x += velocity.x * CONFIG.BASE_SPEED * CONFIG.TIME_STEP;
        translation.y += velocity.y * CONFIG.BASE_SPEED * CONFIG.TIME_STEP;

        if movable.auto_despawn {
            if translation.y > win_size.1 / 2. ||
               translation.y < -win_size.1 / 2. || 
               translation.x > win_size.0 / 2. || 
               translation.x < -win_size.0 / 2. {
                commands.entity(entity).despawn();
            } 
        }
    }
}

