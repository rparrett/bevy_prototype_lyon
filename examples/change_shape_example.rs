use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ShapeChangeTimer(Timer::from_seconds(2.0, true)))
        .insert_resource(ShapeSides { num: 6, delta: -1 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup.system())
        .add_system(change_color.system())
        .add_system(change_stroke.system())
        .add_system(change_shape.system())
        .add_system(rotate.system())
        .run();
}

struct ExampleShape;
struct ShapeChangeTimer(Timer);
struct ShapeSides {
    num: i32,
    delta: i32,
}

fn rotate(mut query: Query<&mut Transform, With<ExampleShape>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        let delta = time.delta_seconds();

        transform.rotate(Quat::from_rotation_z(0.2 * delta));
    }
}

fn change_color(mut query: Query<&mut ShapeColors>, time: Res<Time>) {
    for mut colors in query.iter_mut() {
        let h = (time.seconds_since_startup() * 50.0) % 360.0;

        *colors = ShapeColors {
            main: Color::hsl(h as f32, 1.0, 0.5),
            outline: Color::BLACK,
        };
    }
}

fn change_stroke(mut query: Query<&mut DrawMode>, time: Res<Time>) {
    for mut draw_mode in query.iter_mut() {
        let w = 2.0 + time.seconds_since_startup().sin().abs() * 10.0;

        *draw_mode = DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(w as f32),
        }
    }
}

fn change_shape(
    mut commands: Commands,
    query: Query<Entity, With<ExampleShape>>,
    mut sides: ResMut<ShapeSides>,
    mut timer: ResMut<ShapeChangeTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if !timer.0.just_finished() {
        return;
    }

    for ent in query.iter() {
        sides.num += sides.delta;
        if sides.num >= 6 || sides.num <= 3 {
            sides.delta = -sides.delta;
        }

        let mut builder = Builder::default();

        let shape = shapes::RegularPolygon {
            sides: sides.num as usize,
            feature: shapes::RegularPolygonFeature::Radius(200.0),
            ..shapes::RegularPolygon::default()
        };

        shape.add_geometry(&mut builder);

        let path = builder.build();

        commands.entity(ent).insert(path);
    }
}

fn setup(mut commands: Commands) {
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(200.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            ShapeColors::outlined(Color::TEAL, Color::BLACK),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(10.0),
            },
            Transform::default(),
        ))
        .insert(ExampleShape);
}
