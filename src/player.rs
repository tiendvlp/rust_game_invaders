use crate::components::{GameTextures, Movable};

use std::borrow::BorrowMut;
use bevy::render::render_resource::Texture;
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
            .add_system(player_movement_provider_system)
            .add_system(player_keyboard_event_system)
            .add_system(player_fire_system);
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
    .insert(Velocity {x: 0., y: 0.});

    info!("Player plugin initiated");
}

fn player_movement_provider_system(
    mut query: Query<(&Velocity, &mut Transform), With<Player>>) {
    for (velocity, mut transform) in query.iter_mut() {
        let mut translation = transform.translation.borrow_mut();
        translation.x += velocity.x * CONFIG.BASE_SPEED * CONFIG.TIME_STEP;
        translation.y += velocity.y * CONFIG.BASE_SPEED * CONFIG.TIME_STEP;
    }
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if kb.pressed(KeyCode::Left) {
            -1.
        } else if kb.pressed(KeyCode::Right) {
            1.
        } else {
            0.
        };
    }
}

fn player_fire_system(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    textures: Res<GameTextures>,
    query: Query<&Transform, With<Player>>
) {
    if let Ok(player_tf) = query.get_single() {
        let (x, y) = (player_tf.translation.x, player_tf.translation.y);
        if kb.just_pressed(KeyCode::Space) {
            commands.spawn(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(x, y, 0.),
                    scale: Vec3::new(CONFIG.SPRITE_SCALE.0, CONFIG.SPRITE_SCALE.1, 1.), 
                    ..Default::default()
                  
                },
                texture: textures.player_laser.clone(),
                ..Default::default()
            })
            .insert(Movable { auto_despawn: true});
        }
    }    
}

fn player_laser_movement_system(
    mut commands: Commands,
    win_size: Res<WindowSize>,
    query: Query<(Entity, &Velocity, &mut Transform, &Movable),With<Movable>>
) {
   for (velocity, mut transform) in query.iter_mut() {
       let translation = &mut transform.translation; 
   } 
}

