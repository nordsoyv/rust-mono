use crate::components::Bullet;
use crate::{Lifetime, Player, Velocity};
use bevy::prelude::*;

const MAX_PLAYER_VELOCITY: f32 = 30.0;
const MAX_BULLET_VELOCITY: f32 = 50.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      // .add_system(gamepad_system)
      .add_system(player_shooting)
      .add_system(move_player)
      .add_system(bullet_despawn)
      .register_type::<Player>();
  }
}

// fn gamepad_system(
//   gamepads: Res<Gamepads>,
//   button_inputs: Res<Input<GamepadButton>>,
//   button_axes: Res<Axis<GamepadButton>>,
//   axes: Res<Axis<GamepadAxis>>,
// ) {
//   for gamepad in gamepads.iter().cloned() {
//     if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South)) {
//       info!("{:?} just pressed South", gamepad);
//     } else if button_inputs.just_released(GamepadButton::new(gamepad, GamepadButtonType::South)) {
//       info!("{:?} just released South", gamepad);
//     }
//
//     let right_trigger = button_axes
//       .get(GamepadButton::new(
//         gamepad,
//         GamepadButtonType::RightTrigger2,
//       ))
//       .unwrap();
//     if right_trigger.abs() > 0.01 {
//       info!("{:?} RightTrigger2 value is {}", gamepad, right_trigger);
//     }
//
//     let left_stick_x = axes
//       .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
//       .unwrap();
//     let left_stick_y = axes
//       .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
//       .unwrap();
//     let right_stick_x = axes
//       .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickX))
//       .unwrap();
//     let right_stick_y = axes
//       .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickY))
//       .unwrap();
//     let gamepad_info = format!(
//       "{:+.4} {:+.4}, {:+.4} {:+.4}",
//       left_stick_x, left_stick_y, right_stick_x, right_stick_y
//     );
//     debug(DebugTextType::GamePad, gamepad_info);
//   }
// }

fn player_shooting(
  mut commands: Commands,
  mut players: Query<(&mut Player, &Transform)>,
  time: Res<Time>,
  gamepads: Res<Gamepads>,
  axes: Res<Axis<GamepadAxis>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let mut right_stick_x = 0.0;
  let mut right_stick_y = 0.0;
  for gamepad in gamepads.iter().cloned() {
    right_stick_x = axes
      .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickX))
      .unwrap();
    right_stick_y = axes
      .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickY))
      .unwrap();
  }
  let shooting_dir = Vec3::new(right_stick_x, right_stick_y, 0.0);

  for (mut p, transform) in &mut players {
    p.shooting_timer.tick(time.delta());
    if p.shooting_timer.finished() && shooting_dir.length() > 0.3 {
      // dbg!("Shooting");
      p.shooting_timer = Timer::from_seconds(0.1, false);
      let shooting_dir_angle = shooting_dir.angle_between(Vec3::new(10.0, 0.0, 0.0));
      let spawn_transform = Transform::from_translation(transform.translation)
        .with_rotation(Quat::from_rotation_z(shooting_dir_angle));

      commands
        .spawn_bundle(PbrBundle {
          mesh: meshes.add(Mesh::from(shape::UVSphere {
            radius: 0.5,
            ..default()
          })),
          material: materials.add(StandardMaterial {
            emissive: Color::rgb(47.0 / 255.0, 255.0 / 255.0, 0.0 / 255.0),
            perceptual_roughness: 0.7,
            reflectance: 9.5,
            ..default()
          }),
          transform: spawn_transform,
          ..default()
        })
        .insert(Lifetime {
          timer: Timer::from_seconds(2.5, false),
        })
        .insert(Velocity(shooting_dir.normalize() * MAX_BULLET_VELOCITY))
        .insert(Bullet)
        .insert(Name::new("Bullet"));
    }
  }
}

fn move_player(
  mut players: Query<&mut Velocity, With<Player>>,
  gamepads: Res<Gamepads>,
  axes: Res<Axis<GamepadAxis>>,
) {
  let mut left_stick_x = 0.0;
  let mut left_stick_y = 0.0;
  for gamepad in gamepads.iter().cloned() {
    left_stick_x = axes
      .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
      .unwrap();
    left_stick_y = axes
      .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
      .unwrap();
  }
  let vec_a = Vec3::new(left_stick_x, left_stick_y, 0.0);

  for mut vel in &mut players {
    if vec_a.length() > 0.2 {
      vel.0 = vec_a * MAX_PLAYER_VELOCITY;
    } else {
      vel.0 = Vec3::ZERO;
    }
  }
}

fn bullet_despawn(
  mut commands: Commands,
  mut bullets: Query<(Entity, &mut Lifetime)>,
  time: Res<Time>,
) {
  for (entity, mut lifetime) in &mut bullets {
    lifetime.timer.tick(time.delta());
    if lifetime.timer.just_finished() {
      commands.entity(entity).despawn_recursive();
    }
  }
}
