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
use player::{player_movement, Player};
use particle::{ParticlePlugin, particle_collision_system};
use walls::spawn_walls;



#[derive(Component)]
struct Charge {
    value: bool,
}

fn main() {
	println!("HIII", );
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
		.add_startup_system(setup)
		.add_startup_system(spawn_walls)
		.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(constants::TIME_STEP as f64))
				.with_system(player_movement)
				.with_system(particle_collision_system)
		)

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

	// spawn a sprite
	let bottom = window.height()/2.;

	// TODO: make a function in player module that is called here
	commands
		.spawn_bundle(
			SpriteBundle {
			texture: asset_server.load(PLAYER_SPRITE),
			transform: Transform {
			translation: Vec3::new(0., bottom/ 4., 10.),
			scale: Vec3::new(SCALE, SCALE, 1.),
			..Default::default()
			},
			..Default::default()
		})
		.insert(Player{speed:150.})
		.insert(Collider::Player);

}
