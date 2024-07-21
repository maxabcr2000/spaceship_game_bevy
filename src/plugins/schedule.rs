use bevy::prelude::*;

#[derive(Debug, SystemSet, Clone, PartialEq, Eq, Hash)]
pub enum InGameSet {
    UserInput,
    EntityUpdate,
    CollisionDetection,
    DespawnEntities,
}

pub struct SchedulePlugin;
impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app
        .configure_sets(
            Update,
            (
                InGameSet::DespawnEntities,
                InGameSet::UserInput,
                InGameSet::EntityUpdate,
                InGameSet::CollisionDetection,
            ).chain(),
        );

        //#NOTE: Bevy 0.13 update: apply_deferred is now automatically applied when we use ordering in systems that have commands.

        // .add_systems(Update, 
        //     //#NOTE: Manually add a flush point between InGameSet::DespawnEntities and InGameSet::UserInput system set
        //     apply_deferred
        //     .after(InGameSet::DespawnEntities)
        //     .before(InGameSet::UserInput),
        // );
    }
}