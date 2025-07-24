//! Pong-style court with boundaries for gameplay.

use bevy::prelude::*;

use super::collision::Collidable;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Court>();
    app.register_type::<CourtBoundary>();
}

// Court dimensions
const COURT_SIZE: Vec2 = Vec2::new(800.0, 600.0);
// const COURT_PADDING: f32 = 50.0;  // Reserved for future use

// Boundary line dimensions (width x height)
const BOUNDARY_SIZE: Vec2 = Vec2::new(COURT_SIZE.x, 8.0);

// Center line properties
const CENTER_LINE_SIZE: Vec2 = Vec2::new(4.0, 20.0);
const CENTER_LINE_GAP: f32 = 15.0;

// Colors
const LINE_COLOR: Color = Color::WHITE;
// const COURT_BACKGROUND_COLOR: Color = Color::BLACK;  // Reserved for future use

// Z-ordering
const COURT_Z: f32 = -1.0;  // Behind game objects

/// Marker component for the court entity
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Court;

/// Identifies which boundary this is for collision handling
#[derive(Component, Reflect)]
#[reflect(Component)]
pub enum CourtBoundary {
    Top,
    Bottom,
}


/// Spawns the complete court with boundaries and center line
pub fn spawn_court(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) -> Entity {
    let line_material = materials.add(LINE_COLOR);
    
    // Calculate boundary positions
    let half_height = COURT_SIZE.y / 2.0;
    
    // Create the main court entity
    let court_entity = commands.spawn((
        Name::new("Court"),
        Court,
        Transform::from_xyz(0.0, 0.0, COURT_Z),
        Visibility::default(),
    )).id();
    
    // Spawn top boundary
    let top_boundary = commands.spawn((
        Name::new("Top Boundary"),
        CourtBoundary::Top,
        Collidable::from_vec2(BOUNDARY_SIZE),
        Mesh2d(meshes.add(Rectangle::new(BOUNDARY_SIZE.x, BOUNDARY_SIZE.y))),
        MeshMaterial2d(line_material.clone()),
        Transform::from_xyz(0.0, half_height - BOUNDARY_SIZE.y / 2.0, 0.0),
    )).id();
    
    // Spawn bottom boundary
    let bottom_boundary = commands.spawn((
        Name::new("Bottom Boundary"),
        CourtBoundary::Bottom,
        Collidable::from_vec2(BOUNDARY_SIZE),
        Mesh2d(meshes.add(Rectangle::new(BOUNDARY_SIZE.x, BOUNDARY_SIZE.y))),
        MeshMaterial2d(line_material.clone()),
        Transform::from_xyz(0.0, -half_height + BOUNDARY_SIZE.y / 2.0, 0.0),
    )).id();
    
    // Spawn center line
    let center_line = spawn_center_line(commands, meshes, line_material);
    
    // Make boundaries and center line children of the court
    commands.entity(court_entity).add_children(&[top_boundary, bottom_boundary, center_line]);
    
    court_entity
}

/// Spawns the decorative center dashed line
fn spawn_center_line(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    material: Handle<ColorMaterial>,
) -> Entity {
    // Calculate the available space between boundaries
    let available_height = COURT_SIZE.y - (BOUNDARY_SIZE.y * 2.0);
    
    // Calculate how many complete dash+gap patterns we can fit
    // We want to start and end with a gap for better visual balance
    let pattern_height = CENTER_LINE_SIZE.y + CENTER_LINE_GAP;
    let max_patterns = (available_height - CENTER_LINE_GAP) / pattern_height;
    let dash_count = max_patterns.floor() as i32;
    
    // Calculate the actual height used by all dashes and gaps
    let used_height = (dash_count as f32 * pattern_height) + CENTER_LINE_GAP;
    
    // Center the pattern vertically
    let start_y = used_height / 2.0 - CENTER_LINE_GAP - CENTER_LINE_SIZE.y / 2.0;
    
    let dash_mesh = meshes.add(Rectangle::new(CENTER_LINE_SIZE.x, CENTER_LINE_SIZE.y));
    
    // Create center line parent entity
    let center_line_entity = commands.spawn((
        Name::new("Center Line"),
        Transform::default(),
        Visibility::default(),
    )).id();
    
    // Create dash entities
    let mut dash_entities = Vec::new();
    for i in 0..dash_count {
        let y = start_y - (i as f32 * pattern_height);
        let dash_entity = commands.spawn((
            Name::new(format!("Dash {}", i)),
            Mesh2d(dash_mesh.clone()),
            MeshMaterial2d(material.clone()),
            Transform::from_xyz(0.0, y, 0.0),
        )).id();
        dash_entities.push(dash_entity);
    }
    
    // Add dashes as children of center line
    commands.entity(center_line_entity).add_children(&dash_entities);
    
    center_line_entity
}