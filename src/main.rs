#![allow(unused)]

use bevy::{
    core::FixedTimestep,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

const PLAYER_SPRITE: &str = "player.png";
const ELECTRON_SPRITE: &str = "electron.png";
const POSITRON_SPRITE: &str = "positron.png";
const SPHERE_SPRITE: &str = "charge_sphere.png";
const BACKGROUND_SPRITE: &str = "backgound.png";


const SCALE: f32 = 0.5;
const TIME_STEP: f32 = 1. / 60.;


#[derive(Component)]
struct Player {
    speed: f32,
}

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
			width: 600.0,
			height: 500.0,
			..Default::default()
		})
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup)
		.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
		)
		.run();
}


fn player_movement(
	mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
	let (player, mut transform) = query.single_mut();
	let mut  direction = Vec2::new(0.0, 0.0);
    if keyboard_input.pressed(KeyCode::Left) {
        direction.x -= 1.;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        direction.x += 1.;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        direction.y -=1.;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        direction.y += 1.;
    }
	let translation = &mut transform.translation;
   // move the player
   translation.x += direction.x * player.speed * TIME_STEP;
   translation.y += direction.y * player.speed * TIME_STEP;

   // bound the player within the walls
   translation.x = translation.x.min(380.0).max(-380.0);
   translation.y = translation.y.min(380.0).max(-380.0);
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

	commands.spawn_bundle(SpriteBundle {
		sprite: Sprite {
        custom_size: Some(Vec2::new(30.0, 30.0)),
        ..Default::default()
    },
        texture: asset_server.load(PLAYER_SPRITE),
        ..Default::default()
	});
}
