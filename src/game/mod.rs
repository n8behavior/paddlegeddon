use bevy::prelude::*;

mod animation;
pub mod ball;
mod court;
mod debug;
pub mod level;
mod physics;
pub mod player;
mod scoring;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    // Add GamePhase as a sub-state of Screen::Gameplay
    app.add_sub_state::<GamePhase>();
    
    app.add_plugins((
        animation::plugin,
        ball::plugin,
        court::plugin,
        debug::plugin,
        level::plugin,
        player::plugin,
        scoring::plugin,
    ));
}

/// Sub-states for different phases of gameplay
#[derive(SubStates, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
#[source(Screen = Screen::Gameplay)]
pub enum GamePhase {
    #[default]
    WaitingToServe,  // Waiting for player to press space
    Playing,         // Ball is in play
    GoalScored,      // Brief pause after goal
    GameOver,        // Show winner, wait for input
}
