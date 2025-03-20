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
  terrain_type: TerrainType,
  simple_noise_config: SimpleNoiseConfig,
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
          .subdivisions(200)
          .normal(Dir3::Z),
      ),
    ),
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
      ui.add(egui::Slider::new(&mut ui_resource.simple_noise_config.scale, 0.0..=1.0).text("Scale"));
      ui.add(egui::Slider::new(&mut ui_resource.simple_noise_config.octaves, 1..=10).text("Ocataves"));
    }

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
      let noise_1 = SuperSimplex::new(ui_resource.seed);
      let scale = ui_resource.simple_noise_config.scale;
      let octaves = ui_resource.simple_noise_config.octaves;
      for vertex in vertices.iter_mut() {
        let v1 = vertex[0] as f64;
        let v2 = vertex[1] as f64;
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
        vertex[2] = sum.powi(ui_resource.simple_noise_config.exponent) as f32;
      }
      mesh.compute_normals();
    }

    ui_resource.generate = false;
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
      terrain_type: TerrainType::SimpleNoise,
    })
    .add_systems(Startup, setup)
    .add_systems(Update, (ui_example_system, update_plane).chain())
    .run();
}
