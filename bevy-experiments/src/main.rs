use std::f32::{INFINITY, NEG_INFINITY};

use bevy::{
  color::{self, palettes::css::LIME},
  pbr::wireframe::{WireframeColor, WireframePlugin},
  prelude::*,
  render::{
    RenderPlugin,
    mesh::VertexAttributeValues,
    settings::{WgpuFeatures, WgpuSettings},
  },
  ui,
};
use bevy_egui::{EguiContexts, EguiPlugin, egui};
use noise::{NoiseFn, SuperSimplex};

#[derive(Component)]
struct MyCameraMarker;

#[derive(Component)]
struct PlaneMarker;

#[derive(Resource)]
struct UiResource {
  pub seed: u32,
  generate: bool,
  water_level: f32,
  terrain_type: TerrainType,
  simple_noise_config: SimpleNoiseConfig,
  height_map: HeightMap,
}

struct HeightMap {
  width: usize,
  height: usize,
  values: Vec<Vertex>,
}

impl HeightMap {
  fn set(&mut self, x: usize, y: usize, value: f32) {}
}

impl HeightMap {
  fn new(size: usize) -> Self {
    let side_length = size + 2;
    let values = Vec::with_capacity(side_length * side_length);

    Self {
      width: side_length,
      height: side_length,
      values,
    }
  }
}

struct Vertex {
  x: f32,
  y: f32,
  z: f32,
}

struct SimpleNoiseConfig {
  exponent: i32,
  scale: f64,
  octaves: i32,
}

#[derive(Debug, PartialEq)]
enum TerrainType {
  SimpleNoise,
  Next,
}

const SUB_DIVISIONS: usize = 200;

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let plane_mesh = Plane3d::default()
    .mesh()
    .size(10.0, 10.0)
    .subdivisions(SUB_DIVISIONS as u32)
    .normal(Dir3::Z);

  commands.spawn((
    Mesh3d(meshes.add(plane_mesh)),
    MeshMaterial3d(materials.add(Color::from(color::palettes::css::ALICE_BLUE))),
    Transform::from_xyz(0.0, 0.0, 0.0)
      .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    //Wireframe,
    WireframeColor { color: LIME.into() },
    PlaneMarker,
  ));
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

fn setup_heightmap(
  mut ui_resource: ResMut<UiResource>,
  query: Query<&Mesh3d, With<PlaneMarker>>,
  meshes: ResMut<Assets<Mesh>>,
) {
  let mesh_handle = query.get_single().unwrap();
  let mesh = meshes.get(mesh_handle).unwrap();
  let vertices = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
  let VertexAttributeValues::Float32x3(vertices) = vertices else {
    panic!("Unexpected vertex format, expected Float32x3.");
  };
  for vertex in vertices {
    let value = Vertex {
      x: vertex[0],
      y: vertex[1],
      z: vertex[2],
    };
    ui_resource.height_map.values.push(value);
  }
}

fn ui_example_system(mut contexts: EguiContexts, mut ui_resource: ResMut<UiResource>) {
  egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
    egui::ComboBox::from_label("Terrain type")
      .selected_text(format!("{:?}", ui_resource.terrain_type))
      .show_ui(ui, |ui| {
        ui.selectable_value(
          &mut ui_resource.terrain_type,
          TerrainType::SimpleNoise,
          "Simple",
        );
        ui.selectable_value(&mut ui_resource.terrain_type, TerrainType::Next, "Next");
      });
    ui.add(egui::Slider::new(&mut ui_resource.seed, 0..=100).text("Seed"));
    if ui_resource.terrain_type == TerrainType::SimpleNoise {
      ui.add(egui::Slider::new(&mut ui_resource.simple_noise_config.exponent, 1..=5).text("Exp"));
      ui.add(
        egui::Slider::new(&mut ui_resource.simple_noise_config.scale, 0.0..=1.0).text("Scale"),
      );
      ui.add(
        egui::Slider::new(&mut ui_resource.simple_noise_config.octaves, 1..=10).text("Ocataves"),
      );
    }
    ui.add(egui::Slider::new(&mut ui_resource.water_level, 0.0..=2.0).text("Water level"));
    if ui.button("Generate").clicked() {
      ui_resource.generate = true;
    }
  });
}

fn update_plane(
  mut ui_resource: ResMut<UiResource>,
  query: Query<&Mesh3d, With<PlaneMarker>>,
  mut meshes: ResMut<Assets<Mesh>>,
) {
  if ui_resource.generate {
    let mesh_handle = query.get_single().unwrap();
    let mesh = meshes.get_mut(mesh_handle).unwrap();
    let vertices = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION).unwrap();
    let VertexAttributeValues::Float32x3(vertices) = vertices else {
      panic!("Unexpected vertex format, expected Float32x3.");
    };
    if ui_resource.terrain_type == TerrainType::SimpleNoise {
      generate_simplex_terrain( &mut ui_resource);
    }
    update_mesh(vertices, &ui_resource);
    mesh.compute_normals();
    ui_resource.generate = false;
  }
}

fn generate_simplex_terrain(config: &mut UiResource) {
  let mut largest = NEG_INFINITY;
  let mut smallest = INFINITY;
  let noise_1 = SuperSimplex::new(config.seed);
  let scale = config.simple_noise_config.scale;
  let octaves = config.simple_noise_config.octaves;

  for vertex in &mut config.height_map.values {
    let v1 = vertex.x as f64;
    let v2 = vertex.y as f64;
    let mut sum = 0.0;
    let mut divisor = 0.0;
    for octave in 0..=octaves {
      let o = 2.0_f64.powi(octave);
      let mut n = noise_1.get([v1 * o * scale, v2 * o * scale]);
      n = n / o;
      sum = sum + n;
      divisor = divisor + 1.0 / o;
    }
    sum = sum / divisor;
    let mut vertex_z_value = sum.powi(config.simple_noise_config.exponent) as f32;

    vertex_z_value += 1.0;
    if vertex_z_value > largest {
      largest = vertex_z_value;
    }
    if vertex_z_value < smallest {
      smallest = vertex_z_value;
    }

    if vertex_z_value < config.water_level {
      vertex_z_value = config.water_level;
    }
    vertex.z = vertex_z_value;
  }
  dbg!(largest);
  dbg!(smallest);
}

fn update_mesh(vertices: &mut [[f32; 3]], config: &UiResource){
  for (vertex, value) in vertices.iter_mut().zip(&config.height_map.values) {
    vertex[0]= value.x;
    vertex[1]= value.y;
    vertex[2]= value.z;
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
    .insert_resource(UiResource {
      seed: 10,
      simple_noise_config: SimpleNoiseConfig {
        exponent: 1,
        scale: 0.25,
        octaves: 4,
      },
      generate: false,
      water_level: 0.0,
      terrain_type: TerrainType::SimpleNoise,
      height_map: HeightMap::new(SUB_DIVISIONS),
    })
    .add_systems(Startup, (setup, setup_heightmap).chain())
    .add_systems(Update, (ui_example_system, update_plane).chain())
    .run();
}
