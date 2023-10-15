use gemini_engine::elements::{containers::CollisionContainer, Vec2D, view::{utils, ViewElement}};

use super::{Block, BlockType};

pub fn try_move_block(collision: &CollisionContainer, block: &mut Block, offset: Vec2D) -> bool {
    let did_move = !collision.will_overlap_element(block, offset);
    if did_move {
        block.pos += offset
    }

    did_move
}

pub fn try_rotate_block(
    collision: &CollisionContainer,
    block: &mut Block,
    clockwise: bool,
) -> bool {
    if let BlockType::O = block.block_shape {
        return false;
    }

    let rotation_index = block.get_rotation_indexes(clockwise);
    let mut hypothetical_block = block.clone();
    hypothetical_block.rotate(clockwise);

    let mut did_move = false;
    for possible_offset in &block.block_shape.get_wall_kick_data()[&rotation_index] {
        hypothetical_block.pos = block.pos + *possible_offset;
        if !collision.overlaps_element(&hypothetical_block) {
            did_move = true;
            block.pos += *possible_offset;
            block.rotate(clockwise);
            break;
        }
    }

    did_move
}

pub fn handle_t_spin(
    collision: &CollisionContainer,
    block: &Block,
    cleared_lines: isize,
) -> Option<(isize, String)> {
    if let BlockType::T = block.block_shape {
        let collision_pixels = utils::pixels_to_points(collision.active_pixels());

        let positions_to_check: Vec<Vec2D> = [
            Vec2D::new(1, 1),   // Top-left
            Vec2D::new(1, -1),  // Top-right
            Vec2D::new(-1, 1),  // Bottom-left
            Vec2D::new(-1, -1), // Top-left
        ]
        .into_iter()
        .map(|o| block.pos + o)
        .collect();
        let mut counted_positions = 0;
        for pos in &positions_to_check {
            if collision_pixels.contains(pos) {
                counted_positions += 1;
            }
        }

        let blocked_from_top_right = if let 0..=2 = block.rotation {
            collision_pixels.contains(&positions_to_check[1])
        } else {
            false
        };
        let blocked_from_top_left = if let 0 | 2 | 3 = block.rotation {
            collision_pixels.contains(&positions_to_check[3])
        } else {
            false
        };

        if counted_positions > 2 && (blocked_from_top_left || blocked_from_top_right) {
            Some(match cleared_lines {
                0 => (400, String::from("T-Spin!")),
                1 => (800, String::from("T-Spin Single!")),
                2 => (1200, String::from("T-Spin Double!")),
                3 => (1600, String::from("T-Spin Triple!")),
                _ => (200, String::from("T-Spin?")),
            })
        } else {
            None
        }
    } else {
        None
    }
}
