use bevy::prelude::*;

mod animation;
mod court;
mod debug;
pub mod level;
mod physics;
pub mod player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        court::plugin,
        debug::plugin,
        level::plugin,
        player::plugin,
    ));
}
