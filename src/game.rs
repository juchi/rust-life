extern crate time;

use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

use std::clone::Clone;

#[derive(Clone)]
struct Square {
    x: usize,
    y: usize,
    active: bool
}

pub fn run() {
    println!("run game");

    let mut grid = get_grid_content();

    let mut last_update: f64 = 0.0;
    loop {
        let t = time::precise_time_s();
        if t - last_update > 0.5 {
            display_grid(&grid);
            update_grid(&mut grid);
            last_update = t;
        }
    }
}

fn update_grid(grid: &mut Vec<Vec<Square>>) {
    let old_grid = grid.clone();
    for row in grid.iter_mut() {
        for c in row.iter_mut() {
            c.active = get_updated_status(&old_grid, c.x, c.y);
        }
    }
}

fn get_updated_status(old_grid: &Vec<Vec<Square>>, x: usize, y: usize) -> bool {
    let height = old_grid.len();
    let width = old_grid[0].len();
    let mut allowed_x: Vec<usize> = vec![x];
    let mut allowed_y: Vec<usize> = vec![y];
    if x > 0 {
        allowed_x.push(x-1);
    }
    if x < width-1 {
        allowed_x.push(x+1);
    }
    if y > 0 {
        allowed_y.push(y-1);
    }
    if y < height-1 {
        allowed_y.push(y+1);
    }

    let mut total: i32 = 0;
    for i in allowed_x.iter() {
        for j in allowed_y.iter() {
            if *i == x && *j == y {
                continue;
            }
            if old_grid[*j][*i].active == true {
                total += 1;
            }
        }
    }

    match total {
        2 => old_grid[y][x].active,
        3 => true,
        _ => false
    }
}

fn display_grid(grid: &Vec<Vec<Square>>) {
    for row in grid.iter() {
        for c in row.iter() {
            print!("{}", match c.active {false => 0i32, true => 1i32});
        }
        print!("\n");
    }
    print!("\n");
}

fn get_grid_content() -> Vec<Vec<Square>> {
    let grid_path = Path::new("./resources/grid.txt");
    let file = match File::open(&grid_path) {
        Ok(file) => file,
        Err(..) => panic!("Error reading file")
    };
    let buffer = BufReader::new(file);
    let lines: Vec<String> = buffer.lines().map(|x| x.unwrap()).collect();
    let mut grid: Vec<Vec<Square>> = Vec::new();

    let mut y = 0;
    for line in lines.iter() {
        let mut row: Vec<Square> = Vec::new();
        let mut myline = line.clone();
        myline.pop();
        let mut x = 0;
        for c in myline.chars() {
            let v: bool = match c.to_digit(2) {
                Some(1) => true,
                _ => false
            };
            row.push(Square{x:x, y:y, active:v});
            x += 1;
        }
        y += 1;
        grid.push(row);
    }
    return grid;
}
