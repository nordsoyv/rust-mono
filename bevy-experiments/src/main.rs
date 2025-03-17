use bevy::{
  color::{self, palettes::css::LIME},
  pbr::wireframe::{NoWireframe, Wireframe, WireframeColor, WireframeConfig, WireframePlugin},
  prelude::*,
  render::{
    RenderPlugin,
    mesh::VertexAttributeValues,
    settings::{WgpuFeatures, WgpuSettings},
  },
};
use bevy_egui::{EguiContexts, EguiPlugin, egui};

#[derive(Component)]
struct MyCameraMarker;

#[derive(Component)]
struct PlaneMarker;

#[derive(Resource)]
struct UiResource {
  pub value: f32,
}

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands.spawn((
    Mesh3d(
      meshes.add(
        Plane3d::default()
          .mesh()
          .size(10.0, 10.0)
          .subdivisions(20)
          .normal(Dir3::Z),
      ),
    ),
    MeshMaterial3d(materials.add(Color::from(color::palettes::css::ALICE_BLUE))),
    Transform::from_xyz(0.0, 0.0, 0.0)
      .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    Wireframe,
    WireframeColor { color: LIME.into() },
    PlaneMarker,
  ));
  // cube
  // commands.spawn((
  //   Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
  //   MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
  //   Transform::from_xyz(0.0, 0.5, 0.0),
  // ));
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

fn ui_example_system(mut contexts: EguiContexts, mut ui_resource: ResMut<UiResource>) {
  egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
    ui.add(egui::Slider::new(&mut ui_resource.value, 0.0..=100.0).text("My value"))
  });
}

fn update_plane(
  ui_resource: Res<UiResource>,
  query: Query<&Mesh3d, With<PlaneMarker>>,
  mut meshes: ResMut<Assets<Mesh>>,
) {
  let mesh_handle = query.get_single().unwrap();
  let mesh = meshes.get_mut(mesh_handle).unwrap();
  let vertices = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION).unwrap();
  let VertexAttributeValues::Float32x3(vertices) = vertices else {
    panic!("Unexpected vertex format, expected Float32x3.");
  };
  let mut index = 0;
  for vertex in vertices.iter_mut() {
    if index == 50 {
      vertex[2] = ui_resource.value / 10.0;
    }
    index += 1;
  }
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
      EguiPlugin,
    ))
    .insert_resource(UiResource { value: 10.0 })
    .add_systems(Startup, setup)
    .add_systems(Update, (ui_example_system, update_plane).chain())
    .run();
}
