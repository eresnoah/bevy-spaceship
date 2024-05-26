use bevy::prelude::*;

use crate::schedule::InGameSet;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_position.after(InGameSet::EntityUpdates));
    }
}

fn print_position(query: Query<(Entity, &Transform)>) {
    // Log the entity ID and position of each entity with a "Position" component
    for (entity, transform) in query.iter() {
        info!("Entity {:?} is at position {:?},", entity, transform);
    }
}