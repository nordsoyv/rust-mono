use eframe::egui::{Color32, Context, Key, Sense, Ui, Vec2};
use eframe::{egui, Frame};
use std::process::Command;

use crate::djikstra::Djikstra;
use crate::generators::growing_tree::{GrowingTreeGenerator, Strategy};
use crate::generators::Generator;
use crate::grids::circle_grid::CircleGrid;
use crate::grids::hex_grid::HexGrid;
use crate::grids::square_grid::SquareGrid2D;
use crate::grids::triangle_grid::TriangleGrid2D;
use crate::grids::{Grid, GridType};
use crate::OptionsWindow;

fn save_image(bytes: &[u8], width: i32, height: i32) {
  //  let buffer = shared_buffer.lock().unwrap();
  let mut img_buf: image::RgbImage = image::ImageBuffer::new(width as u32, height as u32);

  // let mut buffer_index = 0;
  let mut bytes_index = 0;
  for (_x, _y, pixel) in img_buf.enumerate_pixels_mut() {
    //let color = buffer[buffer_index];
    let _alpha = bytes[bytes_index];
    bytes_index += 1;
    let red = bytes[bytes_index];
    bytes_index += 1;
    let green = bytes[bytes_index];
    bytes_index += 1;
    let blue = bytes[bytes_index];
    bytes_index += 1;
    *pixel = image::Rgb([red, green, blue]);
  }
  img_buf.save("image.png").unwrap();
}

pub struct MyEguiApp {
  options_window: OptionsWindow,
  maze: Box<dyn Grid>,
  custom_frame: egui::containers::Frame,
  generator: Box<dyn Generator>,
}

impl MyEguiApp {
  pub fn new() -> MyEguiApp {
    let maze = Box::new(HexGrid::new(5, 5, 20, 5.0));
    // maze.carve(CellCoord::new(5.0, 5.0), Direction::North);
    MyEguiApp {
      options_window: OptionsWindow::new(),
      maze,
      generator: Box::new(GrowingTreeGenerator::new(Strategy::LastAndRandom(10))),
      custom_frame: egui::containers::Frame {
        inner_margin: Default::default(),
        outer_margin: Default::default(),
        rounding: Default::default(),
        shadow: Default::default(),
        fill: Color32::WHITE,
        stroke: Default::default(),
      },
    }
  }
  fn draw_maze(&mut self, ui: &mut Ui) {
    let (_response, painter) = ui.allocate_painter(
      Vec2::new(ui.available_width(), ui.available_height()),
      Sense::hover(),
    );

    self.maze.draw_background(&painter);
    self.maze.draw(&painter);
  }
}

fn should_generate_new_maze(
  options_window: &OptionsWindow,
  maze: &Box<dyn Grid>,
  difficulty: i32,
  old_grid: GridType,
) -> bool {
  let width_changed = options_window.width != maze.get_num_cells_horizontal();

  let height_changed = options_window.height != maze.get_num_cells_vertical();
  let cell_size_changed = options_window.cell_size != maze.get_cell_size();
  let maring_changed = options_window.margin != maze.get_margin();
  let diff_changed = options_window.difficulty != difficulty;
  let grid_type_changed = options_window.grid_type != old_grid;
  options_window.new_maze
    || width_changed
    || height_changed
    || cell_size_changed
    || maring_changed
    || diff_changed
    || grid_type_changed
}

impl eframe::App for MyEguiApp {
  fn update(&mut self, ctx: &Context, frame: &mut Frame) {
    if ctx.input().key_pressed(Key::Escape) {
      frame.quit();
    }
    self.options_window.reset_flags();

    let old_difficulty = self.options_window.difficulty;
    let old_grid = self.options_window.grid_type;
    self.options_window.draw(ctx, &self.maze);
    if should_generate_new_maze(&self.options_window, &self.maze, old_difficulty, old_grid) {
      let mut maze: Box<dyn Grid>;
      match self.options_window.grid_type {
        GridType::Square => {
          maze = Box::new(SquareGrid2D::new(
            self.options_window.width,
            self.options_window.height,
            self.options_window.cell_size,
            0,
            self.options_window.margin as f32,
          ));
        }
        GridType::Hex => {
          maze = Box::new(HexGrid::new(
            self.options_window.width,
            self.options_window.height,
            self.options_window.cell_size,
            self.options_window.margin as f32,
          ));
        }
        GridType::Triangle => {
          maze = Box::new(TriangleGrid2D::new(
            self.options_window.width,
            self.options_window.height,
            self.options_window.cell_size,
            self.options_window.margin as f32,
          ));
        }
        GridType::Circle => {
          maze = Box::new(CircleGrid::new(
            self.options_window.width,
            self.options_window.cell_size,
            self.options_window.margin,
          ))
        }
      }

      maze.init();
      self.maze = maze;
      self.generator = Box::new(GrowingTreeGenerator::new(Strategy::LastN(
        self.options_window.difficulty,
      )));
      self.generator.init(&mut self.maze);
    }

    if !self.generator.done() {
      for _ in 0..self.options_window.speed {
        self.generator.generate_step(&mut self.maze);
        if self.generator.done() {
          self.maze.find_dead_ends();
          break;
        }
      }
      if !self.generator.done() {
        ctx.request_repaint();
      }
    }
    if self.options_window.show_solution && self.generator.done() {
      if !self.maze.has_solution() {
        Djikstra::new().run(&mut self.maze);
      }
    }
    if !self.options_window.show_solution && self.generator.done() {
      if self.maze.has_solution() {
        self.maze.clear_solution();
      }
    }
    if self.options_window.remove_deadends && self.generator.done() {
      self.maze.remove_dead_end();
      self.maze.clear_solution();
      self.maze.find_dead_ends();
    }
    let response = egui::CentralPanel::default()
      .frame(self.custom_frame)
      .show(ctx, |ui| {
        self.draw_maze(ui);
      });
    if self.options_window.take_screenshot || self.options_window.print {
      frame.copy_pixels = Some(response.response.rect);
    }
  }

  fn get_pixel_data(&self, bytes: &[u8], width: i32, height: i32) {
    save_image(bytes, width, height);
    if self.options_window.print {
      Command::new("mspaint")
        .args(&["/pt", "image.png"])
        .output()
        .expect("Failed to execute process");
    }
  }
}
