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

        println!(
            "sq_distance: {} radius: {}",
            calc_square_distance(current_square),
            self.radius
        );

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
        if self.direction_index == 4 && current_direction != 4 {
            self.radius += 1;
        }

        println!("Direction: {:?}", self.direction_index);

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

fn main() {
    let mut spiral = SquareSpiral::new();
    let count = 64;
    for i in 0..count {
        if let Some(value) = spiral.next() {
            println!("step: {}", i + 1);
            println!("Point: {value:?}");
            println!("------//------");
        }
    }
}
