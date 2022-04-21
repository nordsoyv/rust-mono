use crate::grids::CellCoord;

#[allow(dead_code)]
pub enum CellConnectionStatus {
  Open,
  Wall,
}

#[allow(dead_code)]
pub struct CellConnection {
  pub cell_1: CellCoord,
  pub cell_2: CellCoord,
  status: CellConnectionStatus,
}

impl CellConnectionStatus {
  #[allow(dead_code)]
  pub fn is_wall(&self) -> bool {
    match *self {
      CellConnectionStatus::Open => false,
      CellConnectionStatus::Wall => true,
    }
  }
  #[allow(dead_code)]
  pub fn is_open(&self) -> bool {
    match *self {
      CellConnectionStatus::Open => true,
      CellConnectionStatus::Wall => false,
    }
  }
}

impl CellConnection {
  #[allow(dead_code)]
  pub fn new(c1: CellCoord, c2: CellCoord) -> CellConnection {
    CellConnection {
      cell_1: c1,
      cell_2: c2,
      status: CellConnectionStatus::Wall,
    }
  }
  #[allow(dead_code)]
  pub fn is_open(&self) -> bool {
    self.status.is_open()
  }
  #[allow(dead_code)]
  pub fn is_wall(&self) -> bool {
    self.status.is_wall()
  }
}
