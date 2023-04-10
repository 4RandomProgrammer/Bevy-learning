use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn main() {
    //Default plugins tem tudo que é básico e ja vem com a Bevy. Porém a bevy não tem um motor de física.
    App::new()
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins(DefaultPlugins.set(
        WindowPlugin{
            primary_window: Some(Window {
                resolution: (WIDTH, HEIGHT).into(),
                title: "Bevy Tower Defense".to_string(),
                resizable: false,
                ..Default::default()

            }),
            ..default()
        })
        .set(
            AssetPlugin {
                watch_for_changes: true,
                ..Default::default()
            }
        ))
    .add_plugin(WorldInspectorPlugin::new())
    .add_startup_system(spawn_basic_scene)
    .add_startup_system(spawn_camera)
    .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle{
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    //Qual a diferença entre plane{size} e plane.from_size
    commands.spawn(PbrBundle{
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    })
    .insert(Name::new("Ground"));

    commands.spawn(PbrBundle{
        mesh: meshes.add(Mesh::from(shape::Cube{size: 1.0})),
        material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    })
    .insert(Name::new("Square"));

    commands.spawn(PointLightBundle{
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    }).insert(Name::new("Luz"));
}