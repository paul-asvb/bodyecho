use bevy::prelude::*;

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
        .add_systems(Update, (
            update_loading_progress,
            loading_screen_system,
            move_player,
            animate_sprite
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Create texture atlas layouts for animations
    let idle_layout = TextureAtlasLayout::from_grid(UVec2::new(64, 64), 4, 1, None, None);
    let idle_atlas_handle = texture_atlases.add(idle_layout);

    let walk_layout = TextureAtlasLayout::from_grid(UVec2::new(64, 64), 6, 1, None, None);
    let walk_atlas_handle = texture_atlases.add(walk_layout);

    // Load animation textures for each direction
    let animations = CharacterAnimations {
        idle_south: asset_server.load("characters/582b204c-ad0f-401d-b99a-97ecaf9a0abe/breathing-idle-south.png"),
        idle_north: asset_server.load("characters/582b204c-ad0f-401d-b99a-97ecaf9a0abe/breathing-idle-north.png"),
        idle_west: asset_server.load("characters/582b204c-ad0f-401d-b99a-97ecaf9a0abe/breathing-idle-west.png"),
        idle_east: asset_server.load("characters/582b204c-ad0f-401d-b99a-97ecaf9a0abe/breathing-idle-east.png"),
        walk_south: asset_server.load("characters/582b204c-ad0f-401d-b99a-97ecaf9a0abe/walking-south.png"),
        walk_north: asset_server.load("characters/582b204c-ad0f-401d-b99a-97ecaf9a0abe/walking-north.png"),
        walk_west: asset_server.load("characters/582b204c-ad0f-401d-b99a-97ecaf9a0abe/walking-west.png"),
        walk_east: asset_server.load("characters/582b204c-ad0f-401d-b99a-97ecaf9a0abe/walking-east.png"),
        idle_atlas: idle_atlas_handle.clone(),
        walk_atlas: walk_atlas_handle.clone(),
    };

    // Spawn player character with animation
    commands.spawn((
        SpriteBundle {
            texture: animations.idle_south.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        TextureAtlas {
            layout: idle_atlas_handle,
            index: 0,
        },
        Player {
            speed: 300.0,
            direction: Direction::South,
            state: AnimationState::Idle,
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
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

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    South,
    North,
    West,
    East,
}

#[derive(Clone, Copy, PartialEq)]
enum AnimationState {
    Idle,
    Walking,
}

#[derive(Resource)]
struct CharacterAnimations {
    idle_south: Handle<Image>,
    idle_north: Handle<Image>,
    idle_west: Handle<Image>,
    idle_east: Handle<Image>,
    walk_south: Handle<Image>,
    walk_north: Handle<Image>,
    walk_west: Handle<Image>,
    walk_east: Handle<Image>,
    idle_atlas: Handle<TextureAtlasLayout>,
    walk_atlas: Handle<TextureAtlasLayout>,
}

#[derive(Component)]
struct AnimationTimer(Timer);

// Movement system
fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    animations: Res<CharacterAnimations>,
    mut query: Query<(&mut Transform, &mut Player, &mut Handle<Image>, &mut TextureAtlas)>,
    time: Res<Time>,
) {
    for (mut transform, mut player, mut texture, mut atlas) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        let mut new_facing: Option<Direction> = None;

        // WASD controls - prioritize last pressed direction for sprite
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
            new_facing = Some(Direction::North);
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
            new_facing = Some(Direction::South);
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
            new_facing = Some(Direction::West);
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
            new_facing = Some(Direction::East);
        }

        // Determine animation state
        let is_moving = direction.length() > 0.0;
        let new_state = if is_moving {
            AnimationState::Walking
        } else {
            AnimationState::Idle
        };

        // Update animation if state or direction changed
        if player.state != new_state || new_facing.is_some() {
            let facing = new_facing.unwrap_or(player.direction);
            player.direction = facing;
            player.state = new_state;

            // Update texture and atlas based on state and direction
            match (new_state, facing) {
                (AnimationState::Idle, Direction::South) => {
                    *texture = animations.idle_south.clone();
                    atlas.layout = animations.idle_atlas.clone();
                }
                (AnimationState::Idle, Direction::North) => {
                    *texture = animations.idle_north.clone();
                    atlas.layout = animations.idle_atlas.clone();
                }
                (AnimationState::Idle, Direction::West) => {
                    *texture = animations.idle_west.clone();
                    atlas.layout = animations.idle_atlas.clone();
                }
                (AnimationState::Idle, Direction::East) => {
                    *texture = animations.idle_east.clone();
                    atlas.layout = animations.idle_atlas.clone();
                }
                (AnimationState::Walking, Direction::South) => {
                    *texture = animations.walk_south.clone();
                    atlas.layout = animations.walk_atlas.clone();
                }
                (AnimationState::Walking, Direction::North) => {
                    *texture = animations.walk_north.clone();
                    atlas.layout = animations.walk_atlas.clone();
                }
                (AnimationState::Walking, Direction::West) => {
                    *texture = animations.walk_west.clone();
                    atlas.layout = animations.walk_atlas.clone();
                }
                (AnimationState::Walking, Direction::East) => {
                    *texture = animations.walk_east.clone();
                    atlas.layout = animations.walk_atlas.clone();
                }
            }
            atlas.index = 0;
        }

        // Normalize diagonal movement
        if is_moving {
            direction = direction.normalize();
        }

        // Apply movement with player speed
        transform.translation += direction * player.speed * time.delta_seconds();
    }
}

// Animation system
fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlas, &Player)>,
) {
    for (mut timer, mut atlas, player) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            let max_frames = match player.state {
                AnimationState::Idle => 4,
                AnimationState::Walking => 6,
            };
            atlas.index = (atlas.index + 1) % max_frames;
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

fn update_loading_progress(
    time: Res<Time>,
    mut loading_progress: ResMut<LoadingProgress>,
) {
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
