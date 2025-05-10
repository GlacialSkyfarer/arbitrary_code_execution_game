use arbitrary_code_execution_game::input::{InputAction, InputMap, get_turn_input};
use macroquad::prelude::*;

#[macroquad::main("ACEGame")]
async fn main() {
    let mut input_map = InputMap {
        up: KeyCode::Up,
        down: KeyCode::Down,
        left: KeyCode::Left,
        right: KeyCode::Right,
        wait: KeyCode::Space,
        undo: KeyCode::Z,
        pause: KeyCode::Escape,
    };
    let mut background_color: Color = BLACK;

    loop {
        let delta = get_frame_time();
        //UPDATE
        let action = get_turn_input(&input_map, get_keys_pressed());
        match action {
            InputAction::None => {}
            _ => {}
        }
        //RENDER
        {
            clear_background(background_color);
        }

        next_frame().await;
    }
}
