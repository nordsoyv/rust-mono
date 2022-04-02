// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app;
mod common;
mod generators;
mod options_window;
mod square_grid;

use crate::app::MyEguiApp;
use crate::common::{Grid, UiComponent};
use crate::options_window::OptionsWindow;
use crate::square_grid::SquareGrid2D;
use eframe::egui::Pos2;

fn main() {
  let app = MyEguiApp::new();
  let native_options = eframe::NativeOptions::default();
  eframe::run_native("My egui app", native_options, Box::new(|_cc| Box::new(app)));
}
