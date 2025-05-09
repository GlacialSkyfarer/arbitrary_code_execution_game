use macroquad::input::KeyCode;
use macroquad::input::is_key_down;

use crate::Vector2;

pub fn get_input_vector(left: KeyCode, right: KeyCode, up: KeyCode, down: KeyCode) -> Vector2 {
    let x: f32 = f32::from(is_key_down(right)) - f32::from(is_key_down(left));
    let y: f32 = f32::from(is_key_down(down)) - f32::from(is_key_down(up));
    Vector2::new(x, y)
}
