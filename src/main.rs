use bevy::prelude::*;
use bevy::render::mesh::shape::Circle;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::DefaultPlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init)
        .run()
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        mesh: bevy::sprite::Mesh2dHandle(meshes.add(Circle {
            radius: 20.0,
            vertices: 50,
        })),
        material: materials.add(Color::rgb(255.0, 255.0, 255.0)),
        transform: Transform::from_xyz(30.0, 30.0, 0.0),
        ..default()
    });
}
