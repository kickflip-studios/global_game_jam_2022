use bevy::{
    core::FixedTimestep,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use rand::{thread_rng, Rng, random};

use crate::constants::*;


struct ActiveParticles(u32);

pub struct ParticlePlugin;


impl Plugin for ParticlePlugin {
	fn build(&self, app: &mut bevy::prelude::App) {
		app
			.insert_resource(ActiveParticles(0))
			.add_system_set(
				SystemSet::new()
					.with_run_criteria(FixedTimestep::step(0.3))
					.with_system(particle_spawn)
			)
			.add_system_set(
				SystemSet::new()
					.with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
					.with_system(particle_wall_collision_system.system())
					.with_system(particle_particle_collision_system.system())
					.with_system(particle_movement_system.system())
			);
	}
}

fn particle_spawn(
	mut commands: Commands,
	mut active_particles: ResMut<ActiveParticles>,
	asset_server: Res<AssetServer>,
) {


	if  active_particles.0 < MAX_NUM_PARTICLES
	{
	let mut rng = thread_rng();
	let charge_bool = rng.gen::<bool>();
	let charge = if charge_bool {1.} else {-1.};
	let sprite_file = if charge_bool {POSITRON_SPRITE} else {ELECTRON_SPRITE};
	let vel =  Vec3::new(rng.gen_range(-1.0..1.0) as f32, rng.gen_range(-1.0..1.0) as f32, 0.).normalize();
	let pos =  Vec2::new(
		rng.gen_range(-SCREEN_WIDTH/2.0..SCREEN_WIDTH/2.0) as f32,
		rng.gen_range(-SCREEN_HEIGHT/2.0..SCREEN_HEIGHT/2.0) as f32,
	);
	commands
		.spawn_bundle(
			SpriteBundle {
			texture: asset_server.load(sprite_file),
			transform: Transform {
			translation: Vec3::new(pos.x,pos.y, 0.),
			scale: Vec3::new(SCALE, SCALE, 1.),
			..Default::default()
			},
			..Default::default()
		})
		.insert(Particle{
			velocity:vel,
			charge,
			mass:100.
		})
		.insert(Collider::Particle);
		active_particles.0 += 1;
		info!("Added another particle (now {:?})", active_particles.0)
	}
}

// fn interact_bodies(mut query: Query<(&Mass, &GlobalTransform, &mut Acceleration)>)
// TEMP WHILE PAUL GETS THE FORCES WORKING
fn particle_movement_system(
	time: Res<Time>,
	mut query: Query<(Entity, &mut Particle, &mut Transform, &mut Collider)>,
) {
	// info!("Particle movement");
    let delta_seconds = f32::min(0.2, time.delta_seconds());
	let mut iter = query.iter_combinations_mut();
	while let Some([(id1, mut p1, mut tx1, c1), (id2, mut p2, mut tx2, c2)]) =
        iter.fetch_next() {
			// tx1.translation += p1.velocity * delta_seconds;
			// tx2.translation += p2.velocity * delta_seconds;

			let delta = tx1.translation - tx2.translation;
			let d = delta.length();
			if d > 0.01 {

				// let dt2 = delta_seconds.powi(2);
				// let scaled_vec = delta / d.powi(3);

				let dt2 = delta_seconds;
				let scaled_vec = delta / d.powi(2);

				let q1 = p1.charge;
				let q2 = p2.charge;

				let force = COULOMB_CONSTANT* q1 * q2 * scaled_vec;
				let a1 = force/p1.mass;
				let a2 = -force/p2.mass;
				let dv1 = a1 * delta_seconds;
				let dv2= a2 * delta_seconds;
				let dx1 = p1.velocity * delta_seconds + 0.5 * a1 * dt2;
				let dx2 = p2.velocity * delta_seconds + 0.5 * a2 * dt2;
				tx1.translation += dx1;
				tx2.translation += dx2;

				if let Collider::Particle = *c1
					{p1.velocity += dv1;}
				if let Collider::Particle = *c2
					{p2.velocity += dv2;}
				// info!("(p1,p2) = ({:?},{:?}), v1={:?}, v2={:?}",id1,id2, p1.velocity, p2.velocity);

			}

		}

}


pub fn particle_wall_collision_system(
	mut commands: Commands,
	sprite_infos: Res<SpriteInfos>,
    mut particle_query: Query<(Entity, &Transform, &Sprite,  &mut Particle  ), (With<Particle>)>,
    mut collider_query: Query<(Entity, &Transform, &Sprite, &Collider)>,
	)
	{
    for (particle_entity, particle_transform, particle_sprite, mut particle) in particle_query.iter_mut(){
		let particle_size = sprite_infos.particle.1 *  particle_transform.scale.truncate();
		for (mut collider_entity, collider_transform, collider_sprite, collider) in collider_query.iter_mut() {
			if let Collider::Wall = *collider {
			let mut collider_size = Vec2::ZERO;
			collider_size = collider_transform.scale.truncate();



			let collision = collide(
				particle_transform.translation, // position of particle
				particle_size,
				collider_transform.translation, // position of collider
				collider_size,
			);


			if let Some(collision) = collision {




					// reflect the ball when it collides
					let mut reflect_x = false;
					let mut reflect_y = false;

					let velocity = &mut particle.velocity;
					// only reflect if the ball's velocity is going in the opposite direction of the
					// collision
					match collision {
						Collision::Left => reflect_x = velocity.x > 0.0,
						Collision::Right => reflect_x = velocity.x < 0.0,
						Collision::Top => reflect_y = velocity.y < 0.0,
						Collision::Bottom => reflect_y = velocity.y > 0.0,
					}

					// reflect velocity on the x-axis if we hit something on the x-axis
					if reflect_x {
						velocity.x = -velocity.x;
					}

					// reflect velocity on the y-axis if we hit something on the y-axis
					if reflect_y {
						velocity.y = -velocity.y;
					}

					// break if this collide is on a wall,
					break;
				}

			}
		}
	}

}


pub fn particle_particle_collision_system(
	mut commands: Commands,
	sprite_infos: Res<SpriteInfos>,
	mut scoreboard: ResMut<Scoreboard>,
    mut query: Query<(Entity, &mut Particle, &mut Transform, &mut Collider)>,
	)
	{

	let mut iter = query.iter_combinations_mut();
	while let Some([(particle_entity, mut particle, mut particle_transform, particle_collider), (collider_entity, mut p2, mut tx2, c2)]) =
        iter.fetch_next() {}

    // for (particle_entity, particle_transform, particle_sprite, mut particle) in particle_query.iter_mut(){
	// 	let particle_size = sprite_infos.particle.1 *  particle_transform.scale.truncate();
	// 	for (collider_entity, collider_transform, collider_sprite, collider, collider_particle) in collider_query.iter() {
	//
	// 		let collider_size = sprite_infos.particle.1 *  collider_transform.scale.truncate();
	//
	// 		let collision = collide(
	// 			particle_transform.translation, // position of particle
	// 			particle_size,
	// 			collider_transform.translation, // position of collider
	// 			collider_size,
	// 		);
	//
	//
	// 		if let Some(collision) = collision {
	// 			if collider_particle.charge != particle.charge
	// 			{
	// 				if let Collider::Player = *collider {
	// 					info!("END GAME");
	// 				}
	// 				else
	// 				{
	// 					info!("INCREASE SCORE");
	// 					scoreboard.score += 1;
	// 				}
	//
	// 				commands.entity(collider_entity).despawn();
	// 				commands.entity(particle_entity).despawn();
	//				active_particles.0 -= 1; // this will allow more to spawn
	//
	// 			}
	//
	//
	// 		}
	// 	}
	// }

	}
