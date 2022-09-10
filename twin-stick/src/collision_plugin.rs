use crate::components::Bullet;
use crate::{Enemy, Player};
use bevy::prelude::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(player_enemy_collision)
      .add_system(bullet_enemy_collision);
  }
}

fn player_enemy_collision(
  mut player_query: Query<&Transform, With<Player>>,
  enemy_query: Query<(&Transform), With<Enemy>>,
) {
  for &transform in &player_query {
    let player_pos = transform.translation;
    for enemy_transform in &enemy_query {
      let enemy_pos = enemy_transform.translation;
      if player_pos.distance(enemy_pos) < 1.0 {}
    }
  }
}

fn bullet_enemy_collision(
  mut commands: Commands,
  mut bullet_query: Query<(Entity, &Transform), With<Bullet>>,
  enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
  for (bullet_entity, &transform) in &bullet_query {
    let bullet_pos = transform.translation;
    for (entity, enemy_transform) in &enemy_query {
      let enemy_pos = enemy_transform.translation;
      if bullet_pos.distance(enemy_pos) < 1.0 {
        commands.entity(entity).despawn_recursive();
        commands.entity(bullet_entity).despawn_recursive();
        break;
      }
    }
  }
}
