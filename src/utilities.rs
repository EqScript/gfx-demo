use macroquad::prelude::*;

// Pixel drawing wrapper
pub fn draw_pixel(x: f32, y: f32, color: Color) {
    draw_rectangle(x, y, 1.0, 1.0, color);
}
