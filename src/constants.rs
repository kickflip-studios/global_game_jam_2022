use bevy::{
    core::FixedTimestep,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

pub const PLAYER_SPRITE: &str = "player.png";
pub const ELECTRON_SPRITE: &str = "electron.png";
pub const POSITRON_SPRITE: &str = "positron.png";
pub const SPHERE_SPRITE: &str = "charge_sphere.png";
pub const BACKGROUND_SPRITE: &str = "backgound.png";


pub const WALL_THICKNESS: f32 = 10.;

pub const SCALE: f32 = 0.1;
pub const TIME_STEP: f32 = 1. / 60.;

pub const SCREEN_WIDTH: f32 = 600.0;
pub const SCREEN_HEIGHT: f32 = 500.0;

#[derive(Component)]
pub enum Collider {
    Wall,
    Particle,
	Player,
}
