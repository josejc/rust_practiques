#![warn(clippy::all, clippy::pedantic)]
mod prelude {
    // Limits of maze must be odd
    pub const MAZE_ROW: isize= 59;
    pub const MAZE_COL: isize= 101;
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

fn print_maze(grid: &Vec<Vec<char>>) {
    let row = MAZE_ROW as usize;
    let col = MAZE_COL as usize;

    for i in 0..row {
        let mut row_print = String::new();
        for j in 0..col {
            if grid[i][j] == 'O' || grid[i][j] == 'R' {
                row_print.push(' ');
            } else {
                row_print.push('▓');
            }
        }
        println!("{}", row_print);
    }
}

// Wall status only has 2 status: O - open, C - Close
fn mark_all_walls_closed(m: &mut Vec<Vec<char>>) {
    let rows = MAZE_ROW as usize;
    let cols = MAZE_COL as usize;

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
        }
    }
}

fn set_all_rooms(m: &mut Vec<Vec<char>>) -> Vec<Coord> {
    let mut set_rooms: Vec<Coord> = vec![];
    let rows = MAZE_ROW as usize;
    let cols = MAZE_COL as usize;

    for row in (1..rows).step_by(2) {
        for col in (1..cols).step_by(2) {
            let room: Coord = Coord{ r: row as isize, c: col as isize};
            m[row][col] = 'R';
            set_rooms.push(room);
        }
    }

    set_rooms
}


fn set_all_pillars(m: &mut Vec<Vec<char>>) {
    let rows = MAZE_ROW as usize;
    let cols = MAZE_COL as usize;

    for row in (0..rows).step_by(2) {
        for col in (0..cols).step_by(2) {
            m[row][col] = 'P';
        }
    }
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

// Find the rooms adjacent to the wall, checking neighbors cells
fn rooms_adjacent(neighbors: Vec<Coord>) -> Vec<Coord> {
    let mut rooms: Vec<Coord> = vec![];

    for neighbor in neighbors.iter() {
        if kind(neighbor) == 'R' {
            rooms.push(*neighbor);
        }
    }

    rooms
}

// Number of adjacent rooms in the path
fn adjacent_rooms_path(path: &Vec<Coord>, neighbors: &Vec<Coord>) -> i32 {
    let mut i: i32 = 0;
    
    for r in neighbors.iter() {
        for p in path.iter() {
            if (r.c == p.c) && (r.r == p.r) {
                i += 1;
                break;
            }
        }
    }

    i
}

fn open_wall(w: &Coord, grid: &mut Vec<Vec<char>>) {
    grid[w.r as usize][w.c as usize] = 'O';
}

fn unvisited_room(path: &Vec<Coord>, rooms_ad: &Vec<Coord>) -> Coord {
    let mut unvisited = Coord {r: MAZE_ROW, c: MAZE_COL};
    let mut found: bool = false;

    for r in rooms_ad.iter() {
        for p in path.iter() {
            if (r.c == p.c) && (r.r == p.r) {
                found = true;
                break;
            }
        }
        if !found {
            unvisited.r = r.r;
            unvisited.c = r.c;
            break;
        }  else {
            found = false;
        }
    }

    if unvisited.r == MAZE_ROW {
        panic!("Error unvisited");
    }
    unvisited
}

fn add_walls(walls: &mut Vec<Coord>, neighbors: Vec<Coord>) {
    let mut n = neighbors.clone();

    walls.append(&mut n);
}

// Implementation Prim's Algorithm
// 1. Mark all walls as closed.
// 2. Select a room from the set of rooms, and add it to the "path".
// 3. Add the four walls of the room to the "wall list". This is the list that we keep processing until it is empty.
// 4. While the wall list is not empty:
//      4.1 Select a wall from the list.
//      4.2 Find the rooms adjacent to the wall.
//      4.3 If there are two adjacent rooms, and exactly one ofthem is not in the path:
//              4.3.1 Mark the wall as "Open".
//              4.3.2 Add the unvisited room to the path.
//              4.3.3 Add the walls adjacent to the unvisited room to the wall list.
//      4.4 Remove the wall from the wall list.
//
fn main() {
    let col = MAZE_COL as usize;
    let row = MAZE_ROW as usize;
    let mut set_rooms: Vec<Coord>;
    let mut path: Vec<Coord> = vec![];                  // List of rooms
    let mut walls: Vec<Coord>;                          // List of walls

    let mut square_grid = vec![vec!['.'; col]; row];

    set_all_pillars(&mut square_grid);
    mark_all_walls_closed(&mut square_grid);                    // 1.
    set_rooms = set_all_rooms(&mut square_grid);
    print_square(&square_grid);
    //println!("Set of rooms: {:?}", set_rooms); 
    let index = thread_rng().gen_range(0..set_rooms.len());     // 2.
    let room = set_rooms.iter().nth(index).unwrap().clone();
    set_rooms.remove(index);
    path.push(room);
    //println!("Random Room: {:?}", room);
    //println!("Set of rooms: {:?}", set_rooms); 
    walls = neighbors(room);                                    // 3.
    //println!("Wall list: {:?}", walls);
    while !walls.is_empty() {                                   // 4.
        let index = thread_rng().gen_range(0..walls.len());     
        let wall = walls.iter().nth(index).unwrap().clone();    // 4.1
        //println!("Wall: {:?}", wall);
        let rooms_ad = rooms_adjacent(neighbors(wall));         // 4.2
        //println!("Rooms adjacent: {:?}", rooms_ad);
        //println!("Number of adjacent rooms: {:?}", rooms_ad.len());
        //println!("Number of adjacent rooms in path: {:?}", adjacent_rooms_path(&path, &rooms_ad));    
        if (rooms_ad.len() == 2) && 
            (adjacent_rooms_path(&path, &rooms_ad) == 1) {      // 4.3
            open_wall(&wall, &mut square_grid);                 // 4.3.1
            let unvisited = unvisited_room(&path, &rooms_ad);          
            //println!("Unvisited: {:?}", unvisited);
            path.push(unvisited);                               // 4.3.2
            add_walls(&mut walls, neighbors(unvisited));        // 4.3.3
            //println!("Walls: {:?}", walls);
        }
        walls.remove(index);                                    // 4.4 
        //println!("Path: {:?}", path);
        print_maze(&square_grid);
    }
}
