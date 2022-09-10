use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Lifetime {
  pub timer: Timer,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Velocity(pub Vec3);

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Acceleration(pub Vec3);

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Bullet;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct EnemySpawner {
  pub spawn_timer: Timer,
}

#[derive(Reflect, Component, Copy, Clone, Default)]
#[reflect(Component)]
pub enum Enemy {
  #[default]
  Seeker,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Player {
  pub shooting_timer: Timer,
}
