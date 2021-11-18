mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 10;
    pub const SCREEN_HEIGHT: i32 = 10;
}

use prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello world...")
    }
}

fn main() -> BError {
    let col = 5;
    let row = 5;

    let mut square_grid = vec![vec![0 as isize; col]; row];

    //square_grid[col][row] 0..col-1, 0..row-1
    square_grid[2][2] = 5;
    square_grid[1][4] = 1;


    let context = BTermBuilder::simple80x50()
        .with_title("Hello test")
        .build()?;

    main_loop(context, State{})
}