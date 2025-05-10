use std::cell::RefCell;

use arbitrary_code_execution_game::{
    entities::{Entity, Mover, Player},
    input::{InputAction, InputMap, get_turn_input},
};
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
    let mut entities: Vec<RefCell<Box<dyn Entity>>> = Vec::new();
    let mut background_color: Color = BLACK;

    entities.push(RefCell::new(Box::new(Player::new(IVec2 { x: 5, y: 5 }))));
    entities.push(RefCell::new(Box::new(Mover::new(
        IVec2 { x: 7, y: 5 },
        IVec2 { x: -1, y: 0 },
    ))));

    loop {
        let delta = get_frame_time();
        //UPDATE
        let action = get_turn_input(&input_map, get_keys_pressed());
        match action {
            InputAction::None => {}
            _ => {
                for ent in entities.iter() {
                    let result: Result<(), String> = ent.borrow_mut().update(&action, &entities);
                    if let Err(e) = result {
                        eprintln!("Error while updating entity: {e}")
                    };
                }
            }
        }
        //RENDER
        {
            clear_background(background_color);
            for ent in entities.iter() {
                let result: Result<(), String> = ent.borrow().render(delta);
                if let Err(e) = result {
                    eprintln!("Error while rendering entity: {e}")
                }
            }
        }

        next_frame().await;
    }
}
