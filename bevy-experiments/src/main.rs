use bevy::{
  color::{self, palettes::css::LIME}, pbr::wireframe::{NoWireframe, Wireframe, WireframeColor, WireframeConfig, WireframePlugin}, prelude::*, render::{
    settings::{WgpuFeatures, WgpuSettings}, RenderPlugin
  }
};
#[derive(Component)]
struct MyCameraMarker;

fn setup_camera(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands.spawn((
    Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0).subdivisions(20).normal(Dir3::Z))),
    MeshMaterial3d(materials.add(Color::from(color::palettes::css::ALICE_BLUE))),
    Transform::from_xyz(0.0, 0.0, 0.0)
      .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    //Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    Wireframe,
    WireframeColor{ color: LIME.into()}
  ));
  // cube
  commands.spawn((
    Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
    MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
    Transform::from_xyz(0.0, 0.5, 0.0),
  ));
  // light
  commands.spawn((
    PointLight {
      shadows_enabled: true,
      ..default()
    },
    Transform::from_xyz(4.0, 8.0, 4.0),
  ));
  commands.spawn((
    Camera3d {
      ..Default::default()
    },
    Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    MyCameraMarker,
  ));
}


fn main() {
  App::new()
    .add_plugins((
      DefaultPlugins.set(RenderPlugin {
        render_creation: bevy::render::settings::RenderCreation::Automatic(WgpuSettings {
          features: WgpuFeatures::POLYGON_MODE_LINE,
          ..default()
        }),
        ..default()
      }),
      WireframePlugin,
    ))
    .add_systems(Startup, setup_camera)
    .run();
}
