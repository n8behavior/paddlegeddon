//! Player-specific behavior.

use avian2d::prelude::*;
use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::{
    AppSystems, PausableSystems,
    asset_tracking::LoadResource,
    game::court::COURT_HEIGHT,
    game::movement::MovementController,
    game::physics::{PADDLE_FRICTION, PADDLE_MAX_SPEED, PADDLE_RESTITUTION, paddle_layers},
};

// Paddle dimensions (relative to court size)
const PADDLE_HEIGHT_RATIO: f32 = 0.125; // 1/8 of court height
const PADDLE_WIDTH: f32 = 12.0;

// Paddle positioning
pub const PADDLE_X_OFFSET: f32 = 350.0; // Distance from center

pub(super) fn plugin(app: &mut App) {
    app.register_type::<PlayerSide>();
    app.register_type::<Player>();

    app.register_type::<PlayerAssets>();
    app.load_resource::<PlayerAssets>();

    // Player movement systems
    app.add_systems(
        Update,
        (
            move_player.run_if(player_input),
            stop_player_movement.run_if(player_input_stopped),
        )
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );
}

#[derive(Reflect, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlayerSide {
    #[default]
    Left,
    Right,
}

/// Default paddle speed when not specified
pub const DEFAULT_PADDLE_SPEED: f32 = PADDLE_MAX_SPEED;

/// The player character.
pub fn player(
    side: PlayerSide,
    position: Vec3,
    max_speed: f32,
    _player_assets: &PlayerAssets,
    _texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    // A texture atlas is a way to split a single image into a grid of related images.
    // You can learn more in this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    //let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
    //let texture_atlas_layout = texture_atlas_layouts.add(layout);
    //let player_animation = PlayerAnimation::new();

    let paddle_height = COURT_HEIGHT * PADDLE_HEIGHT_RATIO;

    (
        Name::new("Player"),
        Player { side },
        Sprite {
            // Starts with Pong-style paddles that morph later
            //image: player_assets.ducky.clone(),
            //texture_atlas: Some(TextureAtlas {
            //    layout: texture_atlas_layout,
            //    index: player_animation.get_atlas_index(),
            //}),
            color: Color::WHITE,
            custom_size: Some(Vec2::new(PADDLE_WIDTH, paddle_height)),
            ..default()
        },
        Transform::from_translation(position),
        MovementController {
            max_speed,
            ..default()
        },
        // Physics components for paddle
        RigidBody::Dynamic,
        Collider::rectangle(PADDLE_WIDTH, paddle_height),
        paddle_layers(),
        LinearVelocity::default(),
        // Lock rotation and horizontal movement
        LockedAxes::new().lock_rotation().lock_translation_x(),
        // Prevent gravity from affecting the paddle
        GravityScale(0.0),
        // High linear damping to stop quickly when no input
        LinearDamping(10.0),
        // Physics material properties for paddles
        Friction::new(PADDLE_FRICTION),
        Restitution::new(PADDLE_RESTITUTION),
        //player_animation,
    )
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player {
    pub side: PlayerSide,
}

/// Run condition that checks if there's any player movement input
fn player_input(keyboard: Res<ButtonInput<KeyCode>>) -> bool {
    keyboard.any_pressed([
        KeyCode::KeyW,
        KeyCode::KeyS,
        KeyCode::ArrowUp,
        KeyCode::ArrowDown,
    ])
}

/// Run condition that checks if player movement input was just released
fn player_input_stopped(keyboard: Res<ButtonInput<KeyCode>>) -> bool {
    keyboard.any_just_released([
        KeyCode::KeyW,
        KeyCode::KeyS,
        KeyCode::ArrowUp,
        KeyCode::ArrowDown,
    ])
}

/// Apply movement to player based on keyboard input
fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<(&Player, &mut MovementController)>,
) {
    // Apply movement intent only to left paddle.
    for (player, mut controller) in &mut controller_query {
        if player.side == PlayerSide::Left {
            // Collect directional input.
            let mut intent = Vec2::ZERO;
            if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
                intent.y += 1.0;
            }
            if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
                intent.y -= 1.0;
            }

            // Normalize intent so that diagonal movement is the same speed as horizontal / vertical.
            // This should be omitted if the input comes from an analog stick instead.
            controller.intent = intent.normalize_or_zero();

            break; // Only one left paddle, so we can exit early
        }
    }
}

/// Stop player movement when input is released
fn stop_player_movement(mut controller_query: Query<(&Player, &mut MovementController)>) {
    // Clear movement intent for left paddle
    for (player, mut controller) in &mut controller_query {
        if player.side == PlayerSide::Left {
            controller.intent = Vec2::ZERO;
            break; // Only one left paddle, so we can exit early
        }
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    ducky: Handle<Image>,
    #[dependency]
    pub steps: Vec<Handle<AudioSource>>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            ducky: assets.load_with_settings(
                "images/ducky.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            steps: vec![
                assets.load("audio/sound_effects/step1.ogg"),
                assets.load("audio/sound_effects/step2.ogg"),
                assets.load("audio/sound_effects/step3.ogg"),
                assets.load("audio/sound_effects/step4.ogg"),
            ],
        }
    }
}
