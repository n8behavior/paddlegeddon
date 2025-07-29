//! Ball entity and physics for Pong-style gameplay.

use avian2d::prelude::*;
use bevy::prelude::*;
use rand::prelude::*;

use super::{
    physics::ball_layers,
    player::PlayerSide,
    GamePhase,
};
use crate::screens::Screen;

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
    app.register_type::<Ball>()
        .register_type::<ServeDirection>()
        .init_resource::<ServeDirection>()
        .add_systems(
            Update,
            handle_serve_input.run_if(in_state(GamePhase::WaitingToServe).and(in_state(Screen::Gameplay))),
        )
        .add_systems(
            OnEnter(GamePhase::WaitingToServe), 
            setup_serve_ui.run_if(in_state(Screen::Gameplay))
        )
        .add_systems(
            OnExit(GamePhase::WaitingToServe),
            despawn_serve_ui
        )
        .add_systems(
            OnEnter(GamePhase::Playing),
            serve_on_play_start
        );
}

/// Marker component for the ball entity
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Ball;

/// Marker component for serve UI elements
#[derive(Component)]
pub struct ServeUI;

/// Tracks which player should serve next
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct ServeDirection {
    pub side: PlayerSide,
}


/// Spawns a ball entity at the center of the court (without serving)
pub(super) fn spawn_ball(
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
            // Start with zero velocity - will be served later
            LinearVelocity::ZERO,
            // Disable gravity for top-down view
            GravityScale(0.0),
            // Enable transform interpolation for smooth visual movement
            TransformInterpolation,
            // Enable collision events for goal detection
            CollisionEventsEnabled,
        ))
        .id();
    
    // Add damping components separately to avoid tuple size limit
    commands.entity(ball_entity)
        .insert((
            LinearDamping(0.0),
            AngularDamping(0.0),
            StateScoped(Screen::Gameplay),
        ));
    
    ball_entity
}

/// Applies initial velocity to the ball based on serve direction
pub(super) fn serve_ball(
    commands: &mut Commands,
    ball_entity: Entity,
    serve_direction: &ServeDirection,
) {
    let mut rng = rand::rng();

    // Random angle within safe range
    let angle_degrees = rng.random_range(MIN_SERVE_ANGLE..=MAX_SERVE_ANGLE);

    // Randomly choose up or down
    let angle_sign = if rng.random_bool(0.5) { 1.0 } else { -1.0 };

    // Determine serve direction based on which player is serving
    let direction_x = match serve_direction.side {
        PlayerSide::Left => 1.0,   // Left player serves to the right
        PlayerSide::Right => -1.0, // Right player serves to the left
    };

    // Convert to radians and calculate velocity components
    let angle_radians = angle_degrees.to_radians() * angle_sign;
    let velocity_x = angle_radians.cos() * BALL_SPEED * direction_x;
    let velocity_y = angle_radians.sin() * BALL_SPEED;

    // Apply velocity to the ball
    commands
        .entity(ball_entity)
        .insert(LinearVelocity(Vec2::new(velocity_x, velocity_y)));

    info!(
        "{} player served at angle: {:.1}° with velocity: ({:.1}, {:.1})",
        match serve_direction.side {
            PlayerSide::Left => "Left",
            PlayerSide::Right => "Right",
        },
        angle_degrees * angle_sign,
        velocity_x,
        velocity_y
    );
}

/// Handles space bar input to transition from WaitingToServe to Playing
fn handle_serve_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_phase: ResMut<NextState<GamePhase>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        // Transition to playing phase - ball will be served on state entry
        game_phase.set(GamePhase::Playing);
    }
}


/// Sets up the serve UI
fn setup_serve_ui(
    mut commands: Commands,
    serve_direction: Res<ServeDirection>,
) {
    // Main container
    commands.spawn((
        Name::new("Serve UI"),
        ServeUI,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::NONE),
    ))
    .with_children(|parent| {
        // Serve direction indicator
        parent.spawn((
            Text::new(format!(
                "{} player to serve",
                match serve_direction.side {
                    PlayerSide::Left => "Left",
                    PlayerSide::Right => "Right",
                }
            )),
            TextFont {
                font_size: 32.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
        
        // Instructions
        parent.spawn((
            Text::new("Press SPACE to serve"),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.8, 0.8)),
        ));
    });
}

/// Despawns all serve UI elements when transitioning away from WaitingToServe
fn despawn_serve_ui(
    mut commands: Commands,
    serve_ui_query: Query<Entity, With<ServeUI>>,
) {
    for entity in &serve_ui_query {
        commands.entity(entity).despawn();
    }
}

/// Serves the ball when entering the Playing state
fn serve_on_play_start(
    mut commands: Commands,
    balls: Query<Entity, With<Ball>>,
    serve_direction: Res<ServeDirection>,
) {
    // Find the ball and serve it
    for ball_entity in &balls {
        serve_ball(&mut commands, ball_entity, &serve_direction);
    }
}
