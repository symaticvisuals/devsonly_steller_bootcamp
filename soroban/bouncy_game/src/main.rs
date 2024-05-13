use std::fmt::{Display, Formatter, Error};
use std::time::Duration; // Added for sleep

enum VerticalDirection {
    Up,
    Down,
}

enum HorizontalDirection {
    Left,
    Right,
}

struct Ball {
    x: i32,
    y: i32,
    vert_dir: VerticalDirection,
    horiz_dir: HorizontalDirection,
}

struct Frame {
    width: i32,
    height: i32,
}

struct Game {
    frame: Frame,
    ball: Ball,
}

impl Game {
    fn new() -> Game {
        Game {
            frame: Frame { width: 63, height: 31 },
            ball: Ball { x: 44, y: 21, vert_dir: VerticalDirection::Down, horiz_dir: HorizontalDirection::Right },
        }
    }

    fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}

impl Ball {
    fn bounce(&mut self, frame: &Frame) {
        if self.x == 0 || self.x == frame.width - 1 {
            self.horiz_dir = match self.horiz_dir {
                HorizontalDirection::Left => HorizontalDirection::Right,
                HorizontalDirection::Right => HorizontalDirection::Left,
            };
        }
        if self.y == 0 || self.y == frame.height - 1 {
            self.vert_dir = match self.vert_dir {
                VerticalDirection::Up => VerticalDirection::Down,
                VerticalDirection::Down => VerticalDirection::Up,
            };
        }
    }

    fn mv(&mut self) {
        match self.horiz_dir {
            HorizontalDirection::Left => self.x -= 1,
            HorizontalDirection::Right => self.x += 1,
        }
        match self.vert_dir {
            VerticalDirection::Up => self.y -= 1,
            VerticalDirection::Down => self.y += 1,
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let top_bottom = |f: &mut Formatter| {
            write!(f, "+")?;
            for _ in 0..self.frame.width {
                write!(f, "-")?;
            }
            write!(f, "+\n")
        };

        top_bottom(f)?;

        for row in 0..self.frame.height {
            write!(f, "|")?;
            for col in 0..self.frame.width {
                if row == self.ball.y && col == self.ball.x {
                    write!(f, "o")?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "|\n")?;
        }

        top_bottom(f)
    }
}

fn main() {
    let mut new_game = Game::new();
    let sleep_time: Duration = Duration::from_millis(100);
    loop {
        println!("{}", new_game);
        new_game.step();
        std::thread::sleep(sleep_time);
        println!("{} {}", new_game.ball.x, new_game.ball.y);
    }
}
