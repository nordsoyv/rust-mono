use eframe::egui;
use eframe::egui::{Context, Widget};

use crate::grids::{Grid, GridType};
use crate::slider_with_text::SliderWithText;

pub struct OptionsWindow {
  pub width: i32,
  pub height: i32,
  pub cell_size: i32,
  pub difficulty: i32,
  pub speed: i32,
  pub take_screenshot: bool,
  pub new_maze: bool,
  pub margin: i32,
  pub grid_type: GridType,
  pub show_solution: bool,
  pub remove_deadends: bool,
}

impl OptionsWindow {
  pub fn new() -> OptionsWindow {
    OptionsWindow {
      height: 10,
      width: 10,
      cell_size: 10,
      speed: 1,
      difficulty: 1,
      take_screenshot: false,
      new_maze: false,
      margin: 5,
      grid_type: GridType::Square,
      show_solution: false,
      remove_deadends: false,
    }
  }
}

impl OptionsWindow {
  pub fn draw(&mut self, ctx: &Context, maze: &Box<dyn Grid>) {
    egui::SidePanel::right("Options").show(ctx, |ui| {
      ui.horizontal(|ui| {
        let response = ui.label("Grid type:");
        ui.add_space(50.0 - response.rect.width());
        egui::ComboBox::from_label("")
          .selected_text(format!("{:?}", self.grid_type))
          .width(135.0)
          .show_ui(ui, |ui| {
            ui.selectable_value(&mut self.grid_type, GridType::Square, "Square");
            ui.selectable_value(&mut self.grid_type, GridType::Hex, "Hex");
            ui.selectable_value(&mut self.grid_type, GridType::Triangle, "Triangle");
          });
      });

      SliderWithText::new("Width:", &mut self.width, 10..=50).ui(ui);
      SliderWithText::new("Height:", &mut self.height, 10..=50).ui(ui);
      SliderWithText::new("Cell size:", &mut self.cell_size, 5..=20).ui(ui);
      SliderWithText::new("Difficulty:", &mut self.difficulty, 1..=50).ui(ui);
      SliderWithText::new("Speed:", &mut self.speed, 1..=100).ui(ui);
      SliderWithText::new("Margin:", &mut self.margin, 0..=10).ui(ui);
      ui.checkbox(&mut self.show_solution, "Show solution");
      ui.horizontal(|ui| {
        if ui.button("Take screenshot").clicked() {
          self.take_screenshot = true;
        }
        if ui.button("New").clicked() {
          self.new_maze = true;
        }
      });
      ui.horizontal(|ui| {
        ui.label(format!("Deadends: {}", maze.count_dead_ends()));
        if ui.button("Remove deadend").clicked() {
          self.remove_deadends = true;
        }
      });
    });
  }
}
