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
					.with_system(particle_spawn)
			)
			.add_system_set(
				SystemSet::new()
					.with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
					// .with_system(particle_collision_system.system())
					.with_system(particle_movement_system.system())
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
			charge:charge,
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
    mut particle_query: Query<(&mut Particle, &Transform)>,
    collider_query: Query<(Entity, &Collider, &Transform)>,
	) {
    let (mut particle, particle_transform) = particle_query.single_mut();
    let particle_size = particle_transform.scale.truncate();
    let velocity = &mut particle.velocity;


    // check collision with walls
    for (collider_entity, collider, transform) in collider_query.iter() {
        let collision = collide(
            particle_transform.translation,
            particle_size,
            transform.translation,
            transform.scale.truncate(),
        );
        if let Some(collision) = collision {

			info!("Collision occured! ", );

            // if collision with another particle
            if let Collider::Particle = *collider {
				info!("Collision with another particle", );
            }

			// if collision with player
			if let Collider::Player = *collider {
				info!("Collision with Player", );
			}

            // reflect the ball when it collides
            let mut reflect_x = false;
            let mut reflect_y = false;

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
            if let Collider::Wall = *collider {
                break;
            }
        }
    }
}
