use std::collections::HashSet;

use macroquad::prelude::*;

const DIRECTIONS: [(f32, f32); 8] = [
    (1.0, 0.0),   // Right
    (1.0, 1.0),   // RightDown
    (0.0, 1.0),   // Down
    (-1.0, 1.0),  // LeftDown
    (-1.0, 0.0),  // Left
    (-1.0, -1.0), // LeftUp
    (0.0, -1.0),  // Up
    (1.0, -1.0),  // RightUp
];

struct SquareSpiral {
    x: f32,
    y: f32,
    radius: u32,
    direction_index: usize,
}

impl SquareSpiral {
    fn new() -> Self {
        SquareSpiral {
            x: -0.5,
            y: -0.5,
            radius: 0,
            direction_index: 0,
        }
    }
}

impl Iterator for SquareSpiral {
    type Item = (f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        let current_square = (self.x, self.y);
        let current_direction = self.direction_index;

        let directions = [
            (current_direction + 7) % 8,
            current_direction,
            (current_direction + 1) % 8,
            (current_direction + 2) % 8,
        ];

        for dir in directions {
            let next_square = move_square(dir, current_square);
            let sq_distance = calc_square_distance(next_square);

            if sq_distance <= self.radius * self.radius {
                self.direction_index = dir;
                self.x = next_square.0;
                self.y = next_square.1;
                break;
            }
        }

        // Left
        if self.direction_index == 4 && self.x == -0.5 {
            println!("radius {} -> {}", self.radius, self.radius + 1);
            self.radius += 1;
        }

        Some(current_square)
    }
}

fn calc_square_distance((x, y): (f32, f32)) -> u32 {
    let x = x as i32;
    let y = y as i32;
    (x * x + y * y) as u32
}

fn move_square(direction_index: usize, (x, y): (f32, f32)) -> (f32, f32) {
    let (dx, dy) = DIRECTIONS[direction_index];
    (x + dx, y + dy)
}

#[macroquad::main("Square Spiral")]
async fn main() {
    let spiral = SquareSpiral::new();
    let mut visited = HashSet::new();

    let points: Vec<(f32, f32)> = spiral.take(100).collect();

    let scale = 30.0;
    let offset_x = screen_width() / 2.0;
    let offset_y = screen_height() / 2.0;

    for (i, &value) in points.iter().enumerate() {
        println!("{}. {} {:?}", i + 1, calc_square_distance(value), value);
        let point = ((value.0 * 2.0) as i32, (value.1 * 2.0) as i32);

        if !visited.insert(point) {
            println!("Повтор на шаге {}: ({}, {})", i + 1, value.0, value.1);
        }
    }

    loop {
        clear_background(Color::from_rgba(16, 16, 24, 255));

        for &(x, y) in points.iter() {
            let px = offset_x + x * scale;
            let py = offset_y - y * scale;

            if px >= 0.0 && px < screen_width() && py >= 0.0 && py < screen_height() {
                draw_rectangle(px, py, 28.0, 28.0, Color::from_rgba(0, 255, 128, 255));
            }
        }

        draw_circle_lines(
            offset_x + scale / 2.0,
            offset_y + scale / 2.0,
            6.0 * scale,
            3.0,
            RED,
        );

        next_frame().await;
    }
}
