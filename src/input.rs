use std::collections::HashSet;

use macroquad::{
    input::{KeyCode, is_key_down},
    math::{IVec2, Vec2},
};

pub struct InputMap {
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub wait: KeyCode,
    pub undo: KeyCode,
    pub pause: KeyCode,
}

pub enum InputAction {
    Move(IVec2),
    Wait,
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
            result = InputAction::Move(IVec2 { x: 0, y: -1 });
            break;
        } else if k == input_map.down {
            result = InputAction::Move(IVec2 { x: 0, y: 1 });
            break;
        } else if k == input_map.left {
            result = InputAction::Move(IVec2 { x: -1, y: 0 });
            break;
        } else if k == input_map.right {
            result = InputAction::Move(IVec2 { x: 1, y: 0 });
            break;
        } else if k == input_map.wait {
            result = InputAction::Wait;
            break;
        }
    }
    result
}
