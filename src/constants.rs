use bevy::{
    core::FixedTimestep,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};


// pub const SPRITE_DIR: &str = "/";
pub const PLAYER_SPRITE: &str = "player.png";
pub const PLAYER_SPRITE_X: usize = 202;
pub const PLAYER_SPRITE_Y: usize = 202;
pub const ELECTRON_SPRITE: &str = "electron.png";
pub const ELECTRON_SPRITE_X: usize = 191;
pub const ELECTRON_SPRITE_Y: usize = 191;
pub const POSITRON_SPRITE: &str = "positron.png";
pub const POSITRON_SPRITE_X: usize = 191;
pub const POSITRON_SPRITE_Y: usize = 191;
pub const SPHERE_SPRITE: &str = "charge_sphere.png";
pub const SPHERE_SPRITE_X: usize = 191;
pub const SPHERE_SPRITE_Y: usize = 189;
pub const BACKGROUND_SPRITE: &str = "backgound.png";
pub const BACKGROUND_SPRITE_X: usize = 2002;
pub const BACKGROUND_SPRITE_Y: usize = 1125;


pub const WALL_THICKNESS: f32 = 10.;

pub const SCALE: f32 = 0.1;
pub const TIME_STEP: f32 = 1. / 60.;

pub const SCREEN_WIDTH: f32 = 600.0;
pub const SCREEN_HEIGHT: f32 = 500.0;

pub const COULOMB_CONSTANT: f32 = 10000.0;

#[derive(Component)]
pub enum Collider {
    Wall,
    Particle,
	Player,
}


// region:    Resources
pub struct SpriteInfos {
	pub player: (Handle<Image>, Vec2),
	pub particle: (Handle<Image>, Vec2),
}
