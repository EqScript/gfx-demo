use macroquad::prelude::*;
use std::f32;
use std::f32::consts::PI;
mod utilities;
use utilities::draw_pixel;


fn window_conf() -> Conf {
    Conf {
        window_title: "Macroquad Demo".to_owned(),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    let bgr_color = Color {r: 0.1, g: 0.1, b: 0.1, a: 1.0};
    let cx: f32         = 200.0;
    let cy: f32         = 300.0;
    let r:  f32         = 120.0;

    loop {
        clear_background(bgr_color);

        let mut x: f32;
        let mut y: f32;
        let color: Color = WHITE;

        draw_circle(cx + 300.0, cy, r, color);

        for deg  in 0..90 {
            let rad: f32 = (deg as f32)/180.0 * PI;
            x = rad.cos() * r;
            y = rad.sin() * r;

            draw_pixel(cx + x, cy + y, color);
            draw_pixel(cx + x, cy - y, color);
            draw_pixel(cx - x, cy + y, color);
            draw_pixel(cx - x, cy - y, color);
        }
        next_frame().await;
    }
}
