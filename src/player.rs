use std::borrow::BorrowMut;

use bevy::{app::*, prelude::*, sprite::SpriteBundle};
use crate::components::{self, Velocity, Player};
use crate::{WindowSize, TIME_STEP};

const PLAYER_SPRITE: &str = "player_a_01.png";
const SPRITE_SCALE: (f32, f32) = (0.5, 0.5);
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const BASE_SPEED: f32 = 1.;
pub struct PlayerPlugin;

impl PlayerPlugin {
    pub fn new() -> Self {
        PlayerPlugin
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system(player_movement_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut window_size: ResMut<WindowSize>) {
    
    let bottom = window_size.1 / 2. - PLAYER_SIZE.1 * SPRITE_SCALE.0;
    let center = 0.;
    let mut player = commands.spawn(SpriteBundle {
        texture: asset_server.load(PLAYER_SPRITE),
        transform: Transform {
            translation: Vec3::new(center, -bottom, 1.),
            scale: Vec3::new(SPRITE_SCALE.0, SPRITE_SCALE.1, 1.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Player)
    .insert(Velocity {x: 1., y: 0.});

    info!("Player plugin initiated");
}

fn player_movement_system(
    mut query: Query<(&Velocity, &mut Transform), With<Player>>) {
    for (velocity, mut transform) in query.iter_mut() {
        let mut translation = transform.translation.borrow_mut();
        translation.x += velocity.x * BASE_SPEED * TIME_STEP;
        translation.y += velocity.y * BASE_SPEED * TIME_STEP;
    }
}

