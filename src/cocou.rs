use bevy::{input::keyboard::Key, prelude::*};
pub struct CocouPlugin;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Direction(Vec2);

#[derive(Component)]
struct Platform;

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Player>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

fn handle_keys(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Direction), With<Player>>,
) {
    let speed: f32 = 200.0;
    for (mut transform, mut direction) in &mut query {
        if input.pressed(KeyCode::ArrowRight) {
            direction.0.x = 1.0;
        } else if input.pressed(KeyCode::ArrowLeft) {
            direction.0.x = -1.0;
        } else {
            direction.0.x = 0.0;
        }
        if input.pressed(KeyCode::ArrowUp) {
            direction.0.y = 1.0;
        } else if input.pressed(KeyCode::ArrowDown) {
            direction.0.y = -1.0;
        } else {
            direction.0.y = 0.0;
        }
        let move_dir = direction.0.normalize_or_zero();
        transform.translation += move_dir.extend(0.0) * speed * time.delta_secs();
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    let capsule = meshes.add(Capsule2d::new(16.0, 16.0));
    let color = Color::srgba(0.0, 0.0, 0.9, 1.0);
    commands.spawn((
        Mesh2d(capsule),
        Player,
        Direction(Vec2::ZERO),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    let platform_mesh = meshes.add(Rectangle::new(1.0, 1.0));
    let platform_color = Color::srgba(0.4, 0.4, 0.4, 1.0);
    let platform_material = materials.add(platform_color);
    let platform_position = [
        (Vec2::new(-100., -100.), Vec2::new(100., 100.)),
        (Vec2::new(300., -100.), Vec2::new(200., 100.)),
        (Vec2::new(-500., 100.), Vec2::new(100., 200.)),
        (Vec2::new(300., 100.), Vec2::new(200., 100.)),
    ];
    for (pos, size) in platform_position {
        commands.spawn((
            Mesh2d(platform_mesh.clone()),
            MeshMaterial2d(platform_material.clone()),
            Transform {
                translation: pos.extend(0.0),
                scale: size.extend(1.0),
                ..default()
            },
            Platform,
        ));
    }
}

impl Plugin for CocouPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, handle_keys);
        // app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
    }
}
