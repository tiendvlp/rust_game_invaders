use bevy::prelude::*;
use rand::{thread_rng, Rng};
use crate::{GameTextures, components::WindowSize, config::CONFIG};

struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
       app.add_startup_system(enemy_spawn_system); 
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    game_texture: Res<GameTextures>,
    win_size: Res<WindowSize>
) {
    let mut rng = thread_rng();
    let w_span = win_size.0 / 2. - 100.;
    let h_span = win_size.1 / 2. - 100.;
    let x = rng.gen_range(-w_span..w_span);
    let y = rng.gen_range(-h_span..w_span);

    commands.spawn_bundle(SpriteBundle {
        texture: game_texture.enemy.clone(),
        transform: Transform {
            translation: Vec3::new(x, y, 10.),
            scale: Vec3::new(CONFIG.SPRITE_SCALE.0, CONFIG.SPRITE_SCALE.1, 1),
            ..Default::default()
        },
        ..Default::default()
    });
}
