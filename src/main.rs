use std::collections::HashSet;

use macroquad::prelude::*;

use crate::spiral::{SquareSpiral, calc_square_distance};

mod spiral;

#[macroquad::main("Square Spiral")]
async fn main() {
    let spiral = SquareSpiral::new();
    let count = 468;
    let mut visited = HashSet::new();

    let points: Vec<((f32, f32), u32)> = spiral.take(count).collect();

    let scale = 20.0;

    for (i, &(point, _r)) in points.iter().enumerate() {
        println!("{}. {} {:?}", i + 1, calc_square_distance(point), point);
        let point = ((point.0 * 2.0) as i32, (point.1 * 2.0) as i32);

        if !visited.insert(point) {
            println!("Повтор на шаге {}: ({}, {})", i + 1, point.0, point.1);
        }
    }

    let mut drawing_points = Vec::new();

    let mut elapsed = 0.0;

    let mut i = 0;

    loop {
        let offset_x = screen_width() / 2.0;
        let offset_y = screen_height() / 2.0;

        elapsed += get_frame_time();

        if elapsed >= 1.0 / 60.0 {
            elapsed -= 1.0 / 60.0;

            for _ in 0..4 {
                if i < points.len() {
                    drawing_points.push(points[i]);
                    i += 1;
                } else {
                    break;
                }
            }
        }

        clear_background(Color::from_rgba(16, 16, 24, 255));

        for &((x, y), r) in drawing_points.iter() {
            let px = offset_x + x * scale;
            let py = offset_y + y * scale;

            if px >= 0.0 && px < screen_width() && py >= 0.0 && py < screen_height() {
                draw_rectangle(px, py, scale * 0.9, scale * 0.9, color_for_radius(r));
            }
        }

        draw_circle_lines(
            offset_x + scale / 2.0,
            offset_y + scale / 2.0,
            12.0 * scale,
            3.0,
            WHITE,
        );

        next_frame().await;
    }
}

fn color_for_radius(radius: u32) -> Color {
    let colors = [RED, ORANGE, YELLOW, GREEN, SKYBLUE, BLUE, DARKPURPLE];

    colors[radius as usize % colors.len()]
}
