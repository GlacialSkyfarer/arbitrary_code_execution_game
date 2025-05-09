use std::collections::HashSet;

use macroquad::input::KeyCode;
use macroquad::input::is_key_down;
use macroquad::math::Vec2;

pub struct InputMap {
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub wait: KeyCode,
    pub undo: KeyCode,
    pub pause: KeyCode,
}

#[derive(Debug)]
pub enum InputAction {
    Wait,
    Move(usize),
    None,
}

pub fn get_input_vector(left: KeyCode, right: KeyCode, up: KeyCode, down: KeyCode) -> Vec2 {
    let x: f32 = f32::from(is_key_down(right)) - f32::from(is_key_down(left));
    let y: f32 = f32::from(is_key_down(down)) - f32::from(is_key_down(up));
    Vec2::new(x, y)
}

pub fn get_turn_input(input_map: &InputMap, keys: HashSet<KeyCode>) -> InputAction {
    let mut result = InputAction::None;
    for key in keys.iter() {
        let k = *key;
        if k == input_map.up {
            result = InputAction::Move(0);
            break;
        } else if k == input_map.down {
            result = InputAction::Move(1);
            break;
        } else if k == input_map.left {
            result = InputAction::Move(2);
            break;
        } else if k == input_map.right {
            result = InputAction::Move(3);
            break;
        } else if k == input_map.wait {
            result = InputAction::Wait;
            break;
        }
    }
    result
}
