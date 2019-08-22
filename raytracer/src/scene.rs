mod world_builder;

use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

use crate::camera::{CameraBuilder, Camera};
use crate::hitable::HitableList;
use crate::scene::world_builder::build_world;
use crate::canvas::Canvas;

#[derive(Deserialize, Serialize, Debug)]
pub struct SceneBuilder {
  pub camera: CameraBuilder,
  pub canvas: Canvas,
}

impl SceneBuilder {
  pub fn build(&self) -> Scene {
    Scene {
      canvas: self.canvas,
      camera: self.camera.build(),
      world: build_world(),
    }
  }
}

pub struct Scene {
  pub camera: Camera,
  pub canvas: Canvas,
  pub world: HitableList,
}

impl Scene {
  pub fn render(&self) -> Vec<u32> {
    return self.canvas.render(&self.camera, &self.world);
  }
}

pub fn load_scene() -> Result<Scene, Box<dyn Error>> {
  #[cfg(debug_assertions)]
    let path = std::path::Path::new("scene_debug.json");
  #[cfg(not(debug_assertions))]
    let path = std::path::Path::new("scene.json");

  let file = File::open(path)?;
  let reader = BufReader::new(file);
  let s: SceneBuilder = serde_json::from_reader(reader)?;
  Ok(s.build())
}

