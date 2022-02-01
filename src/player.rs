use bevy::prelude::*;

use crate::constants::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(player_spawn)
            .add_system(player_movement);
    }
}

pub fn player_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(PLAYER_SPRITE),
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                scale: Vec3::new(SCALE, SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player { speed: 150. })
        .insert(Collider::Player)
        .insert(Particle {
            velocity: Vec3::ZERO,
            charge: -1.,
            mass: 1.,
        });
}

pub fn player_movement(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    let (player, mut transform) = query.single_mut();
    let mut direction = Vec2::new(0.0, 0.0);
    if keyboard_input.pressed(KeyCode::Left) {
        direction.x -= 1.;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        direction.x += 1.;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        direction.y -= 1.;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        direction.y += 1.;
    }
    let translation = &mut transform.translation;
    // move the player
    translation.x += direction.x * player.speed * TIME_STEP;
    translation.y += direction.y * player.speed * TIME_STEP;

    // bound the player within the walls
    // FIXME: this isnt quite working atm...
    translation.x = translation
        .x
        .min(SCREEN_WIDTH / 2. - WALL_THICKNESS)
        .max(-SCREEN_WIDTH / 2. + WALL_THICKNESS);
    translation.y = translation
        .y
        .min(SCREEN_HEIGHT / 2. - WALL_THICKNESS)
        .max(-SCREEN_HEIGHT / 2. + WALL_THICKNESS);
}
