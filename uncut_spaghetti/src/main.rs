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
    r: isize,
    c: isize,
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

fn neighbors(grid: &Vec<Vec <isize>>, c: Coord) -> Vec<Coord> {
    let mut neighbors = vec![];
    let mut neighbor: Coord = c;

    neighbor.c -= 1;    // (r, c-1)
    if inside(&neighbor) &&  (grid[neighbor.r as usize][neighbor.c as usize] == 0) {
        neighbors.push(neighbor);
    }
    neighbor.c += 2;    // (r, c+1)
    if inside(&neighbor) &&  (grid[neighbor.r as usize][neighbor.c as usize] == 0) {
        neighbors.push(neighbor);
    }
    neighbor.c -= 1;    // (r, c)
    neighbor.r -= 1;    // (r-1, c)
    if inside(&neighbor) &&  (grid[neighbor.r as usize][neighbor.c as usize] == 0) {
        neighbors.push(neighbor);
    }
    neighbor.r += 2;    // (r+1, c)
    if inside(&neighbor) &&  (grid[neighbor.r as usize][neighbor.c as usize] == 0) {
        neighbors.push(neighbor);
    }
    
    neighbors
}

fn minimum_empty(grid: &Vec<Vec<isize>>, neighbors: Vec<Coord>) -> Coord {
    let mut min: Coord = neighbors[0];

    for c in neighbors.iter() {
        if (grid[c.r as usize][c.c as usize] == 0) && (index(*c) < index(min)) {
            min = *c;
        }
    }
    
    min
}

fn print_square(grid: &Vec<Vec<isize>>) {
    let mut col = SIZE_SQUARE as usize;
    let mut row = col;

    for i in 0..row {
        println!("{:?}", grid[i]);
    }
}

fn main() -> BError {
    let mut col = SIZE_SQUARE as usize;
    let mut row = col;

    let mut square_grid = vec![vec![0 as isize; row]; col];
    let ini: Coord = Coord {r: 0, c: 2};
    let mut busy: isize = 0;
    let mut next: Coord;

    //square_grid[row][col] 0..row-1, 0..col-1
    //print_square(&square_grid);
    //println!("Neighbors: {:?}", neighbors(&square_grid, ini));
    //println!("Minimum: {:?}", minimum_empty(&square_grid, neighbors(&square_grid, ini)));
    col = ini.c as usize;
    row = ini.r as usize;
    square_grid[row][col] = 1;
    //print_square(&square_grid);
    busy += 1;
    next = minimum_empty(&square_grid, neighbors(&square_grid, ini));
    col = next.c as usize;
    row = next.r as usize;
    while square_grid[row][col] == 0 {
        square_grid[row][col] = 1;
        busy += 1;
        //println!("Neighbors: {:?}", neighbors(&square_grid, next));
        if neighbors(&square_grid, next).len() != 0 {
            next = minimum_empty(&square_grid, neighbors(&square_grid, next));
            //println!("Next: {:?}", next);
        }
        col = next.c as usize;
        row = next.r as usize;
        //println!("Square: {}", square_grid[row][col]);
    }
    print_square(&square_grid);
    if busy == SIZE_SQUARE * SIZE_SQUARE {
        println!("Square completed");
    } else {
        println!("Cells empty: {}", (SIZE_SQUARE * SIZE_SQUARE) - busy);
    }



    let context = BTermBuilder::simple80x50()
        .with_title("Hello test")
        .build()?;

    main_loop(context, State{})
}
