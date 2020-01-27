use minifb::{Window, WindowOptions, Key};

#[derive(Clone, Copy, PartialEq)]
enum Wall {
  None,
  Wall,
}

const WIDTH: usize = 800;
const HEIGHT: usize = 800;
const CELL_HEIGHT: usize = 10;
const CELL_WIDTH: usize = 10;


struct Maze {
  cells: Vec<Cell>,
}

#[derive(Clone, Copy)]
struct Cell {
  pub left: Wall,
  pub right: Wall,
  pub top: Wall,
  pub bottom: Wall,
  pub x_pos: usize,
  pub y_pos: usize,
}

impl Cell {
  fn draw(&self, buffer: &mut Vec<u32>, top_left: usize) {
    if self.top == Wall::Wall {
      self.draw_horz_line(buffer, top_left, CELL_WIDTH);
    }
    if self.bottom == Wall::Wall {
      self.draw_horz_line(buffer, top_left + CELL_WIDTH * WIDTH, CELL_WIDTH);
    }
    if self.left == Wall::Wall {
      self.draw_vert_line(buffer, top_left, CELL_HEIGHT);
    }
    if self.right == Wall::Wall {
      self.draw_vert_line(buffer, top_left + CELL_WIDTH, CELL_HEIGHT);
    }
  }

  fn draw_horz_line(&self, buffer: &mut Vec<u32>, start_pos: usize, length: usize) {
    for pos in 0..length {
      buffer[start_pos + pos] = 0x00ffffff;
    }
  }

  fn draw_vert_line(&self, buffer: &mut Vec<u32>, start_pos: usize, length: usize) {
    for pos in 0..length {
      buffer[start_pos + (pos * WIDTH)] = 0x00ffffff;
    }
  }
}

impl Maze {
  fn new() -> Maze {
    let mut cells = vec![];

    for x in 0..79 {
      for y in 0..79 {
        cells.push(Cell {
          bottom: Wall::Wall,
          left: Wall::Wall,
          top: Wall::Wall,
          right: Wall::Wall,
          x_pos: x,
          y_pos: y,
        });
      }
    }
    Maze {
      cells
    }
  }

  fn draw(&self, buffer: &mut Vec<u32>) {
    for cell in &self.cells {
      self.draw_cell(buffer, *cell);
    }
  }

  fn draw_cell(&self, buffer: &mut Vec<u32>, cell: Cell) {
    let margin = 5 * WIDTH + 5;
    let top_left = (cell.y_pos * CELL_HEIGHT * WIDTH) + (cell.x_pos * CELL_WIDTH) + margin;
    cell.draw(buffer, top_left);
  }
}


fn main() {
  let mut window = Window::new(
    "Test - ESC to exit",
    WIDTH,
    HEIGHT,
    WindowOptions::default()).unwrap_or_else(|e| {
    panic!("{}", e);
  });


  let maze = Maze::new();
  while window.is_open() && !window.is_key_down(Key::Escape) {
    {
      let mut buffer: Vec<u32> = Vec::new();
      buffer.resize(WIDTH * HEIGHT, 0xff000000);
      maze.draw(&mut buffer);
      // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
      window.update_with_buffer(&buffer).unwrap();
    } // buffer lock ends here
  }
}

