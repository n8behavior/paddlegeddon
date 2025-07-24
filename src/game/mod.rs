use bevy::prelude::*;

mod animation;
mod court;
mod debug;
pub mod level;
mod movement;
mod physics;
pub mod player;
mod touch;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        court::plugin,
        debug::plugin,
        level::plugin,
        movement::plugin,
        player::plugin,
        touch::plugin,
    ));
}
