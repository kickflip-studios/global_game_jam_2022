use bevy::{
    core::FixedTimestep,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use rand::{thread_rng, Rng, random};

use crate::constants::*;

#[derive(Component)]
pub enum Charge {
    Positive,
    Negative,
}



#[derive(Component)]
pub struct Particle {
	pub position: Vec2,
	pub speed: f32,
	pub velocity: Vec3,
	pub charge: Charge,
	pub mass: f32
}



pub struct ParticlePlugin;


impl Plugin for ParticlePlugin {
	fn build(&self, app: &mut bevy::prelude::App) {
		app
			.add_startup_system(particle_spawn)
			.add_system_set(
				SystemSet::new()
					.with_run_criteria(FixedTimestep::step(0.3))
					// .with_system(particle_spawn)
			)
			.add_system_set(
				SystemSet::new()
					.with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
					.with_system(particle_collision_system.system())
					// .with_system(particle_movement_system.system())
			);
	}
}

fn particle_spawn(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {

	let mut rng = thread_rng();
	let charge_bool = rng.gen::<bool>();
	let charge = if charge_bool {Charge::Positive} else {Charge::Negative};
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
			position: pos,
			speed:150.,
			velocity:vel,
			charge,
			mass:100.
		})
		.insert(Collider::Particle);
}


// TEMP WHILE PAUL GETS THE FORCES WORKING
fn particle_movement_system(time: Res<Time>, mut particle_query: Query<(&Particle, &mut Transform)>) {
    let delta_seconds = f32::min(0.2, time.delta_seconds());
    for (particle, mut transform) in particle_query.iter_mut() {
        transform.translation += particle.velocity * particle.speed * delta_seconds;
    }
}


pub fn particle_collision_system(
	mut commands: Commands,
	sprite_infos: Res<SpriteInfos>,
    mut particle_query: Query<(Entity, &Transform, &Sprite,  &mut Particle  ), (With<Particle>)>,
    mut collider_query: Query<(Entity, &Transform, &Sprite, &Collider)>,
	) {
    for (particle_entity, particle_transform, particle_sprite, mut particle) in particle_query.iter_mut(){
		let particle_size = sprite_infos.particle.1 *  particle_transform.scale.truncate();
		for (mut collider_entity, collider_transform, collider_sprite, collider, ) in collider_query.iter_mut() {

			let mut collider_size = Vec2::ZERO;
			if let Collider::Wall = *collider {collider_size = collider_transform.scale.truncate();}
			else {collider_size = sprite_infos.particle.1 *  collider_transform.scale.truncate();}

			let collision = collide(
				particle_transform.translation, // position of particle
				particle_size,
				collider_transform.translation, // position of collider
				collider_size,
			);

			// info!("Checking collision bw particle and {:?}\n",collider);
			// info!("Particle {:?} {:?}\n", particle_size, particle_transform.translation);
			// info!("{:?} {:?} {:?}",  collider, collider_size, collider_transform.translation);

			if let Some(collision) = collision {

				// info!("COLLISION OCCURED");


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

				// break if this collide is on a solid, otherwise continue check whether a solid is
				// also in collision
				if let Collider::Wall = *collider {break;}

			}
		}
	}

}
