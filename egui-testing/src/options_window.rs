use eframe::egui;
use eframe::egui::Context;

use crate::common::UiComponent;

pub struct OptionsWindow {
  pub width: i32,
  pub height: i32,
  pub cell_size: i32,
  pub difficulty: i32,
  pub speed: i32,
  pub generate_new: bool,
  pub take_screenshot: bool,
  pub margin: i32,
}

impl OptionsWindow {
  pub fn new() -> OptionsWindow {
    OptionsWindow {
      height: 10,
      width: 10,
      cell_size: 10,
      generate_new: false,
      speed: 1,
      difficulty: 1,
      take_screenshot: false,
      margin: 0,
    }
  }
}

impl UiComponent for OptionsWindow {
  fn draw(&mut self, ctx: &Context) {
    egui::SidePanel::right("Options").show(ctx, |ui| {
      ui.add(egui::Slider::new(&mut self.width, 10..=50).text("Width"));
      ui.add(egui::Slider::new(&mut self.height, 10..=50).text("Height"));
      ui.add(egui::Slider::new(&mut self.cell_size, 5..=20).text("Cell size"));
      ui.add(egui::Slider::new(&mut self.difficulty, 1..=50).text("Difficulty"));
      ui.add(egui::Slider::new(&mut self.speed, 1..=100).text("Speed"));
      ui.add(egui::Slider::new(&mut self.margin, 0..=10).text("Margin"));
      ui.horizontal(|ui| {
        if ui.button("Generate").clicked() {
          self.generate_new = true;
        }
        if ui.button("Take screenshot").clicked() {
          self.take_screenshot = true;
        }
      });
    });
  }
}
