use bevy::{
  color::{self, palettes::css::LIME},
  pbr::wireframe::{NoWireframe, Wireframe, WireframeColor, WireframeConfig, WireframePlugin},
  prelude::*,
  render::{
    mesh::VertexAttributeValues, settings::{WgpuFeatures, WgpuSettings}, RenderPlugin
  }, ui,
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
  exponent : i32,
  generate: bool,
  scale : f64,
  octaves : i32,
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
    ui.add(egui::Slider::new(&mut ui_resource.seed, 0..=100).text("Seed"));
    ui.add(egui::Slider::new(&mut ui_resource.exponent, 1..=5).text("Exp"));
    ui.add(egui::Slider::new(&mut ui_resource.scale, 0.0..=1.0).text("Scale"));
    ui.add(egui::Slider::new(&mut ui_resource.octaves, 1..=10).text("Ocataves"));
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
    let noise_1 = SuperSimplex::new(ui_resource.seed);
    // let noise_2 = SuperSimplex::new(ui_resource.seed + 1);
    // let noise_3 = SuperSimplex::new(ui_resource.seed + 2);
    // let noise_4 = SuperSimplex::new(ui_resource.seed + 3);
    // let noise_5 = SuperSimplex::new(ui_resource.seed + 4);
    let scale = ui_resource.scale;
    let octaves = ui_resource.octaves;
    for vertex in vertices.iter_mut() {
      let v1 = vertex[0] as f64;
      let v2 = vertex[1] as f64;
      let mut sum = 0.0;
      let mut divisor= 0.0;
      for octave  in 0..=octaves {
        let o = 2.0_f64.powi(octave);
        //dbg!(o);
        let mut n =noise_1.get([v1*o*scale, v2*o*scale]);
        n = n / o ;
        sum = sum + n;
        divisor = divisor + 1.0/o;
      }
      //dbg!(sum);
      //dbg!(divisor);
      sum = sum/divisor;


      // let n1 = noise_1.get([v1, v2]) as f32;
      // let n2 = noise_2.get([v1 * 2.0, v2 * 2.0]) as f32;
      // let n3 = noise_3.get([v1 * 4.0, v2 * 4.0]) as f32;
      // let n4 = noise_4.get([v1 * 8.0, v2 * 8.0]) as f32;
      // let n5 = noise_5.get([v1 * 16.0, v2 * 16.0]) as f32;
      // let mut v = n1 + (n2 * 0.5) + (n3 * 0.25) + (n4 * 0.125) + (n5* 0.0625);
      // v = v / (1.0 + 0.5 + 0.25 + 0.125 + 0.0625);
      vertex[2] = sum.powi(ui_resource.exponent) as f32;
    }
    mesh.compute_normals();
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
      exponent:1,
      generate: false,
      octaves:4,
      scale: 0.25
    })
    .add_systems(Startup, setup)
    .add_systems(Update, (ui_example_system, update_plane).chain())
    .run();
}
