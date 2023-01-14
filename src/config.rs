pub struct Config {
    pub PLAYER_SPRITE: String,
    pub PLAYER_LASER_SPRITE: String,
    pub PLAYER_LASER_SIZE: (f32, f32),
    pub SPRITE_SCALE: (f32, f32),
    pub PLAYER_SIZE: (f32, f32),
    pub BASE_SPEED: f32,
    pub TIME_STEP: f32
}

impl Default for Config {
    fn default() -> Self {
        Config {
            PLAYER_SIZE: (144., 75.),
            SPRITE_SCALE: (0.5, 0.5),
            PLAYER_SPRITE: String::from("player_a_01.png"),
            BASE_SPEED: 500.,
            TIME_STEP: 1. / 60.,
            PLAYER_LASER_SPRITE: String::from("laser_a_01.png"),
            PLAYER_LASER_SIZE: (9., 54.),
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Default::default();
}

