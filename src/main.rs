use bevy::prelude::*;

fn main() {
    #[cfg(target_arch = "wasm32")]
    init_panic_hook();

    App::new()
        .insert_resource(SplashTimer(Timer::from_seconds(3.0, TimerMode::Once)))
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
        .add_systems(Update, splash_screen_timer)
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

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.35)),
            ..default()
        })
        .insert(SplashScreen)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Bodyecho",
                TextStyle {
                    font_size: 72.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
}

#[cfg(target_arch = "wasm32")]
fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[derive(Resource)]
struct SplashTimer(Timer);

#[derive(Component)]
struct SplashScreen;

fn splash_screen_timer(
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
    mut commands: Commands,
    splash_query: Query<Entity, With<SplashScreen>>,
) {
    if timer.0.tick(time.delta()).finished() {
        for entity in splash_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
