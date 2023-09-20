use gemini_engine::elements::view::{utils, ColChar, Point, Vec2D, ViewElement};
use rand::seq::SliceRandom;
mod block_data;
use block_data::BlockData;

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
    const ALL_VARIANTS: [BlockType; 7] = [
        BlockType::I,
        BlockType::J,
        BlockType::L,
        BlockType::O,
        BlockType::S,
        BlockType::T,
        BlockType::Z,
    ];
    pub fn bag() -> [BlockType; 7] {
        let mut variants = BlockType::ALL_VARIANTS;
        variants.shuffle(&mut rand::thread_rng());
        variants
    }

    fn get_rotation_states(self) -> Vec<Vec<Vec2D>> {
        BlockData::from(self).rotation_states.clone()
    }
    fn get_colour(self) -> ColChar {
        ColChar::SOLID.with_colour(BlockData::from(self).colour)
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub pos: Vec2D,
    block_shape: BlockType,
    rotation: usize,
}

impl Block {
    pub fn new(block_shape: BlockType) -> Block {
        Block {
            pos: Vec2D::new(5, 0),
            block_shape,
            rotation: 0,
        }
    }

    pub fn rotate(&mut self) {
        self.rotation += 1
    }
}

impl ViewElement for Block {
    fn active_pixels(&self) -> Vec<Point> {
        let rotation_states = self.block_shape.get_rotation_states();
        let block_colour = self.block_shape.get_colour();

        let block_points = rotation_states[self.rotation % rotation_states.len()]
            .iter()
            .flat_map(|p| {
                // Position block
                let mut positioned = *p + self.pos;

                // Widen block so that each pixels appears square
                positioned.x *= 2;
                vec![positioned, positioned + Vec2D::new(1, 0)]
            })
            .collect();

        utils::points_to_pixels(block_points, block_colour)
    }
}
