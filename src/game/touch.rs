//! Touch input handling for paddle control via swipe gestures.

use bevy::{input::touch::*, prelude::*};
use std::collections::HashMap;

use super::player::{Player, PlayerSide};
use crate::{AppSystems, PausableSystems, game::movement::MovementController};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        handle_touch_input
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );
}

/// State for tracking active touches
#[derive(Debug, Clone)]
struct TouchState {
    last_position: Vec2,
}

const SWIPE_SENSITIVITY: f32 = 2.0;

/// Handle touch input events for paddle movement
fn handle_touch_input(
    mut touch_events: EventReader<TouchInput>,
    mut active_touches: Local<HashMap<u64, TouchState>>,
    mut controller_query: Query<(&Player, &mut MovementController)>,
) {
    for event in touch_events.read() {
        match event.phase {
            TouchPhase::Started => {
                // Track new touch
                active_touches.insert(
                    event.id,
                    TouchState {
                        last_position: event.position,
                    },
                );
            }
            TouchPhase::Moved => {
                // Calculate swipe delta and update movement
                if let Some(state) = active_touches.get_mut(&event.id) {
                    let delta = event.position - state.last_position;

                    // Apply movement based on vertical swipe delta
                    if delta.y.abs() > 0.1 {
                        // Screen coords increase down, world coords increase up
                        let movement_intent = -delta.y.signum() * SWIPE_SENSITIVITY;

                        // Apply to left paddle only (consistent with keyboard controls)
                        for (player, mut controller) in &mut controller_query {
                            if player.side == PlayerSide::Left {
                                controller.intent.y = movement_intent;
                                break;
                            }
                        }
                    }

                    state.last_position = event.position;
                }
            }
            TouchPhase::Ended | TouchPhase::Canceled => {
                // Stop movement when touch ends
                active_touches.remove(&event.id);

                // If no more active touches, stop the paddle
                if active_touches.is_empty() {
                    for (player, mut controller) in &mut controller_query {
                        if player.side == PlayerSide::Left {
                            controller.intent.y = 0.0;
                            break;
                        }
                    }
                }
            }
        }
    }
}
