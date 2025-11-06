use bevy::prelude::*;
use std::collections::HashMap;

const PLAYER_MOVEMENT_SPEED: f32 = 80.0;

fn main() {
    #[cfg(target_arch = "wasm32")]
    init_panic_hook();

    App::new()
        .insert_resource(LoadingProgress::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bodyecho".into(),
                resolution: (960.0, 540.0).into(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                update_loading_progress,
                loading_screen_system,
                move_player,
                animate_sprite,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Base path for character assets
    let base_path = "characters/582b204c-ad0f-401d-b99a-97ecaf9a0abe/";

    // Load all frames based on the directory structure
    let mut idle_frames = HashMap::new();
    let mut walk_frames = HashMap::new();

    // Define the directions and their frame counts
    let directions = [
        Direction::South,
        Direction::SouthEast,
        Direction::East,
        Direction::NorthEast,
        Direction::North,
        Direction::NorthWest,
        Direction::West,
        Direction::SouthWest,
    ];

    // Load idle animation frames (4 frames per direction)
    for direction in &directions {
        let dir_str = format!("{:?}", direction)
            .to_lowercase()
            .replace("southeast", "south-east")
            .replace("northeast", "north-east")
            .replace("southwest", "south-west")
            .replace("northwest", "north-west");

        let mut frames = Vec::new();
        for i in 0..4 {
            let path = format!(
                "{}animations/breathing-idle/{}/frame_{:03}.png",
                base_path, dir_str, i
            );
            frames.push(asset_server.load(path));
        }
        idle_frames.insert(*direction, frames);
    }

    // Load walk animation frames (6 frames per direction)
    for direction in &directions {
        let dir_str = format!("{:?}", direction)
            .to_lowercase()
            .replace("southeast", "south-east")
            .replace("northeast", "north-east")
            .replace("southwest", "south-west")
            .replace("northwest", "north-west");

        let mut frames = Vec::new();
        for i in 0..6 {
            let path = format!(
                "{}animations/walk/{}/frame_{:03}.png",
                base_path, dir_str, i
            );
            frames.push(asset_server.load(path));
        }
        walk_frames.insert(*direction, frames);
    }

    let animations = CharacterAnimations {
        idle_frames: idle_frames.clone(),
        walk_frames: walk_frames.clone(),
    };

    // Spawn player character with first idle frame
    let first_frame = idle_frames.get(&Direction::South).unwrap()[0].clone();
    commands.spawn((
        SpriteBundle {
            texture: first_frame,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Player {
            speed: PLAYER_MOVEMENT_SPEED,
            direction: Direction::South,
            state: AnimationState::Idle,
        },
        AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
        AnimationFrameIndex(0),
    ));

    // Store animations as resource
    commands.insert_resource(animations);

    // Loading screen overlay
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
            ..default()
        })
        .insert(LoadingScreen)
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle::from_section(
                "Bodyecho",
                TextStyle {
                    font_size: 72.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            // Spacing
            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(40.0),
                    ..default()
                },
                ..default()
            });

            // Progress bar container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(400.0),
                        height: Val::Px(20.0),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    border_color: BorderColor(Color::srgb(0.6, 0.6, 0.6)),
                    ..default()
                })
                .with_children(|parent| {
                    // Progress bar fill
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(0.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            background_color: BackgroundColor(Color::srgb(0.2, 0.8, 0.4)),
                            ..default()
                        })
                        .insert(ProgressBar);
                });

            // Loading text
            parent
                .spawn(TextBundle::from_section(
                    "Loading... 0%",
                    TextStyle {
                        font_size: 24.0,
                        color: Color::srgb(0.8, 0.8, 0.8),
                        ..default()
                    },
                ))
                .insert(LoadingText);
        });
}

#[cfg(target_arch = "wasm32")]
fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

// Player component
#[derive(Component)]
struct Player {
    speed: f32,
    direction: Direction,
    state: AnimationState,
}

#[derive(Clone, Copy, PartialEq, Debug, Eq, Hash)]
enum Direction {
    South,
    SouthEast,
    East,
    NorthEast,
    North,
    NorthWest,
    West,
    SouthWest,
}

impl Direction {
    fn from_movement(x: f32, y: f32) -> Self {
        let angle = y.atan2(x);
        let degrees = angle.to_degrees();

        // Convert angle to direction (8-way)
        match degrees {
            d if d >= -22.5 && d < 22.5 => Direction::East,
            d if d >= 22.5 && d < 67.5 => Direction::NorthEast,
            d if d >= 67.5 && d < 112.5 => Direction::North,
            d if d >= 112.5 && d < 157.5 => Direction::NorthWest,
            d if d >= 157.5 || d < -157.5 => Direction::West,
            d if d >= -157.5 && d < -112.5 => Direction::SouthWest,
            d if d >= -112.5 && d < -67.5 => Direction::South,
            d if d >= -67.5 && d < -22.5 => Direction::SouthEast,
            _ => Direction::South,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum AnimationState {
    Idle,
    Walking,
}

#[derive(Resource)]
struct CharacterAnimations {
    idle_frames: HashMap<Direction, Vec<Handle<Image>>>,
    walk_frames: HashMap<Direction, Vec<Handle<Image>>>,
}

#[derive(Component)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct AnimationFrameIndex(usize);

// Movement system
fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    animations: Res<CharacterAnimations>,
    mut query: Query<(
        &mut Transform,
        &mut Player,
        &mut Handle<Image>,
        &mut AnimationFrameIndex,
    )>,
    time: Res<Time>,
) {
    for (mut transform, mut player, mut texture, mut frame_index) in query.iter_mut() {
        let mut movement = Vec2::ZERO;

        // WASD controls
        if keyboard_input.pressed(KeyCode::KeyW) {
            movement.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            movement.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            movement.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            movement.x += 1.0;
        }

        // Determine animation state
        let is_moving = movement.length() > 0.0;
        let new_state = if is_moving {
            AnimationState::Walking
        } else {
            AnimationState::Idle
        };

        // Calculate facing direction from movement
        let new_direction = if is_moving {
            Some(Direction::from_movement(movement.x, movement.y))
        } else {
            None
        };

        // Update animation if state or direction changed
        let state_changed = player.state != new_state;
        let direction_changed =
            new_direction.is_some() && new_direction.unwrap() != player.direction;

        if state_changed || direction_changed {
            if let Some(dir) = new_direction {
                player.direction = dir;
            }
            player.state = new_state;

            // Update to first frame of new animation
            frame_index.0 = 0;
            let frames = match new_state {
                AnimationState::Idle => &animations.idle_frames,
                AnimationState::Walking => &animations.walk_frames,
            };

            if let Some(direction_frames) = frames.get(&player.direction) {
                if let Some(first_frame) = direction_frames.first() {
                    *texture = first_frame.clone();
                }
            }
        }

        // Normalize diagonal movement
        if is_moving {
            movement = movement.normalize();
        }

        // Apply movement with player speed
        transform.translation.x += movement.x * player.speed * time.delta_seconds();
        transform.translation.y += movement.y * player.speed * time.delta_seconds();
    }
}

// Animation system
fn animate_sprite(
    time: Res<Time>,
    animations: Res<CharacterAnimations>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut AnimationFrameIndex,
        &mut Handle<Image>,
        &Player,
    )>,
) {
    for (mut timer, mut frame_index, mut texture, player) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            // Get the appropriate frame list based on state and direction
            let frames = match player.state {
                AnimationState::Idle => &animations.idle_frames,
                AnimationState::Walking => &animations.walk_frames,
            };

            if let Some(direction_frames) = frames.get(&player.direction) {
                // Advance to next frame
                frame_index.0 = (frame_index.0 + 1) % direction_frames.len();

                // Update the texture
                if let Some(new_frame) = direction_frames.get(frame_index.0) {
                    *texture = new_frame.clone();
                }
            }
        }
    }
}

#[derive(Resource, Default)]
struct LoadingProgress {
    progress: f32,
    timer: f32,
}

#[derive(Component)]
struct LoadingScreen;

#[derive(Component)]
struct ProgressBar;

#[derive(Component)]
struct LoadingText;

fn update_loading_progress(time: Res<Time>, mut loading_progress: ResMut<LoadingProgress>) {
    if loading_progress.progress < 100.0 {
        loading_progress.timer += time.delta_seconds();
        // Simulate loading - reaches 100% after 2 seconds
        loading_progress.progress = (loading_progress.timer / 2.0 * 100.0).min(100.0);
    }
}

fn loading_screen_system(
    loading_progress: Res<LoadingProgress>,
    mut commands: Commands,
    loading_screen_query: Query<Entity, With<LoadingScreen>>,
    mut progress_bar_query: Query<&mut Style, With<ProgressBar>>,
    mut loading_text_query: Query<&mut Text, With<LoadingText>>,
) {
    // Update progress bar width
    for mut style in progress_bar_query.iter_mut() {
        style.width = Val::Percent(loading_progress.progress);
    }

    // Update loading text
    for mut text in loading_text_query.iter_mut() {
        text.sections[0].value = format!("Loading... {:.0}%", loading_progress.progress);
    }

    // Remove loading screen when complete
    if loading_progress.progress >= 100.0 {
        for entity in loading_screen_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
