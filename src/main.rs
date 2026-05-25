use raylib::color::Color;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init() //
        .title("Bugscape")
        .size(800, 800)
        .resizable()
        .build();
    rl.set_target_fps(30);
    let bg = rl
        .load_texture(&thread, "harmonycobelfather.png")
        .expect("Failed to load title sprite");
    let mut y = rl.get_screen_height() / 2;
    let text_w = rl.measure_text("this is epic", 20);
    let mut x = (rl.get_screen_width() - text_w) / 2;
    while !rl.window_should_close() {
        let max_x = rl.get_screen_width() - text_w;
        let max_y = rl.get_screen_height() - 20;
        if rl.is_key_down(KeyboardKey::KEY_S) {
            y = y + 2;
        }
        if rl.is_key_down(KeyboardKey::KEY_W) {
            y = y - 2;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            x = x - 2;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            x = x + 2;
        }
        if x < 0 {
            x = 0
        }
        if x > max_x {
            x = max_x
        }
        if y > max_y {
            y = max_y
        }
        if y < 0 {
            y = 0
        }
        let mut d = rl.begin_drawing(&thread);
        // d.draw_texture_ex(&bg, Vector2::new(0.0, 0.0), 0.0, 1.6, Color::WHITE);
        d.draw_texture_pro(
            &bg,
            Rectangle::new(0.0, 0.0, 500.0, 500.0),
            Rectangle::new(
                0.0,
                0.0,
                d.get_screen_width() as f32,
                d.get_screen_height() as f32,
            ),
            Vector2::new(0.0, 0.0),
            0.0,
            Color::POWDERBLUE,
        );
        d.draw_text("this is epic", x, y, 20, Color::SALMON);
        d.clear_background(Color::BLACK);
    }
}
