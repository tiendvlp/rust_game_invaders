#![allow(unused)]

use bevy::{prelude::*, transform};

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);

fn main() {
  App::new()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
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
    let (ww, wh) = (window.width(), window.height());
    window.set_position(MonitorSelection::Current, IVec2::new(0, 0));
    
    // Add player
    let bottom = wh / 2. - PLAYER_SIZE.1;
    let center = 0.;
    commands.spawn(SpriteBundle {
        texture: asset_server.load(PLAYER_SPRITE),
        transform: Transform {
            translation: Vec3::new(center, -bottom, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}