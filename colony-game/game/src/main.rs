use macroquad::prelude::*;

struct TileDrawer {
  tiles: Texture2D,
}

impl TileDrawer {
  pub async fn new(path: &str) -> TileDrawer {
    let tiles = load_texture(path).await.unwrap();
    TileDrawer { tiles }
  }

  pub fn draw_tile(&self, pos_x: f32, pos_y: f32, x: f32, y: f32) {
    draw_texture_ex(
      &self.tiles,
      pos_x,
      pos_y,
      WHITE,
      DrawTextureParams {
        dest_size: Some(Vec2 { x: 16.0, y: 16.0 }),
        source: Some(Rect {
          x: x * 16.0,
          y: y * 16.0,
          w: 16.0,
          h: 16.0,
        }),
        ..Default::default()
      },
    );
  }
}

#[macroquad::main("Colony")]
async fn main() {
  let tile_drawer = TileDrawer::new("game/resources/16x16_terrain.png").await;
  loop {
    clear_background(BLACK);

    // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
    // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
    // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
    // draw_text("Hello", 20.0, 20.0, 20.0, DARKGRAY);
    for pos_x in 0..30 {
      tile_drawer.draw_tile(100.0 + (pos_x as f32 *16.0), 100.0, pos_x as f32, 10.0);        
    }

    next_frame().await
  }
}