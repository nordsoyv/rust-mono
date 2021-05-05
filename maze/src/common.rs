#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
  North,
  NorthEast,
  NorthWest,
  East,
  South,
  SouthEast,
  SouthWest,
  West,
}

pub const MARGIN: i32 = 10;
pub const CELL_ACTIVE_COLOR: u32 = 0xffffff00;
