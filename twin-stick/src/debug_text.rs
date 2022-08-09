use bevy::prelude::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Eq, PartialEq, Hash)]
pub enum DebugTextType {
  Fps,
  GamePad,
}

#[derive(Component)]
struct DebugText;

pub struct DebugTextPlugin;

impl Plugin for DebugTextPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(debug_text_init)
      .add_system_to_stage(CoreStage::PreUpdate, debug_text_pre_frame)
      .add_system_to_stage(CoreStage::PostUpdate, debug_text_post_frame);
  }
}

static TEXTS: Lazy<Mutex<HashMap<DebugTextType, String>>> =
  Lazy::new(|| Mutex::new(HashMap::new()));

fn debug_text_init(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands
    .spawn_bundle(
      TextBundle::from_section(
        "",
        TextStyle {
          font: asset_server.load("fonts/FiraMono-Medium.ttf"),
          font_size: 20.0,
          color: Color::WHITE,
        },
      )
      .with_style(Style {
        align_self: AlignSelf::FlexEnd,
        ..default()
      }),
    )
    .insert(DebugText);
}

pub fn debug(debug_type: DebugTextType, text: String) {
  TEXTS.lock().unwrap().insert(debug_type, text);
}

fn debug_text_pre_frame() {
  TEXTS.lock().unwrap().clear();
}

fn debug_text_post_frame(mut query: Query<&mut Text, With<DebugText>>) {
  let mut collected = String::new();
  let texts = TEXTS.lock().unwrap();
  if texts.contains_key(&DebugTextType::Fps) {
    collected.push_str(&format!(
      "FPS: {}\n",
      texts.get(&DebugTextType::Fps).unwrap()
    ));
  }
  if texts.contains_key(&DebugTextType::GamePad) {
    collected.push_str(&format!(
      "Gamepad: {}\n",
      texts.get(&DebugTextType::GamePad).unwrap()
    ));
  }
  for mut text in &mut query {
    text.sections[0].value = collected.clone();
  }
}
