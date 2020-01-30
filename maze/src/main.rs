use minifb::{Window, WindowOptions, Key};
use rand::distributions::{Uniform, Distribution};
use std::convert::TryFrom;
use rand::prelude::ThreadRng;

const WIDTH: i32 = 810;
const HEIGHT: i32 = 810;
const CELL_HEIGHT: i32 = 10;
const CELL_WIDTH: i32 = 10;
const NUM_CELLS: i32 = 80;
const BACKGROUND_COLOR: u32 = 0x00ffffff;
const FOREGROUND_COLOR: u32 = 0xff000000;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Wall {
  None,
  Wall,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
  North,
  East,
  South,
  West,
}

struct Canvas {
  width: i32,
  height: i32,
  buffer: Vec<u32>,
}

impl Canvas {
  fn clear(&mut self) {
    self.buffer = Vec::new();
    let size = usize::try_from(self.width * self.height).unwrap();
    self.buffer.resize(size, BACKGROUND_COLOR);
  }

  fn draw_vertical_line(&mut self, start_x: i32, start_y: i32, length: i32) {
    assert!(length > 0);
    assert!(start_x >= 0);
    assert!(start_y >= 0);
    let margin = 5 * self.width + 5;
    let top_left = (start_y * self.width) + (start_x) + margin;
    for pos in 0..length {
      self.buffer[(top_left + (pos * self.width)) as usize] = FOREGROUND_COLOR;
    }
  }
  fn draw_horizontal_line(&mut self, start_x: i32, start_y: i32, length: i32) {
    assert!(length > 0);
    assert!(start_x >= 0);
    assert!(start_y >= 0);
    let margin = 5 * self.width + 5;
    let top_left = (start_y * self.width) + (start_x) + margin;
    for pos in 0..length {
      self.buffer[(top_left + pos) as usize] = FOREGROUND_COLOR;
    }
  }
}

struct Maze {
  cells: Vec<Cell>,
  rng: ThreadRng,
  random: Uniform<f32>,
}

impl Maze {
  fn new() -> Maze {
    Maze {
      rng: rand::thread_rng(),
      random: Uniform::from(0.0f32..1.0f32),
      cells: vec![],
    }
  }

  fn get_direction(&mut self) -> Direction {
    let mut d = self.random.sample(&mut self.rng);
    d = d * 4.0f32;
    if d < 1.0f32 {
      Direction::North
    } else if d < 2.0f32 {
      Direction::East
    } else if d < 3.0f32 {
      Direction::South
    } else {
      Direction::West
    }
  }

  fn get_cell(&self, x: i32, y: i32) -> &Cell {
    let index = y * NUM_CELLS + x;
    return &self.cells[index as usize];
  }

  fn get_mut_cell(&mut self, x: i32, y: i32) -> &mut Cell {
    let index = y * NUM_CELLS + x;
    return &mut self.cells[index as usize];
  }

  fn carve(&mut self, x_start: i32, y_start: i32, dir: Direction) {
    let x_end = match dir {
      Direction::West => x_start - 1,
      Direction::East => x_start + 1,
      _ => x_start
    };
    let y_end = match dir {
      Direction::South => y_start + 1,
      Direction::North => y_start - 1,
      _ => y_start
    };


    assert_eq!((x_start - x_end).abs() + (y_start - y_end).abs(), 1);

    if x_start < 0 || x_end < 0
      || y_start < 0 || y_end < 0
      || x_start > NUM_CELLS || x_end > NUM_CELLS
      || y_start > NUM_CELLS || y_end > NUM_CELLS {
      return;
    }
    {
      let start_cell = self.get_mut_cell(x_start, y_start);
      match dir {
        Direction::North => {
          start_cell.top = Wall::None;
        }
        Direction::South => {
          start_cell.bottom = Wall::None;
        }
        Direction::East => {
          start_cell.right = Wall::None;
        }
        Direction::West => {
          start_cell.left = Wall::None;
        }
      }
    }
    {
      let end_cell = self.get_mut_cell(x_end, y_end);
      match dir {
        Direction::North => {
          end_cell.bottom = Wall::None;
        }
        Direction::South => {
          end_cell.top = Wall::None;
        }
        Direction::East => {
          end_cell.left = Wall::None;
        }
        Direction::West => {
          end_cell.right = Wall::None;
        }
      }
    }
  }

  fn generate(&mut self) {
    for y in 0..NUM_CELLS {
      for x in 0..NUM_CELLS {
        self.cells.push(Cell {
          bottom: Wall::Wall,
          left: Wall::Wall,
          top: Wall::Wall,
          right: Wall::Wall,
          x_pos: x,
          y_pos: y,
        });
      }
    }

    let mut start = self.get_mut_cell(5, 0);
    start.top = Wall::None;
    let mut end = self.get_mut_cell(5, 79);
    end.bottom = Wall::None;
  }

  fn draw(&self, canvas: &mut Canvas) {
    for cell in &self.cells {
      self.draw_cell(canvas, *cell);
    }
  }

  fn draw_cell(&self, canvas: &mut Canvas, cell: Cell) {
    cell.draw(canvas);
  }
}

#[derive(Clone, Copy, Debug)]
struct Cell {
  pub left: Wall,
  pub right: Wall,
  pub top: Wall,
  pub bottom: Wall,
  pub x_pos: i32,
  pub y_pos: i32,
}

impl Cell {
  fn draw(&self, canvas: &mut Canvas) {
    if self.top == Wall::Wall {
      canvas.draw_horizontal_line(self.x_pos * CELL_WIDTH, self.y_pos * CELL_HEIGHT, CELL_WIDTH);
    }
    if self.bottom == Wall::Wall {
      canvas.draw_horizontal_line(self.x_pos * CELL_WIDTH, (self.y_pos + 1) * CELL_HEIGHT, CELL_WIDTH);
    }
    if self.left == Wall::Wall {
      canvas.draw_vertical_line(self.x_pos * CELL_WIDTH, self.y_pos * CELL_HEIGHT, CELL_HEIGHT);
    }
    if self.right == Wall::Wall {
      canvas.draw_vertical_line((self.x_pos + 1) * CELL_WIDTH, self.y_pos * CELL_HEIGHT, CELL_HEIGHT);
    }
  }
}

/*
fn random_wall() -> Wall {
  let mut rng = rand::thread_rng();
  let random = Uniform::from(0..10);
  if random.sample(&mut rng) < 7 {
    Wall::None
  } else {
    Wall::Wall
  }
}
*/


fn main() {
  let mut window = Window::new(
    "Test - ESC to exit",
    usize::try_from(WIDTH).unwrap(),
    usize::try_from(HEIGHT).unwrap(),
    WindowOptions::default()).unwrap_or_else(|e| {
    panic!("{}", e);
  });


  let mut maze = Maze::new();
  maze.generate();
  maze.carve(10, 1, Direction::South);
  while window.is_open() && !window.is_key_down(Key::Escape) {
    {
      let mut canvas = Canvas {
        width: WIDTH,
        height: HEIGHT,
        buffer: vec![],
      };
      canvas.clear();
      maze.draw(&mut canvas);
      // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
      window.update_with_buffer(&canvas.buffer).unwrap();
    } // buffer lock ends here
  }
}

