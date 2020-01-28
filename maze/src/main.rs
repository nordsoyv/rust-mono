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

struct Canvas {
  width : usize,
  height : usize,
  buffer : Vec<u32>,
}

impl Canvas {
  fn clear (&mut self) {
    self.buffer = Vec::new();
    self.buffer.resize(self.width * self.height, 0xff000000);
  }

  fn draw_vert_line(&mut self, start_pos: usize, length: usize) {
    for pos in 0..length {
      self.buffer[start_pos + (pos * self.width)] = 0x00ffffff;
    }
  }

  fn draw_horz_line(&mut self,  start_pos: usize, length: usize) {
    for pos in 0..length {
      self.buffer[start_pos + pos] = 0x00ffffff;
    }
  }
}

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
  fn draw(&self, canvas: &mut Canvas, top_left: usize) {
    if self.top == Wall::Wall {
      canvas.draw_horz_line( top_left, CELL_WIDTH);
    }
    if self.bottom == Wall::Wall {
      canvas.draw_horz_line( top_left + CELL_WIDTH * WIDTH, CELL_WIDTH);
    }
    if self.left == Wall::Wall {
      canvas.draw_vert_line( top_left, CELL_HEIGHT);
    }
    if self.right == Wall::Wall {
      canvas.draw_vert_line( top_left + CELL_WIDTH, CELL_HEIGHT);
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

  fn draw(&self, canvas: &mut Canvas) {
    for cell in &self.cells {
      self.draw_cell(canvas, *cell);
    }
  }

  fn draw_cell(&self, canvas: &mut Canvas, cell: Cell) {
    let margin = 5 * WIDTH + 5;
    let top_left = (cell.y_pos * CELL_HEIGHT * WIDTH) + (cell.x_pos * CELL_WIDTH) + margin;
    cell.draw(canvas, top_left);
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
//      let mut buffer: Vec<u32> = Vec::new();
//      buffer.resize(WIDTH * HEIGHT, 0xff000000);
      let mut canvas = Canvas {
        width : WIDTH,
        height : HEIGHT,
        buffer: vec![]
      };
      canvas.clear();
      maze.draw(&mut canvas);
      // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
      window.update_with_buffer(&canvas.buffer).unwrap();
    } // buffer lock ends here
  }
}

