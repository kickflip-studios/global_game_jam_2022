use bevy::{
    core::FixedTimestep,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

pub const SPRITE_DIR: &str = "assets";
pub const PLAYER_SPRITE: &str = "player.png";
pub const ELECTRON_SPRITE: &str = "electron.png";
pub const POSITRON_SPRITE: &str = "positron.png";
pub const SPHERE_SPRITE: &str = "charge_sphere.png";
pub const BACKGROUND_SPRITE: &str = "backgound.png";

pub const NORMAL_FONT: &str = "fonts/FiraSans-Medium.ttf";
pub const BOLD_FONT: &str = "fonts/FiraSans-Bold.ttf";

pub const GAME_OVER_MAIN: &str = "Game Over!";
pub const GAME_OVER_MINOR: &str ="Press r to restart";

pub const MAX_NUM_PARTICLES: u32 = 20;

pub const WALL_THICKNESS: f32 = 10.;

pub const SCALE: f32 = 0.1;
pub const TIME_STEP: f32 = 1. / 60.;

pub const ACTIVE_WIDTH: f32 = 1400.0;
pub const ACTIVE_HEIGHT: f32 = 500.0;

pub const SCREEN_WIDTH: f32 = ACTIVE_WIDTH + 2. * XOFFSET;
pub const SCREEN_HEIGHT: f32 = ACTIVE_HEIGHT + 2. * YOFFSET;

pub const XOFFSET: f32 = 100.;
pub const YOFFSET: f32 = 100.;

pub const COULOMB_CONSTANT: f32 = 50000.0;

pub const VELOCITY_SCALE: f32 = 0.0;

pub const MAXIMUM_SPEED: f32 = 10.0 * WALL_THICKNESS / TIME_STEP;

pub const EXCLUSION_RADIUS: f32 = 100.0;

#[derive(Component,PartialEq)]
pub enum Collider {
    Wall,
    Particle,
    Player,
}

#[derive(Component)]
pub struct Scoreboard {
    pub score: usize,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Playing,
    GameOver
}


// region:    Resources
pub struct SpriteInfos {
    pub player: (Handle<Image>, Vec2),
    pub particle: (Handle<Image>, Vec2),
}

#[derive(Component)]
pub struct Particle {
    pub velocity: Vec3,
    pub charge: f32,
    pub mass: f32,
    pub active: bool,
}
