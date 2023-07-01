pub mod objects;
pub mod shapes_utils;

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
            .add_system(add_aircraft.run_if(on_timer(Duration::from_secs_f32(5.0))))
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
            x: rng.gen::<f32>() * 1200.0 - 600.0,
            y: rng.gen::<f32>() * 700.0 - 350.0,
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

        let airport = Airport {
            pos: *point,
            name: String::from("Airport"),
        };
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape_path),
                ..default()
            },
            Fill::color(Color::ORANGE),
            airport,
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

fn add_aircraft(mut commands: Commands, query1: Query<&Airport>, query2: Query<&Airport>) {
    let destinations: Vec<&Airport> = query2.iter().collect();
    for airport in query1.iter() {
        let mut rng = rand::thread_rng();
        let destination = destinations
            .iter()
            .filter(|x| x.pos != airport.pos)
            .choose(&mut rng)
            .unwrap();

        let direction = (destination.pos - airport.pos).normalize();
        let points = shapes_utils::directed_triangle_points(airport.pos, direction, 20.0);
        let shape_path = shapes::Polygon {
            points: points,
            closed: true,
        };

        let aircraft = Aircraft::new(airport.pos, destination.pos, 1.0);
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape_path),
                ..default()
            },
            Fill::color(Color::ORANGE_RED),
            aircraft,
        ));
    }
}

fn move_aircraft(
    mut commands: Commands,
    mut query: Query<(&mut Aircraft, &mut Transform, Entity)>,
) {
    for (mut aircraft, mut transform, entity) in query.iter_mut() {
        if aircraft.remaining_steps == 0 {
            commands.entity(entity).despawn();
        } else {
            let direction = (aircraft.end_pos - aircraft.start_pos).normalize();
            transform.translation += Vec3::new(direction.x, direction.y, 1.0) * aircraft.speed;
            aircraft.remaining_steps -= 1;
        }
    }
}
