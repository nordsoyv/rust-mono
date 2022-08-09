use crate::debug_text::{debug, DebugTextPlugin, DebugTextType};
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

mod debug_text;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

struct GreetTimer(Timer);

fn add_people(mut commands: Commands) {
  commands.spawn_bundle(Camera2dBundle::default());
  commands
    .spawn()
    .insert(Person)
    .insert(Name("Elaina Proctor".to_string()));
  commands
    .spawn()
    .insert(Person)
    .insert(Name("Renzo Hume".to_string()));
  commands
    .spawn()
    .insert(Person)
    .insert(Name("Zayna Nieves".to_string()));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
  if timer.0.tick(time.delta()).just_finished() {
    for name in query.iter() {
      println!("hello {}!", name.0);
    }
  }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
      .add_startup_system(add_people)
      .add_system(greet_people);
  }
}

fn gamepad_system(
  gamepads: Res<Gamepads>,
  button_inputs: Res<Input<GamepadButton>>,
  button_axes: Res<Axis<GamepadButton>>,
  axes: Res<Axis<GamepadAxis>>,
) {
  for gamepad in gamepads.iter().cloned() {
    if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South)) {
      info!("{:?} just pressed South", gamepad);
    } else if button_inputs.just_released(GamepadButton::new(gamepad, GamepadButtonType::South)) {
      info!("{:?} just released South", gamepad);
    }

    let right_trigger = button_axes
      .get(GamepadButton::new(
        gamepad,
        GamepadButtonType::RightTrigger2,
      ))
      .unwrap();
    if right_trigger.abs() > 0.01 {
      info!("{:?} RightTrigger2 value is {}", gamepad, right_trigger);
    }

    let left_stick_x = axes
      .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
      .unwrap();
    let left_stick_y = axes
      .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
      .unwrap();
    let right_stick_x = axes
      .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickX))
      .unwrap();
    let right_stick_y = axes
      .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickY))
      .unwrap();
    let gamepad_info = format!(
      "{:+.4} {:+.4}, {:+.4} {:+.4}",
      left_stick_x, left_stick_y, right_stick_x, right_stick_y
    );
    debug(DebugTextType::GamePad, gamepad_info);
  }
}

fn fps_update_system(diagnostics: Res<Diagnostics>) {
  if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
    if let Some(average) = fps.average() {
      let fps_info = format!("{average:.2}");
      debug(DebugTextType::Fps, fps_info);
    }
  }
}

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(HelloPlugin)
    .add_plugin(DebugTextPlugin)
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_system(gamepad_system)
    .add_system(fps_update_system)
    .add_system(bevy::window::close_on_esc)
    .run();
}
