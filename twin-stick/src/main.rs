use crate::debug_text::{debug, DebugTextPlugin, DebugTextType};
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use std::f32::consts::PI;

mod debug_text;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

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

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
  pub shooting_timer: Timer,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Lifetime {
  pub timer: Timer,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Velocity(Vec3);

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Acceleration(Vec3);

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Player;

fn spawn_camera(mut commands: Commands) {
  commands.spawn_bundle(Camera3dBundle {
    // transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    transform: Transform::from_xyz(0.0, -20.0, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
    ..default()
  });
}

fn spawn_basic_scene(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands
    .spawn_bundle(PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Capsule {
        radius: 0.5,
        ..default()
      })),
      material: materials.add(StandardMaterial {
        emissive: Color::rgb(47.0 / 255.0, 0.0 / 255.0, 255.0 / 255.0),
        perceptual_roughness: 0.7,
        reflectance: 7.5,
        ..default()
      }),
      transform: Transform::from_xyz(10.0, 0.0, 0.0),
      ..default()
    })
    .insert(Player)
    .insert(Acceleration(Vec3::ZERO))
    .insert(Velocity(Vec3::ZERO))
    .insert(Name::new("Player"));
  commands
    .spawn_bundle(PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
      material: materials.add(StandardMaterial {
        emissive: Color::rgb(47.0 / 255.0, 0.0 / 255.0, 255.0 / 255.0),
        perceptual_roughness: 0.7,
        reflectance: 7.5,
        ..default()
      }),
      transform: Transform::from_xyz(0.0, 0.5, 0.0),
      ..default()
    })
    .insert(Tower {
      shooting_timer: Timer::from_seconds(1.0, true),
    })
    .insert(Name::new("Tower"));
  commands
    .spawn_bundle(PointLightBundle {
      point_light: PointLight {
        intensity: 1500.0,
        shadows_enabled: true,
        ..default()
      },
      transform: Transform::from_xyz(4.0, 8.0, 4.0),
      ..default()
    })
    .insert(Name::new("Light"));
}

fn tower_shooting(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut towers: Query<&mut Tower>,
  time: Res<Time>,
) {
  for mut tower in &mut towers {
    tower.shooting_timer.tick(time.delta());
    if tower.shooting_timer.just_finished() {
      let spawn_transform =
        Transform::from_xyz(0.0, 0.7, 0.6).with_rotation(Quat::from_rotation_y(-PI / 2.0));
      commands
        .spawn_bundle(PbrBundle {
          mesh: meshes.add(Mesh::from(shape::UVSphere {
            radius: 0.1,
            ..default()
          })),
          // material: materials.add(Color::rgb(0.87, 0.44, 0.42).into()),
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
        .insert(Velocity(Vec3::new(1.0, 0.0, 0.0)))
        .insert(Name::new("Bullet"));
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

fn update_movers(mut movers: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
  let elapsed = time.delta_seconds();
  for (vel, mut transform) in &mut movers {
    let scaled_vel = vel.0 * elapsed;
    transform.translation += scaled_vel;
  }
}

const MAX_PLAYER_VELOCITY: f32 = 30.0;

fn move_player(
  mut players: Query<(&mut Velocity, &Player)>,
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
  for (mut vel, _player) in &mut players {
    let vec_a = Vec3::new(left_stick_x, left_stick_y, 0.0);
    vel.0 = vec_a * MAX_PLAYER_VELOCITY;
  }
}

fn main() {
  App::new()
    // .insert_resource(WgpuSettings {
    //   features: WgpuFeatures::POLYGON_MODE_LINE,
    //   ..default()
    // })
    .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
    .insert_resource(WindowDescriptor {
      width: WIDTH,
      height: HEIGHT,
      title: "Twin stick shooter".to_string(),
      resizable: false,
      ..Default::default()
    })
    // .add_plugin(WireframePlugin)
    .add_system(gamepad_system)
    .add_plugins(DefaultPlugins)
    .add_plugin(WorldInspectorPlugin::new())
    .add_plugin(DebugTextPlugin)
    .register_type::<Tower>()
    .register_type::<Velocity>()
    .register_type::<Acceleration>()
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_basic_scene)
    .add_system(tower_shooting)
    .add_system(bullet_despawn)
    .add_system(update_movers)
    .add_system(move_player)
    .add_system(bevy::window::close_on_esc)
    .run();
}
