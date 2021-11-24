mod prelude {
    pub const MAZE_ROW: isize= 11;
    pub const MAZE_COL: isize= 11;
}

use prelude::*;

#[derive(Debug, Copy, Clone)]
struct Coord {
    r: isize,
    c: isize,
}

fn inside(c: &Coord) -> bool {
    if c.c < 0 || c.c == MAZE_COL || c.r < 0 || c.r == MAZE_ROW {
        return false;
    }
    return true;
}

fn print_square(grid: &Vec<Vec<char>>) {
    let mut col = MAZE_COL as usize;
    let mut row = MAZE_ROW as usize;

    for i in 0..row {
        println!("{:?}", grid[i]);
    }
}

fn main() {
    let mut col = MAZE_COL as usize;
    let mut row = MAZE_ROW as usize;

    let mut square_grid = vec![vec!['.'; row]; col];
    let ini: Coord = Coord {r: 0, c: 2};

    print_square(&square_grid);
}
