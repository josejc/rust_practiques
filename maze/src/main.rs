mod prelude {
    // Limits of maze must be odd
    pub const MAZE_ROW: isize= 11;
    pub const MAZE_COL: isize= 11;
    pub use rand::thread_rng;
    pub use rand::Rng;
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


fn neighbors(c: Coord) -> Vec<Coord> {
    let mut neighbors = vec![];
    let mut neighbor: Coord = c;

    neighbor.c -= 1;    // (r, c-1)
    if inside(&neighbor) {
        neighbors.push(neighbor);
    }
    neighbor.c += 2;    // (r, c+1)
    if inside(&neighbor) {
        neighbors.push(neighbor);
    }
    neighbor.c -= 1;    // (r, c)
    neighbor.r -= 1;    // (r-1, c)
    if inside(&neighbor) {
        neighbors.push(neighbor);
    }
    neighbor.r += 2;    // (r+1, c)
    if inside(&neighbor) {
        neighbors.push(neighbor);
    }

    neighbors
}


fn print_square(grid: &Vec<Vec<char>>) {
    let row = MAZE_ROW as usize;

    for i in 0..row {
        println!("{:?}", grid[i]);
    }
}

// Wall status only has 2 status: O - open, C - Close
fn mark_all_walls_closed(m: &mut Vec<Vec<char>>) -> Vec<Coord> {
    let rows = MAZE_ROW as usize;
    let cols = MAZE_COL as usize;
    let mut set_rooms: Vec<Coord> = vec![];

    for row in 0..rows {
        let i:usize;
        if (row % 2) == 0 {
            i = 1;
        } else {
            i = 0;
        }
        for col in (i..cols).step_by(2) {
            // C = CLOSED
            m[row][col] = 'C';
            let room: Coord = Coord{ r: row as isize, c: col as isize};
            set_rooms.push(room);
        }
    }

    set_rooms
}

// kind return the type of the cell whith a simple char
// P - pillar
// W - wall
// R - room
fn kind(c: &Coord) -> char {
    let c_even = (c.c % 2) == 0;
    let r_even = (c.r % 2) == 0;

    if c_even && r_even {
        return 'P';
    }
    if c_even && !r_even {
        return 'W';
    }
    if !c_even && r_even {
        return 'W';
    }
    return 'R';
}

fn main() {
    let col = MAZE_COL as usize;
    let row = MAZE_ROW as usize;
    let mut set_rooms: Vec<Coord>;
    let mut path: Vec<Coord>;               // List of rooms
    let mut walls: Vec<Coord>;              // List of walls

    let mut square_grid = vec![vec!['.'; row]; col];

    set_rooms = mark_all_walls_closed(&mut square_grid);
    print_square(&square_grid);
    println!("Set of rooms: {:?}", set_rooms); 
    let index = thread_rng().gen_range(0..set_rooms.len());
    let room = set_rooms.iter().nth(index).unwrap().clone();
    //set_rooms.remove(&room);
    println!("Random Room: {:?}", room);
}
