
use bevy::{core::FixedTimestep, prelude::*};

use crate::constants::*;

#[derive(Component)]
pub struct Particle {
    pub position: Vec2,
	pub speed: f32
}



pub struct ParticlePlugin;


impl Plugin for ParticlePlugin {
	fn build(&self, app: &mut bevy::prelude::App) {
		app
			.add_startup_system(particle_spawn)
			.add_system_set(
				SystemSet::new()
					.with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
					// .add_system(particle_collision_system.system())
			);
	}
}

fn particle_spawn(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	// spawn enemy
	commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
			custom_size: Some(Vec2::new(30.0, 30.0)),
			..Default::default()
		},
			texture: asset_server.load(POSITRON_SPRITE),
			..Default::default()
		})
		.insert(Particle{
			position:Vec2::new(30.0, 30.0),
			speed:150.}
		);
}

//
// fn particle_collision_system(
//     mut commands: Commands,
//     mut particle_query: Query<(&mut Particle, &Transform)>,
//     collider_query: Query<(Entity, &Collider, &Transform)>,
// ) {
//     let (mut ball, ball_transform) = ball_query.single_mut();
//     let ball_size = ball_transform.scale.truncate();
//     let velocity = &mut ball.velocity;
//
//     // check collision with walls
//     for (collider_entity, collider, transform) in collider_query.iter() {
//         let collision = collide(
//             ball_transform.translation,
//             ball_size,
//             transform.translation,
//             transform.scale.truncate(),
//         );
//         if let Some(collision) = collision {
//             // scorable colliders should be despawned and increment the scoreboard on collision
//             if let Collider::Scorable = *collider {
//                 scoreboard.score += 1;
//                 commands.entity(collider_entity).despawn();
//             }
//
//             // reflect the ball when it collides
//             let mut reflect_x = false;
//             let mut reflect_y = false;
//
//             // only reflect if the ball's velocity is going in the opposite direction of the
//             // collision
//             match collision {
//                 Collision::Left => reflect_x = velocity.x > 0.0,
//                 Collision::Right => reflect_x = velocity.x < 0.0,
//                 Collision::Top => reflect_y = velocity.y < 0.0,
//                 Collision::Bottom => reflect_y = velocity.y > 0.0,
//             }
//
//             // reflect velocity on the x-axis if we hit something on the x-axis
//             if reflect_x {
//                 velocity.x = -velocity.x;
//             }
//
//             // reflect velocity on the y-axis if we hit something on the y-axis
//             if reflect_y {
//                 velocity.y = -velocity.y;
//             }
//
//             // break if this collide is on a solid, otherwise continue check whether a solid is
//             // also in collision
//             if let Collider::Solid = *collider {
//                 break;
//             }
//         }
//     }
// }
