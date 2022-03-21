#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::egui::Key;
use eframe::{egui, epi};

use crate::epi::Frame;
// hide console window on Windows in release
use crate::epi::egui::Context;

#[derive(Default)]
struct MyEguiApp {
  options_window: OptionsWindow,
}

impl epi::App for MyEguiApp {
  fn update(&mut self, ctx: &Context, frame: &Frame) {
    if ctx.input().key_pressed(Key::Escape) {
      frame.quit();
    }
    egui::CentralPanel::default().show(ctx, |_ui| {});
    self.options_window.draw(ctx);
  }
  fn name(&self) -> &str {
    "My Egui App"
  }
}

#[derive(Default)]
struct OptionsWindow {
  width: i32,
  height: i32,
  cell_size: i32,
  difficulty: i32,
  speed: i32,
}

impl OptionsWindow {
  fn draw(&mut self, ctx: &Context) {
    egui::Window::new("Options").show(ctx, |ui| {
      ui.add(egui::Slider::new(&mut self.width, 10..=50).text("Width"));
      ui.add(egui::Slider::new(&mut self.height, 10..=50).text("Height"));
      ui.add(egui::Slider::new(&mut self.cell_size, 5..=20).text("Cell size"));
      ui.add(egui::Slider::new(&mut self.difficulty, 1..=50).text("Difficulty"));
      ui.add(egui::Slider::new(&mut self.speed, 1..=100).text("Speed"));
    });
  }
}
fn main() {
  let app = MyEguiApp::default();
  let native_options = eframe::NativeOptions::default();
  eframe::run_native(Box::new(app), native_options);
}
