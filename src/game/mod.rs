use bevy::prelude::*;

mod animation;
pub mod ball;
mod court;
mod debug;
pub mod level;
mod physics;
pub mod player;
mod scoring;

pub(super) fn plugin(app: &mut App) {
    // Initialize game phase state
    app.init_state::<GamePhase>();
    
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
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
#[states(scoped_entities)]
pub enum GamePhase {
    #[default]
    WaitingToServe,  // Waiting for player to press space
    Playing,         // Ball is in play
    GoalScored,      // Brief pause after goal
    GameOver,        // Show winner, wait for input
}
