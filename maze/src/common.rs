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

pub const CELL_HEIGHT: i32 = 40;
pub const CELL_WIDTH: i32 = 40;
pub const CELL_INSET: i32 = 5;
pub const NUM_CELLS: i32 = 20;
pub const WIDTH: i32 = (CELL_WIDTH * NUM_CELLS) + (MARGIN * 2);
pub const HEIGHT: i32 = (CELL_WIDTH * NUM_CELLS) + (MARGIN * 2);
pub const MARGIN: i32 = 10;
