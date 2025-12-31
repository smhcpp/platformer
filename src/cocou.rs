use bevy::{input::keyboard::Key, prelude::*};
pub struct CocouPlugin;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

fn add_people(mut commands: Commands) {
    commands.spawn((Player, Name("Elaina Proctor".to_string())));
    commands.spawn((Player, Name("Renzo Hume".to_string())));
    commands.spawn((Player, Name("Zayna Nieves".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Player>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

fn update_people(mut query: Query<&mut Name, With<Player>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // We don't need to change any other names.
        }
    }
}

fn handle_keys(input: Res<ButtonInput<KeyCode>>, input_key: Res<ButtonInput<Key>>) {
    if input.just_pressed(KeyCode::ArrowRight) {
        println!("Right pressed!");
    }
    if input.just_pressed(KeyCode::ArrowLeft) {
        println!("Left pressed!");
    }
    if input.just_pressed(KeyCode::ArrowUp) {
        println!("Up pressed!");
    }
    if input.just_pressed(KeyCode::ArrowDown) {
        println!("Down pressed!");
    }
    if input.just_pressed(KeyCode::KeyC) {
        println!("C pressed!");
    }
    if input.just_pressed(KeyCode::KeyX) {
        println!("X pressed!");
    }
    if input.just_pressed(KeyCode::KeyV) {
        println!("V pressed!");
    }
    if input.just_pressed(KeyCode::KeyZ) {
        println!("Z pressed!");
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
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

impl Plugin for CocouPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, handle_keys);
        // app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
    }
}
