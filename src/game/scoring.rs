//! Scoring system for tracking points and handling goal detection.

use bevy::prelude::*;

use super::{
    ball::{Ball, spawn_ball},
    player::PlayerSide,
};
use crate::screens::Screen;

// Scoring configuration
const MAX_SCORE: u32 = 11; // First to 11 wins
const MERCY_SCORE: u32 = 7; // Mercy rule at 7-0
const SCORE_UI_FONT_SIZE: f32 = 48.0;
const SCORE_UI_Y_OFFSET: f32 = 50.0; // Distance from top (moved above court)
const SCORE_UI_X_OFFSET: f32 = 100.0; // Distance from center

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Score>()
        .register_type::<ScoreDisplay>()
        .init_resource::<Score>()
        .add_event::<GoalScored>()
        .add_systems(OnEnter(Screen::Gameplay), setup_score_ui)
        .add_systems(
            Update,
            update_score_display.run_if(in_state(Screen::Gameplay)),
        )
        .add_observer(handle_goal_and_check_win);
}

/// Event triggered when a goal is scored
#[derive(Event, Reflect)]
pub struct GoalScored {
    pub side: PlayerSide,
}

/// Tracks the current game score
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Score {
    pub left: u32,
    pub right: u32,
}

impl Score {
    /// Returns true if either player has won (normal or mercy)
    pub fn has_winner(&self) -> bool {
        // Normal win: first to 11
        if self.left >= MAX_SCORE || self.right >= MAX_SCORE {
            return true;
        }

        // Mercy win: 7-0
        (self.left >= MERCY_SCORE && self.right == 0)
            || (self.right >= MERCY_SCORE && self.left == 0)
    }

    /// Returns the winning side if there is one
    pub fn winner(&self) -> Option<PlayerSide> {
        // Check normal win
        if self.left >= MAX_SCORE || (self.left >= MERCY_SCORE && self.right == 0) {
            Some(PlayerSide::Left)
        } else if self.right >= MAX_SCORE || (self.right >= MERCY_SCORE && self.left == 0) {
            Some(PlayerSide::Right)
        } else {
            None
        }
    }
}

/// Marker for score display UI elements
#[derive(Component, Reflect)]
#[reflect(Component)]
pub enum ScoreDisplay {
    Left,
    Right,
}

/// Sets up the score UI
fn setup_score_ui(mut commands: Commands) {
    // Left player score
    commands.spawn((
        Name::new("Left Score"),
        ScoreDisplay::Left,
        Text::new("0"),
        TextFont {
            font_size: SCORE_UI_FONT_SIZE,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Px(SCORE_UI_Y_OFFSET),
            margin: UiRect::left(Val::Px(-SCORE_UI_X_OFFSET)),
            ..default()
        },
        StateScoped(Screen::Gameplay),
    ));

    // Right player score
    commands.spawn((
        Name::new("Right Score"),
        ScoreDisplay::Right,
        Text::new("0"),
        TextFont {
            font_size: SCORE_UI_FONT_SIZE,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Px(SCORE_UI_Y_OFFSET),
            margin: UiRect::left(Val::Px(SCORE_UI_X_OFFSET)),
            ..default()
        },
        StateScoped(Screen::Gameplay),
    ));
}

/// Updates the score display UI
fn update_score_display(score: Res<Score>, mut query: Query<(&mut Text, &ScoreDisplay)>) {
    if !score.is_changed() {
        return;
    }

    for (mut text, display) in &mut query {
        match display {
            ScoreDisplay::Left => text.0 = score.left.to_string(),
            ScoreDisplay::Right => text.0 = score.right.to_string(),
        }
    }
}

/// Handles goal scoring events and checks win conditions sequentially
fn handle_goal_and_check_win(
    trigger: Trigger<GoalScored>,
    mut commands: Commands,
    mut score: ResMut<Score>,
    balls: Query<Entity, With<Ball>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut next_state: ResMut<NextState<Screen>>,
) {
    let goal_event = trigger.event();

    // Update score based on which side scored
    match goal_event.side {
        PlayerSide::Left => {
            score.left += 1;
            info!(
                "Left player scores! Score: {} - {}",
                score.left, score.right
            );
        }
        PlayerSide::Right => {
            score.right += 1;
            info!(
                "Right player scores! Score: {} - {}",
                score.left, score.right
            );
        }
    }

    // Check win condition after updating score
    if score.has_winner() {
        let winner = score.winner().unwrap();
        let win_type = if (score.left >= MERCY_SCORE && score.right == 0)
            || (score.right >= MERCY_SCORE && score.left == 0)
        {
            "Mercy win"
        } else {
            "Game win"
        };

        info!(
            "{} for {:?}! Final score: {} - {}",
            win_type, winner, score.left, score.right
        );

        // Despawn all balls before transitioning
        for ball_entity in &balls {
            commands.entity(ball_entity).despawn();
        }

        // TODO: Transition to a win/game over screen
        // For now, just go back to the title screen
        next_state.set(Screen::Title);
    } else {
        // Game continues - despawn old ball and spawn new one
        for ball_entity in &balls {
            commands.entity(ball_entity).despawn();
        }

        spawn_ball(&mut commands, &mut meshes, &mut materials);
    }
}
