use macroquad::prelude::*;

struct Player {
    rect: Rect
}

impl Player {
    pub fn new() -> Self {
        Self { rect: Rect::new(
            screen_width() * 0.5f32 - 150f32*0.5f32,
            screen_height() - 100f32,
            150f32,
            40f32,
        ) }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            DARKBLUE,
        )
    }
}

#[macroquad::main("breakout")]
async fn main() {
    let player_rect = Player::new();

    loop {
        clear_background(WHITE);
        player_rect.draw();
        next_frame().await
    }
}
