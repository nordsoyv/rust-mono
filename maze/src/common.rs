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

pub const BACKGROUND_COLOR: u32 = 0x00ffffff;
pub const FOREGROUND_COLOR: u32 = 0xff000000;
pub const CELL_HEIGHT: i32 = 20;
pub const CELL_WIDTH: i32 = 20;
pub const NUM_CELLS: i32 = 80;
pub const WIDTH: i32 = (CELL_WIDTH * NUM_CELLS) + 10;
pub const HEIGHT: i32 = (CELL_WIDTH * NUM_CELLS) + 10;
