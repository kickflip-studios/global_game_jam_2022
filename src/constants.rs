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

pub const MAX_NUM_PARTICLES: u32 = 10;

pub const WALL_THICKNESS: f32 = 10.;

pub const SCALE: f32 = 0.1;
pub const TIME_STEP: f32 = 1. / 60.;

pub const SCREEN_WIDTH: f32 = 600.0;
pub const SCREEN_HEIGHT: f32 = 500.0;

pub const COULOMB_CONSTANT: f32 = 50000.0;

pub const VELOCITY_SCALE: f32 = 0.0;

pub const MAXIMUM_SPEED: f32 = 10.0 * WALL_THICKNESS / TIME_STEP;

#[derive(Component)]
pub enum Collider {
	Wall,
	Particle,
	Player,
}

pub struct Scoreboard {
	pub score: usize,
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
}
