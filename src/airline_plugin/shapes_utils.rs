use bevy::prelude::*;

pub fn directed_triangle_points(position: Vec2, direction: Vec2, size: f32) -> Vec<Vec2> {
    // Fix this! This doesn't actually produce equilateral triangles???

    let TWO_PI_ON_THREE_ROTATION: Vec2 = Vec2::new(-0.5, (3.0 as f32).sqrt() / 2.0);
    let NEG_TWO_PI_ON_THREE_ROTATION: Vec2 = Vec2::new(-0.5, -(3.0 as f32).sqrt() / 2.0);

    let p1 = position + direction * size;
    let p2 = position + TWO_PI_ON_THREE_ROTATION.rotate(direction * size);
    let p3 = position + NEG_TWO_PI_ON_THREE_ROTATION.rotate(direction * size);

    let points = vec![p1, p2, p3];
    return points;
}
