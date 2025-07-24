//! Physics configuration and collision layers for the game.

use avian2d::prelude::*;

// Boundary physics properties
/// Medium friction for boundaries - allows some sliding along walls
pub const BOUNDARY_FRICTION: f32 = 0.5;
/// High restitution for boundaries - creates bouncy walls
pub const BOUNDARY_RESTITUTION: f32 = 0.8;

// Paddle physics properties
/// Low friction for paddles - allows smooth movement
pub const PADDLE_FRICTION: f32 = 0.1;
/// Zero restitution for paddles - paddles absorb impact
pub const PADDLE_RESTITUTION: f32 = 0.0;
/// Default maximum speed for paddle movement (pixels per second)
pub const PADDLE_MAX_SPEED: f32 = 400.0;

/// Collision layers for different game entities.
/// Each variant automatically gets assigned to a layer (0, 1, 2, etc.)
///
/// ## Design Decision: Non-Colliding Default Layer
/// 
/// The `Default` layer is intentionally set to not collide with anything. This provides:
/// - **Safety**: UI elements, debug visualizations, or other non-game entities won't interfere
/// - **Explicit intent**: Forces conscious assignment of collision behavior
/// - **Performance**: Reduces unnecessary collision checks
///
/// If an entity needs to participate in collisions, it MUST be explicitly assigned
/// a game-specific layer (Paddle, Ball, Boundary, etc.). This prevents accidental
/// gameplay interference from forgotten or misconfigured entities.
///
/// ## Collision Matrix
/// ```text
///          | Default | Paddle | Ball | Boundary | Goal | PowerUp |
/// ---------|---------|--------|------|----------|------|---------|  
/// Default  |   ❌    |   ❌   |  ❌  |    ❌    |  ❌  |   ❌    |
/// Paddle   |   ❌    |   ❌   |  ✅  |    ✅    |  ❌  |   ❌    |
/// Ball     |   ❌    |   ✅   |  ❌  |    ✅    |  ✅  |   ✅    |
/// Boundary |   ❌    |   ✅   |  ✅  |    ❌    |  ❌  |   ❌    |
/// Goal     |   ❌    |   ❌   |  ✅  |    ❌    |  ❌  |   ❌    |
/// PowerUp  |   ❌    |   ❌   |  ✅  |    ❌    |  ❌  |   ❌    |
/// ```
#[derive(PhysicsLayer, Clone, Copy, Debug, Default)]
pub enum GameLayer {
    #[default]
    Default,  // Layer 0 - Unassigned/neutral entities
    Paddle,   // Layer 1
    Ball,     // Layer 2  
    Boundary, // Layer 3
    PowerUp,  // Layer 4
    Goal,     // Layer 5
}

/// Creates collision layers for paddles.
/// Paddles collide with balls and boundaries, but not with other paddles or powerups.
pub fn paddle_layers() -> CollisionLayers {
    CollisionLayers::new(
        GameLayer::Paddle,
        [GameLayer::Ball, GameLayer::Boundary],
    )
}

/// Creates collision layers for the ball.
/// The ball collides with everything.
pub fn ball_layers() -> CollisionLayers {
    CollisionLayers::new(
        GameLayer::Ball,
        [GameLayer::Paddle, GameLayer::Boundary, GameLayer::Goal, GameLayer::PowerUp],
    )
}

/// Creates collision layers for boundaries.
/// Boundaries collide with paddles and balls.
pub fn boundary_layers() -> CollisionLayers {
    CollisionLayers::new(
        GameLayer::Boundary,
        [GameLayer::Paddle, GameLayer::Ball],
    )
}

/// Creates collision layers for powerups.
/// Powerups only collide with the ball (for collection).
pub fn powerup_layers() -> CollisionLayers {
    CollisionLayers::new(
        GameLayer::PowerUp,
        [GameLayer::Ball],
    )
}

/// Creates collision layers for goal sensors.
/// Goals only detect balls passing through them.
pub fn goal_layers() -> CollisionLayers {
    CollisionLayers::new(
        GameLayer::Goal,
        [GameLayer::Ball],
    )
}

/// Creates collision layers for default/unassigned entities.
/// Default entities don't collide with anything.
/// 
/// WARNING: This is intentional! Entities should be explicitly assigned
/// a game-specific layer to participate in collisions. See the GameLayer
/// documentation for the rationale behind this design decision.
pub fn default_layers() -> CollisionLayers {
    CollisionLayers::new(
        GameLayer::Default,
        [] as [GameLayer; 0],
    )
}
