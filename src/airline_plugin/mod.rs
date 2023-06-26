pub mod objects;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::utils::Duration;

use bevy_prototype_lyon::prelude::*;

use rand::prelude::*;

use objects::{Aircraft, Airport};

pub struct AirlinePlugin;

impl Plugin for AirlinePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ShapePlugin)
            .add_startup_system(create_camera)
            .add_startup_system(create_airports)
            .add_system(add_aircraft.run_if(on_timer(Duration::from_secs_f32(1.0))))
            .add_system(move_aircraft);
    }
}

fn create_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn create_airports(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    let points: Vec<Vec2> = (0..5)
        .map(|_| Vec2 {
            x: rng.gen::<f32>() * 500.0 - 250.0,
            y: rng.gen::<f32>() * 500.0 - 250.0,
        })
        .collect();

    let mut pb = PathBuilder::new();
    pb.move_to(points[0]);
    for point in points.iter() {
        // Create and add circles
        let shape_path = shapes::Circle {
            radius: 10.0,
            center: *point,
        };

        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape_path),
                ..default()
            },
            Fill::color(Color::ORANGE),
            Airport {
                pos: *point,
                name: String::from("Airport"),
            },
        ));

        // Create lines connecting the circles
        pb.line_to(*point);
    }
    pb.close();
    let path = pb.build();

    commands.spawn((
        ShapeBundle { path, ..default() },
        Stroke::new(Color::ORANGE, 3.0),
    ));
}

fn add_aircraft(mut commands: Commands, query: Query<&Airport>) {
    for airport in query.iter() {
        let center = airport.pos;
        let p1 = center + Vec2::new(-10.0, -10.0);
        let p2 = center + Vec2::new(-10.0, 10.0);
        let p3 = center + Vec2::new(10.0, 10.0);

        let shape_path = shapes::Polygon {
            points: vec![p1, p2, p3],
            closed: true,
        };

        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape_path),
                ..default()
            },
            Fill::color(Color::RED),
            Aircraft {
                start_pos: airport.pos,
                current_pos: airport.pos,
                end_pos: airport.pos + Vec2::new(100.0, 100.0),
                speed: 1.0,
            },
        ));
    }
}

fn move_aircraft(mut query: Query<(&Aircraft, &mut Transform)>) {
    for (aircraft, mut transform) in query.iter_mut() {
        let direction = (aircraft.end_pos - aircraft.start_pos).normalize();
        transform.translation += Vec3::new(direction.x, direction.y, 0.0) * aircraft.speed;
    }
}
