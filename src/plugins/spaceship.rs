use bevy::prelude::*;

use super::movement::{Acceleration, MovingObjectBundle, Velocity};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

pub struct SpaceShipPlugin;
impl Plugin for SpaceShipPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_entity);
    }
}

// fn spawn_entity(mut commands: Commands) {
//     commands.spawn((
//         SpatialBundle::default(), 
//         Velocity(STARTING_VELOCITY)
//     ));
// }

fn spawn_entity(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(MovingObjectBundle {
        velocity: Velocity::new(STARTING_VELOCITY),
        acceleration: Acceleration::new(Vec3::ZERO),
        model: SceneBundle {
            //#NOTE: Models need to be located under the "assets" folder at the root level, not at the src level
            scene: asset_server.load("Spaceship.glb#Scene0"),
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..default()
        }
    });
}