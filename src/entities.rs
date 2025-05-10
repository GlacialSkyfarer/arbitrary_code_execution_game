use std::{any::Any, cell::RefCell};

use super::DIRECTIONS;
use crate::input::InputAction;
use macroquad::{
    color::{GREEN, RED},
    math::IVec2,
    shapes::draw_rectangle,
};

pub trait Update {
    fn update(
        &mut self,
        _action: &InputAction,
        _entities: &Vec<RefCell<Box<dyn Entity>>>,
    ) -> Result<(), String> {
        Err(String::from("Update function was not implemented!"))
    }
}
pub trait Render {
    fn render(&self, _delta: f32) -> Result<(), String> {
        Err(String::from("Render function was not implemented!"))
    }
}
pub trait TilePosition {
    fn get_position(&self) -> IVec2;
    fn overlaps_position(&self, other: IVec2) -> bool {
        self.get_position() == other
    }
}

pub trait Entity: Update + Render + TilePosition + Any {}

pub struct Mover {
    direction: IVec2,
    position: IVec2,
}
impl Mover {
    pub fn new(position: IVec2, direction: IVec2) -> Self {
        Self {
            position,
            direction,
        }
    }
}
impl Update for Mover {
    fn update(
        &mut self,
        _action: &InputAction,
        entities: &Vec<RefCell<Box<dyn Entity>>>,
    ) -> Result<(), String> {
        match _action {
            InputAction::None => Ok(()),
            _ => {
                let desired_pos = self.position + self.direction;
                let mut has_flipped = false;
                for ent in entities {
                    if ent.borrow().overlaps_position(desired_pos) {
                        if has_flipped {
                            break;
                        }
                        has_flipped = true;
                        self.direction = -self.direction;
                    }
                }
                Ok(())
            }
        }
    }
}
impl Render for Mover {
    fn render(&self, _delta: f32) -> Result<(), String> {
        draw_rectangle(
            self.position.x as f32 * 20.0,
            self.position.y as f32 * 20.0,
            20.0,
            20.0,
            RED,
        );
        Ok(())
    }
}
impl Entity for Mover {}
impl TilePosition for Mover {
    fn get_position(&self) -> IVec2 {
        self.position
    }
}

pub struct Player {
    position: IVec2,
}
impl Player {
    pub fn new(position: IVec2) -> Self {
        Self { position }
    }
}
impl Update for Player {
    fn update(
        &mut self,
        _action: &InputAction,
        entities: &Vec<RefCell<Box<dyn Entity>>>,
    ) -> Result<(), String> {
        match _action {
            InputAction::Wait => {}
            InputAction::Move(dir) => {
                self.position = self.get_position() + DIRECTIONS[*dir];
            }
            _ => {}
        }

        Ok(())
    }
}
impl Render for Player {
    fn render(&self, _delta: f32) -> Result<(), String> {
        draw_rectangle(
            self.position.x as f32 * 20.0,
            self.position.y as f32 * 20.0,
            20.0,
            20.0,
            GREEN,
        );
        Ok(())
    }
}
impl Entity for Player {}
impl TilePosition for Player {
    fn get_position(&self) -> IVec2 {
        self.position
    }
}
