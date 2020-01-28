use minifb::{Window, WindowOptions, Key};
use rand::distributions::{Uniform, Distribution};

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
  width: usize,
  height: usize,
  buffer: Vec<u32>,
}

impl Canvas {
  fn clear(&mut self) {
    self.buffer = Vec::new();
    self.buffer.resize(self.width * self.height, 0xff000000);
  }

  fn draw_vertical_line(&mut self, start_x: usize, start_y: usize, length: usize) {
    let margin = 5 * self.width + 5;
    let top_left = (start_y * self.width) + (start_x) + margin;
    for pos in 0..length {
      self.buffer[top_left + (pos * self.width)] = 0x00ffffff;
    }
  }
  fn draw_horizontal_line(&mut self, start_x: usize, start_y: usize, length: usize) {
    let margin = 5 * self.width + 5;
    let top_left = (start_y * self.width) + (start_x) + margin;
    for pos in 0..length {
      self.buffer[top_left + pos] = 0x00ffffff;
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

fn random_wall() -> Wall {
  let mut rng = rand::thread_rng();
  let random = Uniform::from(0..10);
  if random.sample(&mut rng) < 7 {
    Wall::None
  }else {
    Wall::Wall
  }
}

impl Maze {
  fn new() -> Maze {
    let mut cells = vec![];

    for x in 0..79 {
      for y in 0..79 {
        cells.push(Cell {
          bottom: random_wall(),
          left: random_wall(),
          top: random_wall(),
          right: random_wall(),
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
    cell.draw(canvas);
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

