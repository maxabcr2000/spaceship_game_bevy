use std::collections::HashMap;
use bevy::prelude::*;
use crate::plugins::{asteroid::Asteroid, schedule::InGameSet, spaceship::Spaceship};

/*#NOTE: 
This plugin is just for practicing purpose, it would be better to be replaced by other third-party plugins for physics
like: bevy-rapier(https://github.com/dimforge/bevy_rapier) or bevy-xpbd(https://github.com/Jondolf/avian)
*/

pub struct CollisionDetectionPlugin;
impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, collision_detection.in_set(InGameSet::CollisionDetection))
        .add_systems(Update,
            (
            handle_collision::<Asteroid>,
            handle_collision::<Spaceship>, 
            ).in_set(InGameSet::DespawnEntities),
        );
    }
}


#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
} 

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entities: vec![],
        }
    }
}

//#NOTE: We need GlobalTransform instead of Transform (the one related to the parent) to be able to detect collisions in space
fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>){
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    //#NOTE: Detect collisions
    for (entity_a, transform_a, collider_a) in &query {
        for (entity_b, transform_b, collider_b) in &query {
            if entity_a != entity_b {
                let distance = transform_a.translation().distance(transform_b.translation());
                if distance < collider_a.radius + collider_b.radius {
                    colliding_entities.entry(entity_a).or_insert_with(Vec::new).push(entity_b);
                }
            }    
        }
    }

    //#NOTE: Update collider component
    for (entity, _, mut collider) in &mut query {
        collider.colliding_entities.clear();

        if let Some(collisions) = colliding_entities.get(&entity) {
            collider.colliding_entities = collisions.clone();
        }
    }
}

//#NOTE: System that support Generic Component
fn handle_collision<T: Component>(
    mut commands: Commands,
    query: Query<(Entity, &Collider), With<T>>, 
) {
    for (entity, collider) in &query {
        for &collided_entity in collider.colliding_entities.iter() {
            //#NOTE: Do nothing when same type of entity collides with other
            if query.get(collided_entity).is_ok() {
                continue;
            }

            //#NOTE: Destroy the entity  (including its children)
            commands.entity(entity).despawn_recursive();
        }
    }
}