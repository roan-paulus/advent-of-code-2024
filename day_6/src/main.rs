use macroquad::prelude::*;
use miniquad::window::order_quit;
use std::fs::File;
use std::io::Read;
use std::thread::sleep;
use std::time::Duration;

type Grid = Vec<Vec<char>>;
const GUARD_START: char = '^';
const OBSTACLE: char = '#';

fn window_conf() -> Conf {
    Conf {
        window_title: "Guard Patrol".to_owned(),
        fullscreen: false,
        window_width: 800,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let grid = get_input("input.txt");

    let guard_positions = find_positions(&grid, GUARD_START);

    assert!(guard_positions.len() == 1);
    let (guard_start_y, guard_start_x) = guard_positions[0];

    let obstacles_positions: Vec<(usize, usize)> = find_positions(&grid, OBSTACLE);
    assert!(!obstacles_positions.is_empty());

    let mut guard = Guard {
        x: guard_start_x,
        y: guard_start_y,
        speed: 1,
        distinct_positions: Vec::new(),
        direction: Direction::Up,
        grid: &grid,
    };
    for _ in &grid {
        let row = vec![false; grid[0].len()];
        guard.distinct_positions.push(row);
    }
    // Add starting position.
    guard.distinct_positions[guard.y][guard.x] = true;

    let mut out_of_bounds = OutOfBounds::False;
    let mut distinct_positions = 0;

    let visualize = std::env::args().nth(1).is_some();
    if visualize {
        let grid_size = grid.len() as f32;
        let game_size = screen_width().min(screen_height());
        let sq_size = game_size / grid_size;
        let offset_x = sq_size;
        let offset_y = sq_size;

        clear_background(BLACK);

        loop {
            if is_key_down(KeyCode::Q) {
                println!("Exitting...");
                order_quit();
            }

            if matches!(out_of_bounds, OutOfBounds::True) {
                if distinct_positions == 0 {
                    for row in &guard.distinct_positions {
                        for col in row {
                            if *col {
                                distinct_positions += 1;
                            }
                        }
                    }
                }
                let text = format!(
                    "The guards visited a total amount of {} distinct positions.",
                    distinct_positions
                );
                let font_size = 30.;
                let text_size = measure_text(&text, None, font_size as _, 1.0);

                draw_text(
                    &text,
                    screen_width() / 2. - text_size.width / 2.,
                    screen_height() / 2. + text_size.height / 2.,
                    font_size,
                    WHITE,
                );
            } else {
                out_of_bounds = guard.locomote();
            }

            for pos in obstacles_positions.iter() {
                draw_rectangle(
                    pos.1 as f32 * offset_x,
                    pos.0 as f32 * offset_y,
                    sq_size,
                    sq_size,
                    BROWN,
                );
            }
            draw_rectangle(
                guard.x as f32 * offset_x,
                guard.y as f32 * offset_y,
                sq_size,
                sq_size,
                WHITE,
            );
            print!("{}[2J", 27 as char);
            for row in &guard.distinct_positions {
                for col in row {
                    if *col {
                        print!("X");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
            next_frame().await
        }
    } else {
        while matches!(out_of_bounds, OutOfBounds::False) {
            out_of_bounds = guard.locomote();
        }
        for row in guard.distinct_positions {
            for col in row {
                if col {
                    distinct_positions += 1;
                }
            }
        }
        println!(
            "The guards visited a total amount of {} distinct positions.",
            distinct_positions
        );
    }
}

fn find_positions(grid: &Grid, search: char) -> Vec<(usize, usize)> {
    let mut positions = vec![];
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == search {
                positions.push((i, j))
            }
        }
    }
    positions
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

struct Guard<'g> {
    x: usize,
    y: usize,
    speed: usize,
    direction: Direction,
    distinct_positions: Vec<Vec<bool>>,
    grid: &'g Grid,
}

enum OutOfBounds {
    True,
    False,
}

impl Guard<'_> {
    fn locomote(&mut self) -> OutOfBounds {
        let grid_size = self.grid.len();

        match self.direction {
            Direction::Up => {
                let new_y = self.y.checked_sub(self.speed);
                if new_y.is_none() {
                    return OutOfBounds::True;
                }
                self.update_position(self.x, new_y.unwrap(), Direction::Right);
            }
            Direction::Down => {
                let new_y = self.y + self.speed;
                if new_y > grid_size - 1 {
                    return OutOfBounds::True;
                }
                self.update_position(self.x, new_y, Direction::Left);
            }
            Direction::Right => {
                let new_x = self.x + self.speed;
                if new_x > grid_size - 1 {
                    return OutOfBounds::True;
                }
                self.update_position(new_x, self.y, Direction::Down);
            }
            Direction::Left => {
                let new_x = self.x.checked_sub(self.speed);
                if new_x.is_none() {
                    return OutOfBounds::True;
                }
                self.update_position(new_x.unwrap(), self.y, Direction::Up);
            }
        };
        OutOfBounds::False
    }

    fn update_position(&mut self, x: usize, y: usize, potential_new_direction: Direction) {
        match self.grid[y][x] {
            '#' => self.direction = potential_new_direction,
            _ => {
                self.x = x;
                self.y = y;
                self.distinct_positions[self.y][self.x] = true;
            }
        }
    }
}

fn get_input(path: &str) -> Grid {
    let mut f = File::open(path).unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let mut rows = vec![];
    let lines = input.split('\n');
    let row_length = lines.clone().next().unwrap().len();

    for row in lines {
        let mut char_row = vec![];
        for ch in row.chars() {
            char_row.push(ch);
        }
        if char_row.len() == row_length {
            rows.push(char_row);
        } else {
            println!("Skipped pushing an empty row.");
        }
    }
    println!("Loaded grid.");
    rows
}
