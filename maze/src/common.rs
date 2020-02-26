#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Wall {
  None,
  Wall,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
  North,
  East,
  South,
  West,
}

pub const CELL_HEIGHT: i32 = 25;
pub const CELL_WIDTH: i32 = 25;
pub const NUM_CELLS: i32 = 40;
pub const WIDTH: i32 = (CELL_WIDTH * NUM_CELLS) ;
//pub const WIDTH: i32 = 400;
pub const HEIGHT: i32 = (CELL_WIDTH * NUM_CELLS);
//pub const HEIGHT: i32 = 400;
pub const MARGIN: i32 = 0;
