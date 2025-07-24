//! Touch input handling for paddle control via swipe gestures.

use bevy::{input::touch::*, prelude::*};

use super::player::Player;
use crate::{AppSystems, PausableSystems, game::movement::MovementController};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<TouchGesture>();

    app.add_systems(
        Update,
        detect_swipe_gestures
            .run_if(has_touch_input)
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );
}

/// Component to track active touch gestures
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TouchGesture {
    touch_id: u64,
    start_position: Vec2,
    start_time: f32,
    last_position: Vec2,
}

// Reserved for future use when implementing discrete swipe gestures
// const MIN_SWIPE_DISTANCE: f32 = 20.0;
const SWIPE_SENSITIVITY: f32 = 2.0;

/// Run condition that checks if there are any active touches
fn has_touch_input(touches: Res<Touches>) -> bool {
    touches.iter().next().is_some()
}

fn detect_swipe_gestures(
    touches: Res<Touches>,
    time: Res<Time>,
    mut commands: Commands,
    mut gesture_query: Query<(Entity, &mut TouchGesture)>,
    mut controller_query: Query<&mut MovementController, With<Player>>,
) {
    // Handle new touches
    for touch in touches.iter_just_pressed() {
        commands.spawn((
            TouchGesture {
                touch_id: touch.id(),
                start_position: touch.position(),
                start_time: time.elapsed_secs(),
                last_position: touch.position(),
            },
            Name::new("Touch Gesture"),
        ));
    }

    // Update existing touches and apply movement
    for (_entity, mut gesture) in &mut gesture_query {
        if let Some(touch) = touches.get_pressed(gesture.touch_id) {
            let current_position = touch.position();
            let delta = current_position - gesture.last_position;

            // Apply movement based on vertical swipe delta
            if delta.y.abs() > 0.1 {
                // Screen coords increase down, world increase up
                let movement_intent = -delta.y.signum() * SWIPE_SENSITIVITY;

                // Apply to all players (in future, could be player-specific based on touch location)
                for mut controller in &mut controller_query {
                    controller.intent.y = movement_intent;
                }
            }

            gesture.last_position = current_position;
        }
    }

    // Clean up released touches
    for touch in touches.iter_just_released() {
        for (entity, gesture) in &gesture_query {
            if gesture.touch_id == touch.id() {
                // Stop movement when touch is released
                for mut controller in &mut controller_query {
                    controller.intent.y = 0.0;
                }
                commands.entity(entity).despawn();
            }
        }
    }

    // Clean up canceled touches
    for touch in touches.iter_just_canceled() {
        for (entity, gesture) in &gesture_query {
            if gesture.touch_id == touch.id() {
                // Stop movement when touch is canceled
                for mut controller in &mut controller_query {
                    controller.intent.y = 0.0;
                }
                commands.entity(entity).despawn();
            }
        }
    }
}
