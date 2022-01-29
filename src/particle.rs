use bevy::{
    core::FixedTimestep,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};


use crate::constants::*;

#[derive(Component)]
pub enum Charge {
    Positive,
    Negative,
}

#[derive(Component)]
pub enum Collider {
    Wall,
    Particle,
	Player,
}


#[derive(Component)]
pub struct Particle {
    pub position: Vec2,
	pub speed: f32,
	pub velocity: Vec2,
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
					.with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
					.with_system(particle_collision_system)
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
			speed:150.,
			velocity:Vec2::ZERO,
			charge:Charge::Positive,
			mass:100.
		}
		);
}



fn particle_collision_system(
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

            // if collision with another particle
            if let Collider::Particle = *collider {
				print!("Collision with another particle", );
            }

			// if collision with player
			if let Collider::Player = *collider {
				print!("Collision with Player", );
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
