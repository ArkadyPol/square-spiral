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
    direction_index: usize,
}

impl SquareSpiral {
    fn new() -> Self {
        SquareSpiral {
            x: -0.5,
            y: -0.5,
            radius: 0,
            old_radius: None,
            direction_index: 0,
        }
    }

    fn turn(&mut self) {
        self.direction_index = (self.direction_index + 1) % 8;

        // Left
        if self.direction_index == 4 {
            self.radius += 1;
        }

        // Up
        if self.direction_index == 6 {
            self.old_radius = Some(self.radius - 1);
        }
    }

    fn back_turn(&mut self) {
        self.direction_index = (self.direction_index + 7) % 8;
    }
}

impl Iterator for SquareSpiral {
    type Item = (f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        let current_square = (self.x, self.y);

        println!(
            "sq_distance: {} radius: {}",
            calc_square_distance(current_square),
            self.radius
        );

        let next_square = move_square(self.direction_index, current_square);
        let sq_distance = calc_square_distance(next_square);

        if sq_distance > self.radius * self.radius {
            self.turn();
        }

        let next_square = move_square(self.direction_index, current_square);
        let sq_distance = calc_square_distance(next_square);

        if sq_distance > self.radius * self.radius {
            self.turn();
        }

        if let Some(old_radius) = self.old_radius {
            let next_square = move_square(self.direction_index, current_square);
            let sq_distance = calc_square_distance(next_square);

            if sq_distance <= old_radius * old_radius {
                self.back_turn();
            }
        }

        println!("Direction: {:?}", self.direction_index);

        let new_coords = move_square(self.direction_index, current_square);
        self.x = new_coords.0;
        self.y = new_coords.1;

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
