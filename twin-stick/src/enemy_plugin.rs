use crate::{Enemy, EnemySpawner, Player, Velocity};
use bevy::prelude::*;

const MAX_ENEMY_VELOCITY: f32 = 15.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(spawn_enemies)
      .add_system(move_enemies)
      .register_type::<Enemy>()
      .register_type::<EnemySpawner>();
  }
}

fn spawn_enemies(
  mut commands: Commands,
  mut spawners: Query<&mut EnemySpawner>,
  time: Res<Time>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  for mut spawner in &mut spawners {
    spawner.spawn_timer.tick(time.delta());
    if spawner.spawn_timer.just_finished() {
      commands
        .spawn_bundle(PbrBundle {
          mesh: meshes.add(Mesh::from(shape::Cube {
            size: 0.2,
            ..default()
          })),
          material: materials.add(StandardMaterial {
            emissive: Color::rgb(255.0 / 255.0, 0.0 / 255.0, 255.0 / 255.0),
            perceptual_roughness: 0.7,
            reflectance: 7.5,
            ..default()
          }),
          transform: Transform::from_xyz(40.0, 0.0, 0.0),
          ..default()
        })
        .insert(Enemy::Seeker)
        .insert(Velocity(Vec3::ZERO))
        .insert(Name::new("Enemy"));
    }
  }
}

fn move_enemies(
  mut enemies: Query<(&Enemy, &mut Velocity, &Transform)>,
  players: Query<&Transform, With<Player>>,
) {
  let mut player_location = Vec3::ZERO;
  for location in &players {
    player_location = location.translation;
  }
  for (enemy, mut vel, transform) in &mut enemies {
    match enemy {
      Enemy::Seeker => {
        let dir_to_player = (player_location - transform.translation).normalize();
        vel.0 = dir_to_player * MAX_ENEMY_VELOCITY;
      }
      _ => todo!(),
    }
  }
}
