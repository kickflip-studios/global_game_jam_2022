#![allow(unused)]

use bevy::prelude::*;

const PLAYER_SPRITE: &str = "player.png";
const ELECTRON_SPRITE: &str = "electron.png";
const POSITRON_SPRITE: &str = "positron.png";
const SPHERE_SPRITE: &str = "charge_sphere.png";
const BACKGROUND_SPRITE: &str = "backgound.png";


const SCALE: f32 = 0.5;
const TIME_STEP: f32 = 1. / 60.;


struct Charge {
    value: bool,
}

fn main() {
	print!("HIII", );
    App::new()
		.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
		.insert_resource(WindowDescriptor {
			title: "Global game jam 2022".to_string(),
			width: 600.0,
			height: 500.0,
			..Default::default()
		})
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup.system())
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
	commands
	.spawn_bundle(SpriteBundle {
		material: materials.add(asset_server.load(PLAYER_SPRITE).into()),
		..Default::default()
	});
}