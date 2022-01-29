
mod constants;
use bevy::prelude::*;

#[derive(Component)]
struct Player {
    speed: f32,
}



pub fn player_movement(
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
   translation.x += direction.x * player.speed * constants::TIME_STEP;
   translation.y += direction.y * player.speed * constants::TIME_STEP;

   // bound the player within the walls
   translation.x = translation.x.min(380.0).max(-380.0);
   translation.y = translation.y.min(380.0).max(-380.0);
}
