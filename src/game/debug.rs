//! Debug utilities for development builds.

use avian2d::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // Only add debug systems in development builds
    #[cfg(feature = "dev")]
    app.add_systems(Update, warn_default_collision_layers);
}

/// Warns when entities have colliders but are using the default collision layer.
/// This helps catch entities that should have explicit collision behavior.
#[cfg(feature = "dev")]
fn warn_default_collision_layers(
    query: Query<(Entity, &Name, &CollisionLayers), Added<CollisionLayers>>,
) {
    for (entity, name, layers) in &query {
        // For now, we'll just check if the entity has the default_layers() configuration
        // which has empty filters (doesn't collide with anything)
        if layers == &super::physics::default_layers() {
            warn!(
                "Entity {:?} ({}) spawned with Default collision layer. \
                If this entity should participate in collisions, \
                assign it an explicit game layer (Paddle, Ball, Boundary, etc.)",
                entity,
                name.as_str()
            );
        }
    }
}
