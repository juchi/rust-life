extern crate time;

use std::io::BufferedReader;
use std::io::File;

use std::clone::Clone;

#[deriving(Clone)]
struct Square {
    x: uint,
    y: uint,
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

fn init_grid() -> Vec<Vec<Square>> {
    let mut grid: Vec<Vec<Square>> = Vec::new();
    let cols: int = 10;
    let rows: int = 10;

    for y in range(0, rows) {
        let mut row = Vec::new();
        for x in range(0, cols) {
            let val: bool = match x {2i => true, _ => false};
            row.push(Square{x:x as uint, y:y as uint, active:val});
        }
        grid.push(row);
    }

    return grid;
}
fn update_grid(grid: &mut Vec<Vec<Square>>) {
    let old_grid = grid.clone();
    for row in grid.iter_mut() {
        for c in row.iter_mut() {
            c.active = get_updated_status(&old_grid, c.x, c.y);
        }
    }
}

fn get_updated_status(old_grid: &Vec<Vec<Square>>, x: uint, y: uint) -> bool {
    let mut allowed_x: Vec<uint> = vec![x];
    let mut allowed_y: Vec<uint> = vec![y];
    if x > 0 {
        allowed_x.push(x-1);
    }
    if x < 10-1 {
        allowed_x.push(x+1);
    }
    if y > 0 {
        allowed_y.push(y-1);
    }
    if y < 10-1 {
        allowed_y.push(y+1);
    }

    let mut total: int = 0;
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
        2 => old_grid[x][y].active,
        3 => true,
        _ => false
    }
}

fn display_grid(grid: &Vec<Vec<Square>>) {
    for row in grid.iter() {
        for c in row.iter() {
            print!("{}", match c.active {false => 0i, true => 1i});
        }
        print!("\n");
    }
    print!("\n");
}

fn get_grid_content() -> Vec<Vec<Square>> {
    let level_path = Path::new("./resources/grid.txt");
    let mut file = BufferedReader::new(File::open(&level_path));
    let lines: Vec<String> = file.lines().map(|x| x.unwrap()).collect();
    let mut grid: Vec<Vec<Square>> = Vec::new();

    let mut y = 0;
    for line in lines.iter() {
        let mut row: Vec<Square> = Vec::new();
        let mut myline = line.clone();
        myline.pop();
        let slice: &str = myline.as_slice();
        let mut x = 0;
        for c in slice.chars() {
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
