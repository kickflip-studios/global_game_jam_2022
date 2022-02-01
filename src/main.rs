
#![allow(unused)]

use bevy::{
    core::FixedTimestep,
    prelude::*,
	render::texture::ImageType,
    sprite::collide_aabb::{collide, Collision},
	asset::AssetServerSettings
};
use std::path::Path;

mod constants;
mod player;
mod particle;
mod walls;

use crate::constants::*;
use player::{ PlayerPlugin};
use particle::ParticlePlugin;
use walls::spawn_walls;

use web_sys::console;




fn main() {
    App::new()
		.insert_resource(AssetServerSettings {
			asset_folder: "/".to_string(),
		})
		.insert_resource(ClearColor(Color::rgb(1., 1., 1.)))
		.insert_resource(WindowDescriptor {
			title: "Global game jam 2022".to_string(),
			width: constants::SCREEN_WIDTH,
			height:  constants::SCREEN_HEIGHT,
			..Default::default()
		})
		.add_startup_system(spawn_walls)
		.add_plugins(DefaultPlugins)
		.add_plugin(PlayerPlugin)
		.add_plugin(ParticlePlugin)
		.add_startup_system(setup)
		.run();
}

#[derive(Clone)]
struct State {
    handle: Handle<Image>,
    printed: bool,
	size: Vec2,
}

impl State {
	fn get_handle_and_size(&self) -> (Handle<Image>, Vec2) {
		(self.clone().handle, self.size)
	}
}



// fn load_image(mut commands: Commands, asset_server: Res<AssetServer>, path: &str, size: Vec2) -> (Handle<Image>, Vec2) {
// 	// Note - With bevy v0.6, load images directly and synchronously to capture size
// 	//        See https://github.com/bevyengine/bevy/pull/3696
// 	console::log_1(&format!("Loading {}", path).into());
// 	let handle = asset_server.load(path);
// 	commands.insert_resource(State {
//         handle,
//         printed: false,
//     });

// 	(handle, size)
// }



// fn load_image(images: &mut ResMut<Assets<Image>>, path: &str) -> (Handle<Image>, Vec2) {
// 	// Note - With bevy v0.6, load images directly and synchronously to capture size
// 	//        See https://github.com/bevyengine/bevy/pull/3696
// 	console::log_1(&format!("Loading {}", path).into());
// 	let path = Path::new(SPRITE_DIR).join(path);
// 	let bytes = std::fs::read(&path).unwrap_or_else(|_| panic!("Cannot find {:?}", path));
// 	let image = Image::from_buffer(&bytes, ImageType::MimeType("image/png")).unwrap();
// 	let size = image.texture_descriptor.size;
// 	let size = Vec2::new(size.width as f32, size.height as f32);
// 	let image_handle = images.add(image);
// 	(image_handle, size)
// }

fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut images: ResMut<Assets<Image>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut windows: ResMut<Windows>,
){
	// camera
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());

	// Position windows on your monitor
	let mut window = windows.get_primary_mut().unwrap();
	window.set_position(IVec2::new(1000, 0));


	let player_resource = State {
		handle: asset_server.load(PLAYER_SPRITE),
		printed: false,
		size: Vec2::new(PLAYER_SPRITE_X as f32, PLAYER_SPRITE_Y as f32),
	};
	
	commands.insert_resource(player_resource.clone());

	let particle_resource = State {
		handle: asset_server.load(POSITRON_SPRITE),
		printed: false,
		size: Vec2::new(POSITRON_SPRITE_X as f32, POSITRON_SPRITE_Y as f32),
	};
	
	commands.insert_resource(particle_resource.clone());


	commands.insert_resource(SpriteInfos {
		player: player_resource.get_handle_and_size(),
		particle: particle_resource.get_handle_and_size(),
	});

	let mut collider_query: Query<(Entity, &Transform, &Sprite, &Collider)>;

	// info!("Player sprite size: {:?}", load_image(&mut images, PLAYER_SPRITE).1);
	// info!("Particle sprite size: {:?}", load_image(&mut images, POSITRON_SPRITE));

}
