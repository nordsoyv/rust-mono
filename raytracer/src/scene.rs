mod world_builder;

use serde_derive::{Deserialize, Serialize};
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
  let data = r#"
  {
    "canvas": {
      "width": 400,
      "height": 200,
      "samples" : 200
    },
    "camera": {
      "lookfrom": {
        "x":-2.0,
        "y":2.0,
        "z":1.0
      },
      "lookat": {
        "x":0.0,
        "y":0.0,
        "z":-1.0
      },
      "vup": {
        "x":0.0,
        "y":1.0,
        "z":0.0
      },
      "vfov": 90.0,
      "aspect":2.0
    }
  }"#;
  let s: SceneBuilder = serde_json::from_str(data)?;

  Ok(s.build())
}

