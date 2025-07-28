//! Pong-style court with boundaries for gameplay.

use avian2d::prelude::*;
use bevy::prelude::*;

use super::{
    physics::{BOUNDARY_FRICTION, BOUNDARY_RESTITUTION, boundary_layers, goal_layers},
    ball::Ball,
    player::PlayerSide,
    scoring::GoalScored,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Court>();
    app.register_type::<Goal>();
}

// Court dimensions
pub const COURT_WIDTH: f32 = 800.0;
pub const COURT_HEIGHT: f32 = 600.0;
// const COURT_PADDING: f32 = 50.0;  // Reserved for future use

// Boundary dimensions
const BOUNDARY_THICKNESS: f32 = 8.0;

// Center line properties
const CENTER_LINE_WIDTH: f32 = 4.0;
const CENTER_LINE_DASH_HEIGHT: f32 = 20.0;
const CENTER_LINE_GAP: f32 = 15.0;

// Goal area dimensions (sensor colliders)
const GOAL_WIDTH: f32 = 50.0;
const GOAL_HEIGHT: f32 = COURT_HEIGHT;

// Colors
const LINE_COLOR: Color = Color::WHITE;
// const COURT_BACKGROUND_COLOR: Color = Color::BLACK;  // Reserved for future use

// Z-ordering
const COURT_Z: f32 = -1.0; // Behind game objects

/// Marker component for the court entity
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Court;

/// Goal area sensor for detecting scoring
#[derive(Component, Debug, Clone, Copy, Reflect)]
#[reflect(Component)]
pub enum Goal {
    Left,
    Right,
}

/// Spawns the complete court with boundaries and center line
pub fn spawn_court(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) -> Entity {
    let line_material = materials.add(LINE_COLOR);

    // Calculate boundary positions
    let half_height = COURT_HEIGHT / 2.0;

    // Create the main court entity
    let court_entity = commands
        .spawn((
            Name::new("Court"),
            Court,
            Transform::from_xyz(0.0, 0.0, COURT_Z),
            Visibility::default(),
        ))
        .id();

    // Spawn top boundary
    let top_boundary = commands
        .spawn((
            Name::new("Top Boundary"),
            RigidBody::Static,
            Collider::rectangle(COURT_WIDTH, BOUNDARY_THICKNESS),
            boundary_layers(),
            // Physics material properties for boundaries
            Friction::new(BOUNDARY_FRICTION),
            Restitution::new(BOUNDARY_RESTITUTION),
            Mesh2d(meshes.add(Rectangle::new(COURT_WIDTH, BOUNDARY_THICKNESS))),
            MeshMaterial2d(line_material.clone()),
            Transform::from_xyz(0.0, half_height - BOUNDARY_THICKNESS / 2.0, 0.0),
        ))
        .id();

    // Spawn bottom boundary
    let bottom_boundary = commands
        .spawn((
            Name::new("Bottom Boundary"),
            RigidBody::Static,
            Collider::rectangle(COURT_WIDTH, BOUNDARY_THICKNESS),
            boundary_layers(),
            // Physics material properties for boundaries
            Friction::new(BOUNDARY_FRICTION),
            Restitution::new(BOUNDARY_RESTITUTION),
            Mesh2d(meshes.add(Rectangle::new(COURT_WIDTH, BOUNDARY_THICKNESS))),
            MeshMaterial2d(line_material.clone()),
            Transform::from_xyz(0.0, -half_height + BOUNDARY_THICKNESS / 2.0, 0.0),
        ))
        .id();

    // Spawn center line
    let center_line = spawn_center_line(commands, meshes, line_material);

    // Spawn goal sensors
    let left_goal = spawn_goal(commands, Goal::Left);
    let right_goal = spawn_goal(commands, Goal::Right);

    // Make boundaries, center line, and goals children of the court
    commands.entity(court_entity).add_children(&[
        top_boundary,
        bottom_boundary,
        center_line,
        left_goal,
        right_goal,
    ]);

    court_entity
}

/// Spawns the decorative center dashed line
fn spawn_center_line(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    material: Handle<ColorMaterial>,
) -> Entity {
    // Calculate the available space between boundaries
    let available_height = COURT_HEIGHT - (BOUNDARY_THICKNESS * 2.0);

    // Calculate how many complete dash+gap patterns we can fit
    // We want to start and end with a gap for better visual balance
    let pattern_height = CENTER_LINE_DASH_HEIGHT + CENTER_LINE_GAP;
    let max_patterns = (available_height - CENTER_LINE_GAP) / pattern_height;
    let dash_count = max_patterns.floor() as i32;

    // Calculate the actual height used by all dashes and gaps
    let used_height = (dash_count as f32 * pattern_height) + CENTER_LINE_GAP;

    // Center the pattern vertically
    let start_y = used_height / 2.0 - CENTER_LINE_GAP - CENTER_LINE_DASH_HEIGHT / 2.0;

    let dash_mesh = meshes.add(Rectangle::new(CENTER_LINE_WIDTH, CENTER_LINE_DASH_HEIGHT));

    // Create center line parent entity
    let center_line_entity = commands
        .spawn((
            Name::new("Center Line"),
            Transform::default(),
            Visibility::default(),
        ))
        .id();

    // Create dash entities
    let mut dash_entities = Vec::new();
    for i in 0..dash_count {
        let y = start_y - (i as f32 * pattern_height);
        let dash_entity = commands
            .spawn((
                Name::new(format!("Dash {i}")),
                Mesh2d(dash_mesh.clone()),
                MeshMaterial2d(material.clone()),
                Transform::from_xyz(0.0, y, 0.0),
            ))
            .id();
        dash_entities.push(dash_entity);
    }

    // Add dashes as children of center line
    commands
        .entity(center_line_entity)
        .add_children(&dash_entities);

    center_line_entity
}

/// Spawns a goal sensor area
fn spawn_goal(commands: &mut Commands, goal: Goal) -> Entity {
    // Position goals just inside the court edges, overlapping with the play area
    let x_position = match goal {
        Goal::Left => -(COURT_WIDTH / 2.0 - GOAL_WIDTH / 2.0),
        Goal::Right => COURT_WIDTH / 2.0 - GOAL_WIDTH / 2.0,
    };

    let goal_side = goal;
    commands
        .spawn((
            Name::new(format!("{goal:?} Goal")),
            goal,
            // Sensor collider - doesn't physically block but detects overlaps
            Sensor,
            Collider::rectangle(GOAL_WIDTH, GOAL_HEIGHT),
            goal_layers(),
            Transform::from_xyz(x_position, 0.0, 0.0),
            // Enable collision events for observer-based detection
            CollisionEventsEnabled,
        ))
        .observe(move |trigger: Trigger<OnCollisionStart>, 
                       mut commands: Commands,
                       ball_query: Query<&Ball>| {
            let other_entity = trigger.event().collider;
            
            // Check if the colliding entity is a ball
            if ball_query.contains(other_entity) {
                // Determine which side scores based on which goal was hit
                let scoring_side = match goal_side {
                    Goal::Left => PlayerSide::Right,  // Ball in left goal = right player scores
                    Goal::Right => PlayerSide::Left,  // Ball in right goal = left player scores
                };
                
                info!("Goal detected! {} scores", match scoring_side {
                    PlayerSide::Left => "Left player",
                    PlayerSide::Right => "Right player",
                });
                commands.trigger(GoalScored { side: scoring_side });
            }
        })
        .id()
}
