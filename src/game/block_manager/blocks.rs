use std::collections::HashMap;

use gemini_engine::elements::view::{utils, ColChar, Pixel, Vec2D, ViewElement};
mod block_data;
pub mod block_manipulation;
use block_data::BlockData;
use rand::seq::SliceRandom;

const fn bool_to_polarity(value: bool) -> isize {
    if value {
        1
    } else {
        -1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl BlockType {
    const ALL_VARIANTS: [Self; 7] = [
        Self::I,
        Self::J,
        Self::L,
        Self::O,
        Self::S,
        Self::T,
        Self::Z,
    ];
    pub fn bag() -> [Self; 7] {
        let mut variants = Self::ALL_VARIANTS;
        variants.shuffle(&mut rand::thread_rng());
        variants
    }

    fn get_rotation_states(self) -> Vec<Vec<Vec2D>> {
        BlockData::from(self).rotation_states
    }
    fn get_colour(self) -> ColChar {
        // ColChar::EMPTY.with_char('▒') // Colourless
        ColChar::SOLID.with_colour(BlockData::from(self).colour)
    }
    pub(super) fn get_wall_kick_data(self) -> HashMap<(usize, usize), Vec<Vec2D>> {
        BlockData::from(self).wall_kick_data
    }
}

#[derive(Debug)]
pub struct Block {
    pub pos: Vec2D,
    pub shape: BlockType,
    pub rotation: usize,
    pub(super) is_ghost: bool,
}

impl Block {
    pub const DEFAULT: Self = Self::new(BlockType::O);

    pub const fn new(shape: BlockType) -> Self {
        Self {
            pos: Vec2D::new(5, 0),
            shape,
            rotation: 0,
            is_ghost: false,
        }
    }

    fn rot_state_len(&self) -> isize {
        self.shape.get_rotation_states().len() as isize
    }
    pub fn get_rotation_indexes(&self, clockwise: bool) -> (usize, usize) {
        (
            self.rotation,
            (self.rotation as isize + bool_to_polarity(clockwise)).rem_euclid(self.rot_state_len())
                as usize,
        )
    }
    pub fn rotate(&mut self, clockwise: bool) {
        self.rotation = (self.rotation as isize + bool_to_polarity(clockwise))
            .rem_euclid(self.rot_state_len()) as usize;
    }
}

impl Clone for Block {
    fn clone(&self) -> Self {
        Self {
            pos: self.pos,
            shape: self.shape,
            rotation: self.rotation,
            is_ghost: false,
        }
    }
}

impl ViewElement for Block {
    fn active_pixels(&self) -> Vec<Pixel> {
        let rotation_states = self.shape.get_rotation_states();
        let block_colour = if self.is_ghost {
            ColChar::BACKGROUND
        } else {
            self.shape.get_colour()
        };

        let block_points = rotation_states[self.rotation.rem_euclid(rotation_states.len())]
            .iter()
            .map(|p| *p + self.pos)
            .collect();

        utils::points_to_pixels(block_points, block_colour)
    }
}
