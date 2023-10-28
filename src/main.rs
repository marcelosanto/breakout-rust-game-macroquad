use macroquad::prelude::*;

#[macroquad::main("breakout")]
async fn main() {
    let player_rect = Rect::new(
        screen_width() * 0.5f32,
        screen_height() - 100f32,
        150f32,
        40f32,
    );

    loop {
        clear_background(WHITE);
        draw_rectangle(
            player_rect.x,
            player_rect.y,
            player_rect.w,
            player_rect.h,
            DARKBLUE,
        );
        next_frame().await
    }
}
