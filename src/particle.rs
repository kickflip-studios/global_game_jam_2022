use bevy::app::AppExit;
use bevy::{
    app::Events,
    core::FixedTimestep,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use rand::{random, thread_rng, Rng};

use crate::constants::*;

pub struct ActiveParticles {
    pub positrons: u32,
    pub electrons: u32,
    pub total: u32,
}

fn increase_particle_count(mut particle_count: ResMut<ActiveParticles>, mut charge: f32) -> ResMut<ActiveParticles>
{
    if charge < 0.
    {particle_count.electrons += 1;}
    else
    {particle_count.positrons += 1;}
    particle_count.total = particle_count.positrons + particle_count.electrons;
    particle_count
}

fn decrease_particle_count( mut particle_count: ResMut<ActiveParticles>, mut charge: f32) -> ResMut<ActiveParticles>
{
    if charge < 0.
    {particle_count.electrons -= 1;}
    else
    {particle_count.positrons -= 1;}
    particle_count.total = particle_count.positrons + particle_count.electrons;
    particle_count
}

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ActiveParticles{positrons:0, electrons:0, total:0})
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_run_criteria(FixedTimestep::step(0.3))
                    .with_system(particle_spawn),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(particle_wall_collision_system.system())
                    .with_system(particle_particle_collision_system.system())
                    .with_system(particle_movement_system.system()),
            );
    }
}





fn particle_spawn(
    mut commands: Commands,
    mut particle_count: ResMut<ActiveParticles>,
    mut game_state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
) {

    if *game_state.current() != GameState::Playing {
        return;
    }

    if particle_count.total < MAX_NUM_PARTICLES {
        let mut rng = thread_rng();
        let mut charge_bool = rng.gen::<bool>();

        #[warn(clippy::if_same_then_else)]
        if particle_count.positrons == MAX_NUM_PARTICLES -1
        {
            charge_bool = false; // force next particle --> electron
        }
        else if particle_count.electrons == MAX_NUM_PARTICLES -1
        {
            charge_bool = true; // force next particle --> positron
        }

        let mut charge = 0. ;
        let mut sprite_file = "";
        if charge_bool {
            charge = 1.;
            sprite_file = POSITRON_SPRITE;
         }
         else {
            charge = -1.;
            sprite_file = ELECTRON_SPRITE;
          }
          info!("particle_count before increase = {:?}, {:?}", particle_count.electrons, particle_count.positrons);
          particle_count = increase_particle_count(particle_count, charge);
          info!("particle_count after increase = {:?}, {:?}", particle_count.electrons, particle_count.positrons);

        // slow init vel so as to not kill player immediatly on spawn
        let vel = Vec3::new(
            rng.gen_range(-0.1..0.1) as f32,
            rng.gen_range(-0.1..0.1) as f32,
            0.,
        )
        .normalize()
            * VELOCITY_SCALE;
        let pos = Vec2::new(
            rng.gen_range(-SCREEN_WIDTH / 2.0..SCREEN_WIDTH / 2.0) as f32,
            rng.gen_range(-SCREEN_HEIGHT / 2.0..SCREEN_HEIGHT / 2.0) as f32,
        );
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load(sprite_file),
                transform: Transform {
                    translation: Vec3::new(pos.x, pos.y, 0.),
                    scale: Vec3::new(SCALE, SCALE, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Particle {
                velocity: vel,
                charge,
                mass: 1.,
            })
            .insert(Collider::Particle);

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
    while let Some([(id1, mut p1, mut tx1, c1), (id2, mut p2, mut tx2, c2)]) = iter.fetch_next() {
        // tx1.translation += p1.velocity * delta_seconds;
        // tx2.translation += p2.velocity * delta_seconds;

        let delta = tx1.translation - tx2.translation;
        let d = delta.length();
        if d > 10.0 {
            // let dt2 = delta_seconds.powi(2);
            // let scaled_vec = delta / d.powi(3);

            let dt2 = delta_seconds;
            let scaled_vec = delta / d.powi(3);

            let q1 = p1.charge;
            let q2 = p2.charge;

            let force = COULOMB_CONSTANT * q1 * q2 * scaled_vec;
            let a1 = force / p1.mass;
            let a2 = -force / p2.mass;
            let mut dv1 = a1 * delta_seconds;
            let mut dv2 = a2 * delta_seconds;
            let mut dx1 = p1.velocity * delta_seconds + 0.5 * a1 * dt2;
            let mut dx2 = p2.velocity * delta_seconds + 0.5 * a2 * dt2;
            // let speed1 = dx1.length()/delta_seconds;
            // let speed2 = dx2.length()/delta_seconds;
            // if speed1 > MAXIMUM_SPEED {
            // 	dx1 = dx1 * MAXIMUM_SPEED / speed1;
            // }
            // if speed2 > MAXIMUM_SPEED {
            // 	dx2 = dx2 * MAXIMUM_SPEED / speed2;
            // }
            tx1.translation += dx1;
            tx2.translation += dx2;

            if let Collider::Particle = *c1 {
                p1.velocity += dv1;
            }
            if let Collider::Particle = *c2 {
                p2.velocity += dv2;
            }
        }
    }
}

pub fn particle_wall_collision_system(
    mut commands: Commands,
    sprite_infos: Res<SpriteInfos>,
    mut particle_query: Query<(Entity, &Transform, &Sprite, &mut Particle), (With<Particle>)>,
    mut collider_query: Query<(Entity, &Transform, &Sprite, &Collider)>,
) {
    for (particle_entity, particle_transform, particle_sprite, mut particle) in
        particle_query.iter_mut()
    {
        let particle_size = sprite_infos.particle.1 * particle_transform.scale.truncate();
        for (mut collider_entity, collider_transform, collider_sprite, collider) in
            collider_query.iter_mut()
        {
            if let Collider::Wall = *collider {
                let mut collider_size = Vec2::ONE * 10.0;
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
    mut particle_count: ResMut<ActiveParticles>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    sprite_infos: Res<SpriteInfos>,
    mut scoreboard: ResMut<Scoreboard>,
    mut game_state: ResMut<State<GameState>>,
    mut query: Query<(
        Entity,
        &mut Particle,
        &mut Transform,
        &Sprite,
        &mut Collider,
    )>,
) {
    let mut iter = query.iter_combinations_mut();
    while let Some([(id1, mut p1, mut tf1, sp1, c1), (id2, mut p2, mut tf2, sp2, c2)]) =
        iter.fetch_next()
    {
        let p1_size = sprite_infos.particle.1 * tf1.scale.truncate();
        let p2_size = sprite_infos.particle.1 * tf2.scale.truncate();
        let collision = collide(
            tf1.translation, // position of particle
            p1_size,
            tf2.translation, // position of collider
            p2_size,
        );
        if let Some(collision) = collision {
            if p2.charge != p1.charge {
                if Collider::Player == *c1 || Collider::Player == *c2{
                    info!("END GAME");
                    // app_exit_events.send(AppExit);
                     let _ = game_state.overwrite_set(GameState::GameOver);
                }
                else {
                    scoreboard.score += 1;
                    info!("INCREASE SCORE = {}", scoreboard.score);
                }
                // delete particles
                commands.entity(id1).despawn();
                commands.entity(id2).despawn();

                // update counts
                info!("particle_count before decrease by 2 = {:?}, {:?}", particle_count.electrons, particle_count.positrons);
                particle_count = decrease_particle_count(particle_count, p1.charge);
                particle_count = decrease_particle_count(particle_count, p2.charge);
                info!("particle_count after decrease by 2 = {:?}, {:?}", particle_count.electrons, particle_count.positrons);

            }
        }
    }
}
