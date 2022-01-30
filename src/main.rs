#![allow(unused)]

use bevy::{
    core::FixedTimestep,
    prelude::*,
    render::texture::ImageType,
    sprite::collide_aabb::{collide, Collision},
};
use std::path::Path;

mod constants;
mod particle;
mod player;
mod scoreboard;
mod walls;

use crate::constants::*;
use particle::ParticlePlugin;
use player::PlayerPlugin;
use scoreboard::ScorePlugin;
use walls::spawn_walls;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: "Global game jam 2022".to_string(),
            width: constants::SCREEN_WIDTH,
            height: constants::SCREEN_HEIGHT,
            ..Default::default()
        })
        .add_startup_system(spawn_walls)
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(ParticlePlugin)
        .add_plugin(ScorePlugin)
        .add_startup_system(setup)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn load_image(images: &mut ResMut<Assets<Image>>, path: &str) -> (Handle<Image>, Vec2) {
    // Note - With bevy v0.6, load images directly and synchronously to capture size
    //        See https://github.com/bevyengine/bevy/pull/3696
    let path = Path::new(SPRITE_DIR).join(path);
    let bytes = std::fs::read(&path).unwrap_or_else(|_| panic!("Cannot find {:?}", path));
    let image = Image::from_buffer(&bytes, ImageType::MimeType("image/png")).unwrap();
    let size = image.texture_descriptor.size;
    let size = Vec2::new(size.width as f32, size.height as f32);
    let image_handle = images.add(image);
    (image_handle, size)
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Position windows on your monitor
    let mut window = windows.get_primary_mut().unwrap();
    window.set_position(IVec2::new(1000, 0));

    commands.insert_resource(SpriteInfos {
        player: load_image(&mut images, PLAYER_SPRITE),
        particle: load_image(&mut images, POSITRON_SPRITE),
    });

    let mut collider_query: Query<(Entity, &Transform, &Sprite, &Collider)>;

    info!(
        "Player sprite size: {:?}",
        load_image(&mut images, PLAYER_SPRITE).1
    );
    info!(
        "Particle sprite size: {:?}",
        load_image(&mut images, POSITRON_SPRITE).1
    );
}
