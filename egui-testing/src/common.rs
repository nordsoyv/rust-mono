use eframe::egui::Context;

pub trait UiComponent {
  fn draw(&mut self, ctx: &Context);
}

pub fn is_odd(num: f32) -> bool {
  return (num as i32) & 1 != 0;
}
