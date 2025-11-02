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
        .add_systems(Update, (update_loading_progress, loading_screen_system))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(0.2, 0.6, 0.9),
            custom_size: Some(Vec2::new(200.0, 200.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

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
