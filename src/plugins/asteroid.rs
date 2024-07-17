use std::ops::Range;

use bevy::prelude::*;
use rand::Rng;
use super::{asset_loader::SceneAssets, collision_detection::Collider, movement::{Acceleration, MovingObjectBundle, Velocity}};

const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;
const VELOCITY_SCALER:f32 = 10.0;
const ACCELERATION_SCALER:f32 = 1.0;
const SPAWN_TIME_SECONDS: f32 = 1.0;
const ROTATION_SPEED: f32 = 2.5;
const RADIUS: f32 = 2.5;

pub struct AesteroidPlugin;
impl Plugin for AesteroidPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(SpawnTimer { timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating)})
        .add_systems(Update, (spawn_aesteroid, rotate_asteroids, handle_asteroid_collision));
    }
}

#[derive(Component, Debug)]
pub struct Asteroid;    //TAG

#[derive(Resource, Debug)]
pub struct SpawnTimer{
    timer: Timer,
}

fn spawn_aesteroid(mut commands: Commands, mut spawn_timer: ResMut<SpawnTimer>, time: Res<Time>, scene_assets: Res<SceneAssets>) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    let translation = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X),
        0.,
        rng.gen_range(SPAWN_RANGE_Z),
    );

    //#NOTE: A helper function to generate random unit vector
    //#NOTE: normalize_or_zero() is to make sure the length of the Vec3 will be 1.
    let mut random_unit_vector = || Vec3::new(rng.gen_range(-1.0..1.0), 0., rng.gen_range(-1.0..1.0)).normalize_or_zero();

    let velocity = random_unit_vector() * VELOCITY_SCALER;
    let acceleration = random_unit_vector() * ACCELERATION_SCALER;

    commands.spawn((MovingObjectBundle {
        velocity: Velocity::new(velocity),
        acceleration: Acceleration::new(acceleration),
        collider: Collider::new(RADIUS),
        model: SceneBundle {
            //#NOTE: Models need to be located under the "assets" folder at the root level, not at the src level
            scene: scene_assets.asteroid.clone(),
            transform: Transform::from_translation(translation),
            ..default()
        },
    }, Asteroid));
}

fn rotate_asteroids(
    mut query: Query<&mut Transform, With<Asteroid>>, 
    time: Res<Time>,
){
    for mut transform in &mut query {
        transform.rotate_local_z(ROTATION_SPEED * time.delta_seconds());
    }
}

fn handle_asteroid_collision(
    mut commands: Commands,
    query: Query<(Entity, &Collider), With<Asteroid>>, 
) {
    for (entity, collider) in &query {
        for &collided_entity in collider.colliding_entities.iter() {
            //#NOTE: Do nothing when asteroid collides with other asteroids
            if query.get(collided_entity).is_ok() {
                continue;
            }

            //#NOTE: Destroy the asteroid (including its children)
            commands.entity(entity).despawn_recursive();
        }
    }
}