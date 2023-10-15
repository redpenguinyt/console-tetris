use gemini_engine::elements::{containers::CollisionContainer, PixelContainer, Vec2D};
mod blocks;
pub use blocks::{block_manipulation as tetris_core, Block, BlockType};
use rand::Rng;

pub struct BlockManager {
    bag: Vec<BlockType>,
    pub block: Block,
    pub ghost_block: Block,
    pub held_piece: Option<BlockType>,
    pub has_held: bool,
    pub placing_cooldown: u32,
    // Constants
    piece_preview_count: usize,
    block_place_cooldown: u32,
}

impl BlockManager {
    pub fn new(block_place_cooldown: u32, piece_preview_count: usize) -> BlockManager {
        let mut tmp = BlockManager {
            bag: BlockType::bag()[0..rand::thread_rng().gen_range(1..8)].to_vec(),
            block: Block::DEFAULT,
            ghost_block: Block::DEFAULT,
            held_piece: None,
            has_held: false,
            placing_cooldown: block_place_cooldown,
            block_place_cooldown,
            piece_preview_count,
        };
        tmp.generate_new_block();
        tmp
    }

    pub fn reset_placing_cooldown(&mut self) {
        self.placing_cooldown = self.block_place_cooldown;
    }

    pub fn generate_new_block(&mut self) {
        let next_piece = self.bag.pop().unwrap();
        if self.bag.len() <= self.piece_preview_count {
            let mut new_bag = BlockType::bag().to_vec();
            new_bag.extend(&self.bag);
            self.bag.clear();
            self.bag.extend(new_bag);
        }

        self.block = Block::new(next_piece);
    }

    /// Hold the current block. returns true if need to skip the rest of the frame
    pub fn hold(&mut self) -> bool {
        if !self.has_held {
            let current_held_piece = self.held_piece;
            self.held_piece = Some(self.block.block_shape);
            match current_held_piece {
                Some(piece) => self.block = Block::new(piece),
                None => {
                    self.generate_new_block();
                    // Skip the rest of the frame
                    return true;
                }
            }
            self.has_held = true;
        }

        false
    }

    pub fn generate_ghost_block(&mut self, collision: &CollisionContainer) {
        let mut ghost_block = self.block.clone();
        ghost_block.is_ghost = true;

        while tetris_core::try_move_block(collision, &mut ghost_block, Vec2D::new(0, 1)) {}

        self.ghost_block = ghost_block
    }

    pub fn next_piece_display(&self) -> PixelContainer {
        let mut container = PixelContainer::new();
        for i in 0..self.piece_preview_count {
            let mut next_block_display =
                Block::new(self.bag[self.bag.len() - i - 1]);
            next_block_display.pos = Vec2D::new(15, 12 + i as isize * 3);
            container.blit(&next_block_display);
        }

        container
    }

    pub fn held_piece_display(&self) -> Option<Block> {
        if let Some(piece) = self.held_piece {
            let mut held_block_display = Block::new(piece);
            held_block_display.pos = Vec2D::new(15, 4);
            Some(held_block_display)
        } else {
            None
        }
    }
}
