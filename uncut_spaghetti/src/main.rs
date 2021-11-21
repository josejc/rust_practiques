mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const SIZE_SQUARE: isize= 10;
}

use prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello world...")
    }
}

#[derive(Debug, Copy, Clone)]
struct Coord {
    c: isize,
    r: isize,
}

fn index(c: Coord) -> isize {
    return c.r*SIZE_SQUARE + c.c + 1
}

fn inside(c: &Coord) -> bool {
    if c.c < 0 || c.c == SIZE_SQUARE || c.r < 0 || c.r == SIZE_SQUARE {
        return false;
    }
    return true;
}

fn neighbors(c: Coord) -> Vec<Coord> {
    let mut neighbors = vec![];
    let mut neighbor: Coord = c;

    neighbor.c -= 1;    // (c-1, r)
    if inside(&neighbor) {
        neighbors.push(neighbor);
    }
    neighbor.c += 2;    // (c+1, r)
    if inside(&neighbor) {
        neighbors.push(neighbor);
    }
    neighbor.c -= 1;    // (c, r)
    neighbor.r -= 1;    // (c, r-1)
    if inside(&neighbor) {
        neighbors.push(neighbor);
    }
    neighbor.c += 2;    // (c, r+1)
    if inside(&neighbor) {
        neighbors.push(neighbor);
    }
    
    neighbors
}

fn minimum(neighbors: Vec<Coord>) -> Coord {
    let mut min: Coord = Coord {c:SIZE_SQUARE, r:SIZE_SQUARE};

    for c in neighbors.iter() {
        if index(*c) < index(min) {
            min = *c;
        }
    }
    
    min
}

fn main() -> BError {
    let col = SIZE_SQUARE as usize;
    let row = SIZE_SQUARE as usize;

    let mut square_grid = vec![vec![0 as isize; col]; row];

    //square_grid[col][row] 0..col-1, 0..row-1
    square_grid[2][2] = 5;
    square_grid[1][4] = 1;


    let context = BTermBuilder::simple80x50()
        .with_title("Hello test")
        .build()?;

    main_loop(context, State{})
}
