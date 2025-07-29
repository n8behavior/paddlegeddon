//! Scoring system for tracking points and handling goal detection.

use bevy::prelude::*;

use super::{
    GamePhase,
    ball::{Ball, ServeDirection, spawn_ball},
    player::PlayerSide,
};
use crate::screens::Screen;

// Scoring configuration
const MAX_SCORE: u32 = 11; // First to 11 wins
const MERCY_SCORE: u32 = 7; // Mercy rule at 7-0
const SCORE_UI_FONT_SIZE: f32 = 48.0;
const SCORE_UI_Y_OFFSET: f32 = 50.0; // Distance from top (moved above court)
const SCORE_UI_X_OFFSET: f32 = 100.0; // Distance from center

// Goal scored pause duration
const GOAL_PAUSE_DURATION: f32 = 1.0; // 1 second pause after goal

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Score>()
        .register_type::<ScoreDisplay>()
        .register_type::<GoalTimer>()
        .init_resource::<Score>()
        .init_resource::<GoalTimer>()
        .add_event::<GoalScored>()
        .add_systems(OnEnter(Screen::Gameplay), setup_score_ui)
        .add_systems(
            Update,
            (
                update_score_display.run_if(in_state(Screen::Gameplay)),
                handle_goal_pause.run_if(in_state(GamePhase::GoalScored)),
                handle_game_over_input.run_if(in_state(GamePhase::GameOver)),
            ),
        )
        .add_systems(OnEnter(GamePhase::GameOver), setup_game_over_screen)
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

/// Timer for goal scored pause
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct GoalTimer {
    pub timer: Timer,
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
    mut serve_direction: ResMut<ServeDirection>,
    mut game_phase: ResMut<NextState<GamePhase>>,
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

    // Despawn all balls
    for ball_entity in &balls {
        commands.entity(ball_entity).despawn();
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

        // Transition to game over state
        game_phase.set(GamePhase::GameOver);
    } else {
        // Game continues - set up next serve
        // The player who was scored on gets to serve
        serve_direction.side = match goal_event.side {
            PlayerSide::Left => PlayerSide::Right, // Left scored, so right serves
            PlayerSide::Right => PlayerSide::Left, // Right scored, so left serves
        };

        // Spawn new ball (without serving)
        spawn_ball(&mut commands, &mut meshes, &mut materials);

        // Transition to goal scored state
        game_phase.set(GamePhase::GoalScored);

        // Start the goal timer
        commands.insert_resource(GoalTimer {
            timer: Timer::from_seconds(GOAL_PAUSE_DURATION, TimerMode::Once),
        });
    }
}

/// Handles the pause after a goal is scored
fn handle_goal_pause(
    time: Res<Time>,
    mut goal_timer: ResMut<GoalTimer>,
    mut game_phase: ResMut<NextState<GamePhase>>,
) {
    goal_timer.timer.tick(time.delta());

    if goal_timer.timer.finished() {
        // Transition to waiting for serve
        game_phase.set(GamePhase::WaitingToServe);
    }
}

/// Sets up the game over screen
fn setup_game_over_screen(mut commands: Commands, score: Res<Score>) {
    let winner = score.winner().expect("Game over without winner");
    let win_type = if (score.left >= MERCY_SCORE && score.right == 0)
        || (score.right >= MERCY_SCORE && score.left == 0)
    {
        "MERCY WIN!"
    } else {
        "VICTORY!"
    };

    // Game over overlay
    commands
        .spawn((
            Name::new("Game Over Overlay"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            StateScoped(GamePhase::GameOver),
        ))
        .with_children(|parent| {
            // Win type text
            parent.spawn((
                Text::new(win_type),
                TextFont {
                    font_size: 72.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Winner text
            parent.spawn((
                Text::new(format!(
                    "{} Player Wins!",
                    match winner {
                        PlayerSide::Left => "Left",
                        PlayerSide::Right => "Right",
                    }
                )),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Final score
            parent.spawn((
                Text::new(format!("Final Score: {} - {}", score.left, score.right)),
                TextFont {
                    font_size: 36.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Instructions
            parent.spawn((
                Text::new("Press SPACE to play again or ESC for menu"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
        });
}

/// Handles input on the game over screen
fn handle_game_over_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut score: ResMut<Score>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        // Reset score and play again
        score.left = 0;
        score.right = 0;
        next_screen.set(Screen::Gameplay);
    } else if keyboard.just_pressed(KeyCode::Escape) {
        // Return to title screen
        score.left = 0;
        score.right = 0;
        next_screen.set(Screen::Title);
    }
}
