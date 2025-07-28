//! Ball entity and physics for Pong-style gameplay.

use avian2d::prelude::*;
use bevy::prelude::*;
use rand::prelude::*;

use super::physics::ball_layers;

// Ball properties
const BALL_RADIUS: f32 = 8.0;
const BALL_SPEED: f32 = 300.0; // pixels per second
const BALL_COLOR: Color = Color::WHITE;
const BALL_FRICTION: f32 = 0.0; // No friction for perfect bounces
const BALL_RESTITUTION: f32 = 1.0; // Perfect elastic collisions
const BALL_Z: f32 = 0.0; // Same layer as paddles

// Serve angles - avoid too steep angles for better gameplay
const MIN_SERVE_ANGLE: f32 = 15.0; // degrees from horizontal
const MAX_SERVE_ANGLE: f32 = 45.0; // degrees from horizontal

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Ball>();
}

/// Marker component for the ball entity
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Ball;

/// Spawns a ball entity at the center of the court
pub fn spawn_ball(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    let ball_mesh = meshes.add(Circle::new(BALL_RADIUS));
    let ball_material = materials.add(ColorMaterial::from_color(BALL_COLOR));

    let ball_entity = commands
        .spawn((
            Name::new("Ball"),
            Ball,
            // Rendering
            Mesh2d(ball_mesh),
            MeshMaterial2d(ball_material),
            Transform::from_xyz(0.0, 0.0, BALL_Z),
            // Physics
            RigidBody::Dynamic,
            Collider::circle(BALL_RADIUS),
            ball_layers(),
            Friction::new(BALL_FRICTION),
            Restitution::new(BALL_RESTITUTION),
            // Prevent rotation for now (can add spin later)
            LockedAxes::ROTATION_LOCKED,
            // Start with zero velocity - will be set by serve_ball
            LinearVelocity::ZERO,
            // Enable transform interpolation for smooth visual movement
            TransformInterpolation,
        ))
        .id();

    // Serve the ball immediately after spawning
    serve_ball(commands, ball_entity);

    ball_entity
}

/// Applies initial velocity to the ball at a random angle
fn serve_ball(commands: &mut Commands, ball_entity: Entity) {
    let mut rng = rand::rng();

    // Random angle within safe range
    let angle_degrees = rng.random_range(MIN_SERVE_ANGLE..=MAX_SERVE_ANGLE);
    
    // Randomly choose up or down
    let angle_sign = if rng.random_bool(0.5) { 1.0 } else { -1.0 };
    
    // Randomly choose left or right
    let direction_x = if rng.random_bool(0.5) { 1.0 } else { -1.0 };

    // Convert to radians and calculate velocity components
    let angle_radians = angle_degrees.to_radians() * angle_sign;
    let velocity_x = angle_radians.cos() * BALL_SPEED * direction_x;
    let velocity_y = angle_radians.sin() * BALL_SPEED;

    // Apply velocity to the ball
    commands
        .entity(ball_entity)
        .insert(LinearVelocity(Vec2::new(velocity_x, velocity_y)));

    info!(
        "Ball served at angle: {:.1}° with velocity: ({:.1}, {:.1})",
        angle_degrees * angle_sign,
        velocity_x,
        velocity_y
    );
}