use std::borrow::BorrowMut;
use bevy::{app::*, prelude::*, sprite::SpriteBundle};
use crate::components::{self, Velocity, Player};
use crate::WindowSize;
use crate::config::CONFIG;

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
    
    let bottom = window_size.1 / 2. - CONFIG.PLAYER_SIZE.1 * CONFIG.SPRITE_SCALE.0;
    let center = 0.;
    let mut player = commands.spawn(SpriteBundle {
        texture: asset_server.load(CONFIG.PLAYER_SPRITE.as_str()),
        transform: Transform {
            translation: Vec3::new(center, -bottom, 1.),
            scale: Vec3::new(CONFIG.SPRITE_SCALE.0, CONFIG.SPRITE_SCALE.1, 1.),
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
        translation.x += velocity.x * CONFIG.BASE_SPEED * CONFIG.TIME_STEP;
        translation.y += velocity.y * CONFIG.BASE_SPEED * CONFIG.TIME_STEP;
    }
}

