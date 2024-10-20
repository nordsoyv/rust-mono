use cgmath::*;
use std::f32::consts::FRAC_PI_2;
use winit::dpi::PhysicalPosition;
use winit::event::*;
use winit::keyboard::KeyCode;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;

#[derive(Debug)]
pub struct Camera {
  pub position: Point3<f32>,
  yaw: Rad<f32>,
  pitch: Rad<f32>,
  projection: Projection,
  controller: CameraController,
  pub camera_uniform: CameraUniform,
}

impl Camera {
  pub fn new<V: Into<Point3<f32>>, Y: Into<Rad<f32>>, P: Into<Rad<f32>>>(
    position: V,
    yaw: Y,
    pitch: P,
    width: u32,
    height: u32,
  ) -> Self {
    let position = position.into();
    let yaw = yaw.into();
    let pitch = pitch.into();
    let projection = Projection::new(width, height, cgmath::Deg(45.0), 0.1, 100.0);
    let camera_controller = CameraController::new(4.0, 2.0);
    let mut camera_uniform = CameraUniform::new();
    camera_uniform.update_view_proj(position, &projection, calc_matrix(position, yaw, pitch));
    Self {
      position,
      yaw,
      pitch,
      projection,
      controller: camera_controller,
      camera_uniform,
    }
  }

  pub fn calc_matrix(&self) -> Matrix4<f32> {
    calc_matrix(self.position, self.yaw, self.pitch)
  }

  fn update_view_proj(&mut self) {
    let cam_matrix = self.calc_matrix();
    let uniform = &mut self.camera_uniform;

    uniform.view_position = self.position.to_homogeneous().into();
    uniform.view_proj = (self.projection.calc_matrix() * cam_matrix).into();
  }

  fn update_controller(&mut self, delta_time: f32) {
    let ctrl = &mut self.controller;

    // Move forward/backward and left/right
    let (yaw_sin, yaw_cos) = self.yaw.0.sin_cos();
    let forward = Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
    let right = Vector3::new(-yaw_sin, 0.0, yaw_cos).normalize();
    self.position +=
      forward * (ctrl.amount_forward - ctrl.amount_backward) * ctrl.speed * delta_time;
    self.position += right * (ctrl.amount_right - ctrl.amount_left) * ctrl.speed * delta_time;

    // Move in/out (aka. "zoom")
    // Note: this isn't an actual zoom. The camera's position
    // changes when zooming. I've added this to make it easier
    // to get closer to an object you want to focus on.
    let (pitch_sin, pitch_cos) = self.pitch.0.sin_cos();
    let scrollward = Vector3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize();
    self.position += scrollward * ctrl.scroll * ctrl.speed * ctrl.sensitivity * delta_time;
    ctrl.scroll = 0.0;

    //   // Move up/down. Since we don't use roll, we can just
    //   // modify the y coordinate directly.
    self.position.y += (ctrl.amount_up - ctrl.amount_down) * ctrl.speed * delta_time;

    // Rotate
    self.yaw += Rad(ctrl.rotate_horizontal) * ctrl.sensitivity * delta_time;
    self.pitch += Rad(-ctrl.rotate_vertical) * ctrl.sensitivity * delta_time;

    // If process_mouse isn't called every frame, these values
    // will not get set to zero, and the camera will rotate
    // when moving in a non-cardinal direction.
    ctrl.rotate_horizontal = 0.0;
    ctrl.rotate_vertical = 0.0;

    // Keep the camera's angle from going too high/low.
    if self.pitch < -Rad(SAFE_FRAC_PI_2) {
      self.pitch = -Rad(SAFE_FRAC_PI_2);
    } else if self.pitch > Rad(SAFE_FRAC_PI_2) {
      self.pitch = Rad(SAFE_FRAC_PI_2);
    }
  }

  pub fn update_camera(&mut self, delta_time: f32) {
    self.update_controller(delta_time);
    self.update_view_proj();
  }
}

fn calc_matrix(position: Point3<f32>, yaw: Rad<f32>, pitch: Rad<f32>) -> Matrix4<f32> {
  let (sin_pitch, cos_pitch) = pitch.0.sin_cos();
  let (sin_yaw, cos_yaw) = yaw.0.sin_cos();

  Matrix4::look_to_rh(
    position,
    Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
    Vector3::unit_y(),
  )
}

#[derive(Debug)]
struct Projection {
  aspect: f32,
  fovy: Rad<f32>,
  znear: f32,
  zfar: f32,
}

impl Projection {
  pub fn new<F: Into<Rad<f32>>>(width: u32, height: u32, fovy: F, znear: f32, zfar: f32) -> Self {
    Self {
      aspect: width as f32 / height as f32,
      fovy: fovy.into(),
      znear,
      zfar,
    }
  }

  pub fn resize(&mut self, width: u32, height: u32) {
    self.aspect = width as f32 / height as f32;
  }

  pub fn calc_matrix(&self) -> Matrix4<f32> {
    OPENGL_TO_WGPU_MATRIX * perspective(self.fovy, self.aspect, self.znear, self.zfar)
  }
}

#[derive(Debug)]
struct CameraController {
  amount_left: f32,
  amount_right: f32,
  amount_forward: f32,
  amount_backward: f32,
  amount_up: f32,
  amount_down: f32,
  rotate_horizontal: f32,
  rotate_vertical: f32,
  scroll: f32,
  speed: f32,
  sensitivity: f32,
}

impl CameraController {
  pub fn new(speed: f32, sensitivity: f32) -> Self {
    Self {
      amount_left: 0.0,
      amount_right: 0.0,
      amount_forward: 0.0,
      amount_backward: 0.0,
      amount_up: 0.0,
      amount_down: 0.0,
      rotate_horizontal: 0.0,
      rotate_vertical: 0.0,
      scroll: 0.0,
      speed,
      sensitivity,
    }
  }

  pub fn process_keyboard(&mut self, key: KeyCode, state: ElementState) -> bool {
    let amount = if state == ElementState::Pressed {
      1.0
    } else {
      0.0
    };
    match key {
      KeyCode::KeyW | KeyCode::ArrowUp => {
        self.amount_forward = amount;
        true
      }
      KeyCode::KeyS | KeyCode::ArrowDown => {
        self.amount_backward = amount;
        true
      }
      KeyCode::KeyA | KeyCode::ArrowLeft => {
        self.amount_left = amount;
        true
      }
      KeyCode::KeyD | KeyCode::ArrowRight => {
        self.amount_right = amount;
        true
      }
      KeyCode::Space => {
        self.amount_up = amount;
        true
      }
      KeyCode::ShiftLeft => {
        self.amount_down = amount;
        true
      }
      _ => false,
    }
  }

  pub fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
    self.rotate_horizontal = mouse_dx as f32;
    self.rotate_vertical = mouse_dy as f32;
  }

  pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
    self.scroll = -match delta {
      // I'm assuming a line is about 100 pixels
      MouseScrollDelta::LineDelta(_, scroll) => scroll * 100.0,
      MouseScrollDelta::PixelDelta(PhysicalPosition { y: scroll, .. }) => *scroll as f32,
    };
  }
}

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
  pub view_position: [f32; 4],
  // We can't use cgmath with bytemuck directly, so we'll have
  // to convert the Matrix4 into a 4x4 f32 array
  pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
  pub fn new() -> Self {
    use cgmath::SquareMatrix;
    Self {
      view_position: [0.0; 4],
      view_proj: cgmath::Matrix4::identity().into(),
    }
  }

  fn update_view_proj(
    &mut self,
    position: Point3<f32>,
    projection: &Projection,
    camera_matrix: Matrix4<f32>,
  ) {
    self.view_position = position.to_homogeneous().into();
    self.view_proj = (projection.calc_matrix() * camera_matrix).into();
  }
}
