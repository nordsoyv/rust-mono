#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
  North,
  East,
  South,
  West,
}

pub const CELL_HEIGHT: i32 = 15;
pub const CELL_WIDTH: i32 = 15;
pub const NUM_CELLS: i32 = 50;
pub const WIDTH: i32 = (CELL_WIDTH * NUM_CELLS) + (MARGIN * 2);
pub const HEIGHT: i32 = (CELL_WIDTH * NUM_CELLS) + (MARGIN * 2);
pub const MARGIN: i32 = 10;
pub const CELL_ACTIVE_COLOR : u32= 0xffffff00;
