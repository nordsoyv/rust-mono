use eframe::egui::{Context, Label, Response, Slider, Ui, Widget, WidgetText};
use eframe::{egui, emath};
use std::ops::RangeInclusive;

use crate::common::{GridType, UiComponent};

pub struct OptionsWindow {
  pub width: i32,
  pub height: i32,
  pub cell_size: i32,
  pub difficulty: i32,
  pub speed: i32,
  pub take_screenshot: bool,
  pub margin: i32,
  pub grid_type: GridType,
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
      margin: 5,
      grid_type: GridType::Square,
    }
  }
}

struct SliderWithText<'a, Num: emath::Numeric> {
  text: WidgetText,
  value: &'a mut Num,
  start: Num,
  stop: Num,
}

impl<'a, Num: emath::Numeric> SliderWithText<'a, Num> {
  fn new(
    text: impl Into<WidgetText>,
    value: &'a mut Num,
    start: Num,
    stop: Num,
  ) -> SliderWithText<'a, Num> {
    SliderWithText {
      text: text.into(),
      value,
      stop,
      start,
    }
  }
}

impl<'a, Num: emath::Numeric> Widget for &mut SliderWithText<'a, Num> {
  fn ui(self, ui: &mut Ui) -> Response {
    let SliderWithText {
      value,
      text,
      //      start,
      //   stop,
      ..
    } = self;
    // let mut value = self.value;
    // let text = &self.text;
    // start,
    // stop,

    // text,
    // } = self;

    ui.horizontal(|ui| {
      let response = ui.label(text.text());
      ui.add_space(50.0 - response.rect.width());
      ui.add(egui::Slider::new(*value, self.start..=self.stop));
      response
    })
    .response
  }
}

impl UiComponent for OptionsWindow {
  fn draw(&mut self, ctx: &Context) {
    egui::SidePanel::right("Options").show(ctx, |ui| {
      egui::ComboBox::from_label("Grid type")
        .selected_text(format!("{:?}", self.grid_type))
        .show_ui(ui, |ui| {
          ui.selectable_value(&mut self.grid_type, GridType::Square, "Square");
          ui.selectable_value(&mut self.grid_type, GridType::Hex, "Hex");
        });
      SliderWithText::new("Width:", &mut self.width, 10, 50).ui(ui);
      SliderWithText::new("Height:", &mut self.height, 10, 50).ui(ui);
      SliderWithText::new("Cell size:", &mut self.cell_size, 5, 20).ui(ui);
      SliderWithText::new("Difficulty:", &mut self.difficulty, 1, 50).ui(ui);
      SliderWithText::new("Speed:", &mut self.speed, 1, 100).ui(ui);
      SliderWithText::new("Margin:", &mut self.margin, 0, 10).ui(ui);
      ui.horizontal(|ui| {
        if ui.button("Take screenshot").clicked() {
          self.take_screenshot = true;
        }
      });
    });
  }
}
