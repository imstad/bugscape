use raylib::color::Color;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init() //
        .title("Bugscape")
        .size(800, 800)
        .resizable()
        .build();
    rl.set_target_fps(60);
    let bg = rl
        .load_texture(&thread, "harmonycobelfather.png")
        .expect("Failed to load title sprite");
    let mut y = rl.get_screen_height() / 2;
    let text_w = rl.measure_text("use your keybinds.", 20);
    let cx = rl.measure_text("Successfully copied to clipboard!", 20);
    let mut x = (rl.get_screen_width() - text_w) / 2;
    let mut sdt = false;
    let mut copied = false;
    let mut seconds = 0.0;
    while !rl.window_should_close() {
        let max_x = rl.get_screen_width() - text_w;
        let max_y = rl.get_screen_height() - 20;
        let mid_x = rl.get_screen_width() / 2;
        let mid_y = rl.get_screen_height() / 2;
        let cxm = rl.get_screen_width() - cx;
        let should_draw_text =
            rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) && rl.is_key_pressed(KeyboardKey::KEY_C);
        if should_draw_text {
            rl.set_clipboard_text("what");
            copied = true
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) && rl.is_key_pressed(KeyboardKey::KEY_V) {
            sdt = true;
        }
        if copied {
            seconds = seconds + rl.get_frame_time();
        }
        if seconds > 3.0 {
            copied = false;
            seconds = 0.0;
        }
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
        if sdt {
            d.draw_text("what", mid_x, mid_y, 100, Color::BLANCHEDALMOND);
        }
        if copied {
            d.draw_text(
                "Successfully copied to clipboard!",
                cxm - 20,
                20,
                20,
                Color::DARKORCHID,
            )
        }
        d.draw_text("use your keybinds.", x, y, 20, Color::SALMON);
        d.clear_background(Color::BLACK);
    }
}
