// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app;
mod common;
mod djikstra;
mod generators;
mod grids;
mod options_window;
mod slider_with_text;

use crate::app::MyEguiApp;
use crate::options_window::OptionsWindow;

fn main() {
  let app = MyEguiApp::new();
  let native_options = eframe::NativeOptions::default();
  eframe::run_native("MazeMaker", native_options, Box::new(|_cc| Box::new(app)));
}
