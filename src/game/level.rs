//! Spawn the main level.

use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    audio::music,
    game::{
        court::spawn_court,
        player::{PlayerAssets, player},
    },
    screens::Screen,
};

use super::player::PlayerSide;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LevelAssets>();
    app.load_resource::<LevelAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
        }
    }
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    player_assets: Res<PlayerAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn the main level entity
    let level_entity = commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
    )).id();
    
    // Spawn court as a child
    let court_entity = spawn_court(&mut commands, &mut meshes, &mut materials);
    
    // Spawn player and music
    let children = vec![
        court_entity,
        commands.spawn(player(
            PlayerSide::Left,
            400.0,
            &player_assets,
            &mut texture_atlas_layouts
        )).id(),
        commands.spawn((
            Name::new("Gameplay Music"),
            music(level_assets.music.clone())
        )).id(),
    ];
    
    // Add all children to the level
    commands.entity(level_entity).add_children(&children);
}
