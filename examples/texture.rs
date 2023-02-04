//! This is the example that goes to the README.md file. The README.md should be
//! updated before every release.

use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::*, render::ShapeMaterial};

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ShapeMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(200.0),
        ..shapes::RegularPolygon::default()
    };
    let mat = ShapeMaterial {
        texture: Some(asset_server.load("icon.png")),
        ..default()
    };

    commands.spawn(Camera2dBundle::default());
    let id = commands
        .spawn(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::BLACK, 10.0),
            },
            Transform::default(),
        ))
        .id();
    commands.entity(id).insert(materials.add(mat));
}
