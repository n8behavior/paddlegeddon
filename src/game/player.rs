//! Player-specific behavior.

use avian2d::prelude::*;
use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use bevy_enhanced_input::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    game::court::COURT_HEIGHT,
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

    // Input handling
    app.add_plugins(EnhancedInputPlugin)
        .add_input_context::<Gameplay>()
        .add_observer(move_player);
}

#[derive(Reflect, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlayerSide {
    #[default]
    Left,
    Right,
}

/// Movement action for players - outputs Vec2 for full 2D movement
#[derive(Debug, InputAction)]
#[action_output(Vec2)]
pub struct Move;

/// Context for active gameplay (as opposed to menus)
#[derive(Component, Default)]
pub struct Gameplay;

/// The player character.
pub fn player(
    side: PlayerSide,
    position: Vec3,
    _player_assets: &PlayerAssets,
    _texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    // A texture atlas is a way to split a single image into a grid of related images.
    // You can learn more in this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    //let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
    //let texture_atlas_layout = texture_atlas_layouts.add(layout);
    //let player_animation = PlayerAnimation::new();

    let paddle_height = COURT_HEIGHT * PADDLE_HEIGHT_RATIO;

    // Create actions for both paddles (observer will filter by side)
    let actions = actions!(Gameplay[
        (
            Action::<Move>::new(),
            Bindings::spawn(Cardinal::wasd_keys()),
        ),
    ]);

    (
        Name::new("Player"),
        Player { side },
        Gameplay, // Add the context component
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
        actions,
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

/// Apply movement when Move action is fired
fn move_player(trigger: Trigger<Fired<Move>>, mut paddles: Query<(&Player, &mut LinearVelocity)>) {
    if let Ok((player, mut velocity)) = paddles.get_mut(trigger.target()) {
        // Only move left paddle for now
        if player.side == PlayerSide::Left {
            // Only use the y component of the movement vector
            velocity.y = trigger.value.y * PADDLE_MAX_SPEED;
        }
    }
}
