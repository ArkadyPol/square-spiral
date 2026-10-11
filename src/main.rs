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
    old_radius: Option<u32>,
    old_limit: Option<u32>,
    direction_index: usize,
}

impl SquareSpiral {
    fn new() -> Self {
        SquareSpiral {
            x: -0.5,
            y: -0.5,
            radius: 0,
            old_radius: None,
            old_limit: None,
            direction_index: 0,
        }
    }
}

impl Iterator for SquareSpiral {
    type Item = ((f32, f32), u32);

    fn next(&mut self) -> Option<Self::Item> {
        let current_square = (self.x, self.y);
        let current_direction = self.direction_index;

        let r = self.radius;
        let mid_max_limit = r * r + r;

        let directions = [
            (current_direction + 7) % 8,
            current_direction,
            (current_direction + 1) % 8,
            (current_direction + 2) % 8,
            (current_direction + 6) % 8,
        ];

        let mut diagonal_move = (current_direction, current_square);
        let mut found = false;

        for dir in directions {
            let next_square = move_square(dir, current_square);
            let sq_distance = calc_square_distance(next_square);

            if sq_distance <= mid_max_limit
                && self
                    .old_limit
                    .is_none_or(|old_limit| sq_distance > old_limit)
            {
                if dir % 2 == 1 {
                    diagonal_move = (dir, next_square);
                    continue;
                }

                found = true;

                self.direction_index = dir;
                self.x = next_square.0;
                self.y = next_square.1;
                break;
            }
        }

        if !found {
            self.direction_index = diagonal_move.0;
            self.x = diagonal_move.1.0;
            self.y = diagonal_move.1.1;
        }

        // Left
        if self.direction_index == 4 && self.x == -0.5 {
            println!("radius {} -> {}", self.radius, self.radius + 1);
            self.radius += 1;
        }

        let cur_sq_distance = calc_square_distance((self.x, self.y));

        if cur_sq_distance > 0 && self.old_limit == None {
            self.old_limit = Some(0);
            self.old_radius = Some(0);
        }

        if let Some(old_limit) = self.old_limit
            && let Some(r) = self.old_radius
        {
            let mid_max_limit = r * r + r;
            if cur_sq_distance > mid_max_limit {
                self.old_limit = Some(mid_max_limit);
                self.old_radius = Some(r + 1);
                println!("old_limit {} -> {:?}", old_limit, self.old_limit);
            }
        }

        Some(((current_square), r))
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

fn color_for_radius(radius: u32) -> Color {
    let colors = [RED, ORANGE, YELLOW, GREEN, SKYBLUE, BLUE, DARKPURPLE];

    colors[radius as usize % colors.len()]
}

#[macroquad::main("Square Spiral")]
async fn main() {
    let spiral = SquareSpiral::new();
    let count = 88;
    let mut visited = HashSet::new();

    let points: Vec<((f32, f32), u32)> = spiral.take(count).collect();

    let scale = 30.0;

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

        if elapsed >= 0.1 {
            elapsed -= 0.1;

            if i < points.len() {
                drawing_points.push(points[i]);
                i += 1;
            }
        }

        clear_background(Color::from_rgba(16, 16, 24, 255));

        for &((x, y), r) in drawing_points.iter() {
            let px = offset_x + x * scale;
            let py = offset_y + y * scale;

            if px >= 0.0 && px < screen_width() && py >= 0.0 && py < screen_height() {
                draw_rectangle(px, py, 28.0, 28.0, color_for_radius(r));
            }
        }

        draw_circle_lines(
            offset_x + scale / 2.0,
            offset_y + scale / 2.0,
            5.0 * scale,
            3.0,
            WHITE,
        );

        next_frame().await;
    }
}
