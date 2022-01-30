#![allow(unused)]


use bevy::{
    core::FixedTimestep,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};


mod constants;
mod player;
mod particle;
mod walls;

use crate::constants::*;
use player::{ PlayerPlugin};
use particle::{ParticlePlugin};
use walls::spawn_walls;



fn main() {
    App::new()
		.insert_resource(ClearColor(Color::rgb(1., 1., 1.)))
		.insert_resource(WindowDescriptor {
			title: "Global game jam 2022".to_string(),
			width: constants::SCREEN_WIDTH,
			height:  constants::SCREEN_HEIGHT,
			..Default::default()
		})

		.add_plugins(DefaultPlugins)
		.add_plugin(ParticlePlugin)
		.add_plugin(PlayerPlugin)
		.add_startup_system(setup)
		.add_startup_system(spawn_walls)
		// .add_system_set(
        //     SystemSet::new()
        //         .with_run_criteria(FixedTimestep::step(constants::TIME_STEP as f64))
		// 		.with_system(player_movement)
		// 		.with_system(particle_collision_system)
		// )

		.run();
}






fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut windows: ResMut<Windows>,
){
	// camera
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());

	// Position windows on your monitor
	let mut window = windows.get_primary_mut().unwrap();
	window.set_position(IVec2::new(1000, 0));
}
