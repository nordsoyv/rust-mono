use crate::collision_plugin::CollisionPlugin;
use crate::components::{Enemy, EnemySpawner, Lifetime, Player, Velocity};
use crate::debug_text::DebugTextPlugin;
use crate::enemy_plugin::EnemyPlugin;
use crate::player_plugin::PlayerPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

mod collision_plugin;
mod components;
mod debug_text;
mod enemy_plugin;
mod player_plugin;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn spawn_camera(mut commands: Commands) {
  commands.spawn_bundle(Camera3dBundle {
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
    .insert(Player {
      shooting_timer: Timer::from_seconds(0.01, false),
    })
    .insert(Velocity(Vec3::ZERO))
    .insert(Name::new("Player"));
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
  commands
    .spawn()
    .insert(EnemySpawner {
      spawn_timer: Timer::from_seconds(0.5, true),
    })
    .insert(Name::new("EnemySpawner"));
}

fn update_movers(mut movers: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
  let elapsed = time.delta_seconds();
  for (vel, mut transform) in &mut movers {
    let scaled_vel = vel.0 * elapsed;
    transform.translation += scaled_vel;
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
    // .add_system(gamepad_system)
    .add_plugins(DefaultPlugins)
    .add_plugin(WorldInspectorPlugin::new())
    .add_plugin(DebugTextPlugin)
    .add_plugin(PlayerPlugin)
    .add_plugin(EnemyPlugin)
    .add_plugin(CollisionPlugin)
    .register_type::<Velocity>()
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_basic_scene)
    .add_system(update_movers)
    .add_system(bevy::window::close_on_esc)
    .run();
}
