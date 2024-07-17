use bevy::prelude::*;

use super::{asset_loader::SceneAssets, collision_detection::Collider, movement::{Acceleration, MovingObjectBundle, Velocity}};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const SPACESHIP_RADIOUS: f32 = 5.0; 
const MISSILE_SPAWN_FORWARD_SCALAR: f32 = 7.5;
const MISSILE_SPEED: f32 = 50.0;
const MISSILE_RADIOUS:f32 = 1.0;

pub struct SpaceShipPlugin;
impl Plugin for SpaceShipPlugin {
    fn build(&self, app: &mut App) {
        app
        //#NOTE: Need to be run after asset_loader plugin which is loaded at startup
        .add_systems(PostStartup, spawn_entity)
        .add_systems(Update, (spaceship_movement_controls,spaceship_weapon_controls));
    }
}

#[derive(Component, Debug)]
pub struct Spaceship;    //TAG

#[derive(Component, Debug)]
pub struct SpaceshipMissile;    //TAG


// fn spawn_entity(mut commands: Commands) {
//     commands.spawn((
//         SpatialBundle::default(), 
//         Velocity(STARTING_VELOCITY)
//     ));
// }

fn spawn_entity(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((MovingObjectBundle {
        velocity: Velocity::new(Vec3::ZERO),
        acceleration: Acceleration::new(Vec3::ZERO),
        collider: Collider::new(SPACESHIP_RADIOUS),
        model: SceneBundle {
            //#NOTE: Models need to be located under the "assets" folder at the root level, not at the src level
            scene: scene_assets.spaceship.clone(),
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..default()
        }
    }, Spaceship));
}

fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>, 
    //#NOTE: Bevy v0.12 Input => v0.13 ButtonInput
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = query.single_mut();
    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    //#NOTE: Bevy v0.12 KeyCode::D => v0.13 KeyCode::KeyD
    if keyboard_input.pressed(KeyCode::KeyD) {
        rotation = -SPACESHIP_ROTATION_SPEED * time.delta_seconds();

    //#NOTE: Bevy v0.12 KeyCode::A => v0.13 KeyCode::KeyA
    } else if keyboard_input.pressed(KeyCode::KeyA) {
        rotation = SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    }

    //#NOTE: Bevy v0.12 KeyCode::W => v0.13 KeyCode::KeyW
    if keyboard_input.pressed(KeyCode::KeyW) {
        movement = SPACESHIP_SPEED;

    //#NOTE: Bevy v0.12 KeyCode::S => v0.13 KeyCode::KeyS
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        movement = -SPACESHIP_SPEED;
    }

    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        roll = -SPACESHIP_ROLL_SPEED * time.delta_seconds();

    } else if keyboard_input.pressed(KeyCode::ControlLeft) {
        roll = SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }

    //#NOTE: Rotate around y axis, and ignore z axis rotation
    transform.rotate_y(rotation);

    //#NOTE: Rotate around the local z axis, the rotation is relative to the previous rotation (So that we can lock the spaceship on y axis)
    transform.rotate_local_z(roll);

    //#NOTE: It seems bevy's forward direction is opposite to the common model, so will need to use negative here
    velocity.value = -transform.forward() * movement;
}

fn spaceship_weapon_controls(
    mut commands: Commands, 
    query: Query<&Transform, With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    scene_assets: Res<SceneAssets>,
) {
    let spaceship_transform = query.single();
    if keyboard_input.pressed(KeyCode::Space) {
        commands.spawn((MovingObjectBundle {
            //#NOTE: It seems bevy's forward direction is opposite to the common model, so will need to use negative here
            velocity: Velocity::new(-spaceship_transform.forward() * MISSILE_SPEED),
            acceleration: Acceleration::new(Vec3::ZERO),
            collider: Collider::new(MISSILE_RADIOUS),
            model: SceneBundle {
                scene: scene_assets.missiles.clone(),
                transform: Transform::from_translation(spaceship_transform.translation + -spaceship_transform.forward() * MISSILE_SPAWN_FORWARD_SCALAR),
                ..default()
            }
        }, SpaceshipMissile));
    }
}