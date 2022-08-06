use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

struct GreetTimer(Timer);

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
struct GamePadText;

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;

fn add_people(mut commands: Commands, asset_server: Res<AssetServer>) {
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
      .with_text_alignment(TextAlignment::TOP_LEFT)
      .with_style(Style {
        align_self: AlignSelf::FlexEnd,
        position_type: PositionType::Absolute,
        position: UiRect {
          bottom: Val::Px(5.0),
          right: Val::Px(15.0),
          ..default()
        },
        ..default()
      }),
    )
    .insert(GamePadText);
  // Text with multiple sections
  commands
    .spawn_bundle(
      // Create a TextBundle that has a Text with a list of sections.
      TextBundle::from_sections([
        TextSection::new(
          "FPS: ",
          TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 60.0,
            color: Color::WHITE,
          },
        ),
        TextSection::from_style(TextStyle {
          font: asset_server.load("fonts/FiraMono-Medium.ttf"),
          font_size: 60.0,
          color: Color::GOLD,
        }),
      ])
      .with_style(Style {
        align_self: AlignSelf::FlexEnd,
        ..default()
      }),
    )
    .insert(FpsText);
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
  if timer.0.tick(time.delta()).just_finished() {
    // dbg!(time);
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
      // .add_system(hello_world)
      .add_system(greet_people);
  }
}

fn gamepad_system(
  gamepads: Res<Gamepads>,
  button_inputs: Res<Input<GamepadButton>>,
  button_axes: Res<Axis<GamepadButton>>,
  axes: Res<Axis<GamepadAxis>>,
  mut query: Query<&mut Text, With<GamePadText>>,
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
    for mut text in &mut query {
      text.sections[0].value = format!(
        "{:+.4} {:+.4}, {:+.4} {:+.4}",
        left_stick_x, left_stick_y, right_stick_x, right_stick_y
      );
    }

    // println!(
    //   "{:+.4} {:+.4}, {:+.4} {:+.4}",
    //   left_stick_x, left_stick_y, right_stick_x, right_stick_y
    // );

    // if left_stick_x.abs() > 0.01 {
    //   info!("{:?} LeftStickX value is {}", gamepad, left_stick_x);
    // }
  }
}

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
  for mut text in &mut query {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
      if let Some(average) = fps.average() {
        // Update the value of the second section
        text.sections[1].value = format!("{average:.2}");
      }
    }
  }
}

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(HelloPlugin)
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_system(gamepad_system)
    .add_system(text_update_system)
    // .add_startup_system(add_people)
    // .add_system(hello_world)
    // .add_system(greet_people)
    .run();
}
