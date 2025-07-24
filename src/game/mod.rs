use bevy::prelude::*;

mod animation;
mod court;
pub mod level;
mod movement;
pub mod player;
mod touch;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        court::plugin,
        level::plugin,
        movement::plugin,
        player::plugin,
        touch::plugin,
    ));
}
