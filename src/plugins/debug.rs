use bevy::prelude::*;

use super::schedule::InGameSet;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, print_position.after(InGameSet::EntityUpdate));
    }
}

fn print_position(query: Query<(Entity, &Transform)>) {
    for (entity, transform) in &query {
        println!("Entity: {:?}, Position: {:?}", entity, transform.translation);
    }
}
