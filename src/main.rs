#![allow(unused)]


use bevy::{
    core::FixedTimestep,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

mod constants;
mod player;
use player::player_movement;



#[derive(Component)]
struct Charge {
    value: bool,
}

fn main() {
	print!("HIII", );
    App::new()
		.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
		.insert_resource(WindowDescriptor {
			title: "Global game jam 2022".to_string(),
			width: constants::SCREEN_WIDTH,
			height:  constants::SCREEN_HEIGHT,
			..Default::default()
		})
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup)
		.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(constants::TIME_STEP as f64))
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
	window.set_position(IVec2::new(1500, 0));

	// spawn a sprite
	let bottom = window.height()/2.;

	// TODO: make a function in player module that is called here
	commands.spawn_bundle(SpriteBundle {
		sprite: Sprite {
        custom_size: Some(Vec2::new(30.0, 30.0)),
        ..Default::default()
    },
        texture: asset_server.load(constants::PLAYER_SPRITE),
        ..Default::default()
	});
}
