#[derive(Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

struct SquareSpiral {
    x: f32,
    y: f32,
    radius: u32,
    direction: Direction,
}

impl SquareSpiral {
    fn new() -> Self {
        SquareSpiral {
            x: -0.5,
            y: -0.5,
            radius: 0,
            direction: Direction::Right,
        }
    }
}

impl Iterator for SquareSpiral {
    type Item = (f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        let current_square = (self.x, self.y);

        println!("sq_distance: {}", calc_square_distance(current_square));

        let mut next_square = current_square;

        match self.direction {
            Direction::Right => next_square.0 += 1.0,
            Direction::Down => next_square.1 += 1.0,
            Direction::Left => next_square.0 -= 1.0,
            Direction::Up => next_square.1 -= 1.0,
        }

        let sq_distance = calc_square_distance(next_square);

        if sq_distance > self.radius * self.radius {
            match self.direction {
                Direction::Right => self.direction = Direction::Down,
                Direction::Down => {
                    self.direction = Direction::Left;
                    self.radius += 1;
                }
                Direction::Left => self.direction = Direction::Up,
                Direction::Up => {
                    self.direction = Direction::Right;
                }
            }
        }

        println!("{:?}", self.direction);

        match self.direction {
            Direction::Right => self.x += 1.0,
            Direction::Down => self.y += 1.0,
            Direction::Left => self.x -= 1.0,
            Direction::Up => self.y -= 1.0,
        }

        Some(current_square)
    }
}

fn calc_square_distance((x, y): (f32, f32)) -> u32 {
    let x = x as i32;
    let y = y as i32;
    (x * x + y * y) as u32
}

fn main() {
    let mut spiral = SquareSpiral::new();
    let count = 10;
    for _ in 0..count {
        if let Some(value) = spiral.next() {
            println!("{value:?}");
            println!("------//------");
        }
    }
}
