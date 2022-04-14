use eframe::egui::{Response, Ui, Widget, WidgetText};
use eframe::{egui, emath};
use std::ops::RangeInclusive;

pub struct SliderWithText<'a, Num: emath::Numeric> {
  text: WidgetText,
  value: &'a mut Num,
  range: RangeInclusive<Num>,
}

impl<'a, Num: emath::Numeric> SliderWithText<'a, Num> {
  pub fn new(
    text: impl Into<WidgetText>,
    value: &'a mut Num,
    range: RangeInclusive<Num>,
  ) -> SliderWithText<'a, Num> {
    SliderWithText {
      text: text.into(),
      value,
      range,
    }
  }
}

impl<'a, Num: emath::Numeric> Widget for &mut SliderWithText<'a, Num> {
  fn ui(self, ui: &mut Ui) -> Response {
    let SliderWithText { value, text, .. } = self;
    ui.horizontal(|ui| {
      let response = ui.label(text.text());
      ui.add_space(50.0 - response.rect.width());
      let resp2 = ui.add(egui::Slider::new(*value, self.range.clone()));
      // dbg!(resp2.rect);
      response.union(resp2);
      response
    })
    .response
  }
}
