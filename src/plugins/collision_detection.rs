use std::collections::HashMap;

use bevy::prelude::*;

/*#NOTE: 
This plugin is just for practicing purpose, it would be better to be replaced by other third-party plugins for physics
like: bevy-rapier(https://github.com/dimforge/bevy_rapier) or bevy-xpbd(https://github.com/Jondolf/avian)
*/

pub struct CollisionDetectionPlugin;
impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, collision_detection);
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