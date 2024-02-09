use std::collections::HashMap;

use super::BlockType;
use gemini_engine::elements::{view::Colour, Vec2D};

pub(super) struct BlockData {
    pub rotation_states: Vec<Vec<Vec2D>>,
    pub colour: Colour,
    pub wall_kick_data: HashMap<(usize, usize), Vec<Vec2D>>,
}
impl BlockData {
    fn new(
        rotation_states: Vec<Vec<Vec2D>>,
        colour: Colour,
        wall_kick_data: HashMap<(usize, usize), Vec<Vec2D>>,
    ) -> Self {
        Self {
            rotation_states,
            colour,
            wall_kick_data,
        }
    }

    #[allow(clippy::too_many_lines)]
    fn get_wall_kick_data(block_shape: BlockType) -> HashMap<(usize, usize), Vec<Vec2D>> {
        match block_shape {
            BlockType::J | BlockType::L | BlockType::T | BlockType::S | BlockType::Z => {
                HashMap::from([
                    (
                        (0, 1),
                        vec![
                            Vec2D::ZERO,
                            Vec2D::new(-1, 0),
                            Vec2D::new(-1, -1),
                            Vec2D::new(0, 2),
                            Vec2D::new(-1, 2),
                        ],
                    ),
                    (
                        (1, 0),
                        vec![
                            Vec2D::ZERO,
                            Vec2D::new(1, 0),
                            Vec2D::new(1, 1),
                            Vec2D::new(0, -2),
                            Vec2D::new(1, -2),
                        ],
                    ),
                    (
                        (1, 2),
                        vec![
                            Vec2D::ZERO,
                            Vec2D::new(1, 0),
                            Vec2D::new(1, 1),
                            Vec2D::new(0, -2),
                            Vec2D::new(1, -2),
                        ],
                    ),
                    (
                        (2, 1),
                        vec![
                            Vec2D::ZERO,
                            Vec2D::new(-1, 0),
                            Vec2D::new(-1, -1),
                            Vec2D::new(0, 2),
                            Vec2D::new(-1, 2),
                        ],
                    ),
                    (
                        (2, 3),
                        vec![
                            Vec2D::ZERO,
                            Vec2D::new(1, 0),
                            Vec2D::new(1, -1),
                            Vec2D::new(0, 2),
                            Vec2D::new(1, 2),
                        ],
                    ),
                    (
                        (3, 2),
                        vec![
                            Vec2D::ZERO,
                            Vec2D::new(-1, 0),
                            Vec2D::new(-1, 1),
                            Vec2D::new(0, -2),
                            Vec2D::new(-1, -2),
                        ],
                    ),
                    (
                        (3, 0),
                        vec![
                            Vec2D::ZERO,
                            Vec2D::new(-1, 0),
                            Vec2D::new(-1, 1),
                            Vec2D::new(0, -2),
                            Vec2D::new(-1, -2),
                        ],
                    ),
                    (
                        (0, 3),
                        vec![
                            Vec2D::ZERO,
                            Vec2D::new(1, 0),
                            Vec2D::new(1, -1),
                            Vec2D::new(0, 2),
                            Vec2D::new(1, 2),
                        ],
                    ),
                ])
            }
            BlockType::I => HashMap::from([
                (
                    (0, 1),
                    vec![
                        Vec2D::ZERO,
                        Vec2D::new(-2, 0),
                        Vec2D::new(1, 0),
                        Vec2D::new(-2, 1),
                        Vec2D::new(1, -2),
                    ],
                ),
                (
                    (1, 0),
                    vec![
                        Vec2D::ZERO,
                        Vec2D::new(2, 0),
                        Vec2D::new(-1, 0),
                        Vec2D::new(2, -1),
                        Vec2D::new(-1, 2),
                    ],
                ),
                (
                    (1, 2),
                    vec![
                        Vec2D::ZERO,
                        Vec2D::new(-1, 0),
                        Vec2D::new(2, 0),
                        Vec2D::new(-1, -2),
                        Vec2D::new(2, 1),
                    ],
                ),
                (
                    (2, 1),
                    vec![
                        Vec2D::ZERO,
                        Vec2D::new(1, 0),
                        Vec2D::new(-2, 0),
                        Vec2D::new(1, 2),
                        Vec2D::new(-2, -1),
                    ],
                ),
                (
                    (2, 3),
                    vec![
                        Vec2D::ZERO,
                        Vec2D::new(2, 0),
                        Vec2D::new(-1, 0),
                        Vec2D::new(2, -1),
                        Vec2D::new(-1, 2),
                    ],
                ),
                (
                    (3, 2),
                    vec![
                        Vec2D::ZERO,
                        Vec2D::new(-2, 0),
                        Vec2D::new(1, 0),
                        Vec2D::new(-2, 1),
                        Vec2D::new(1, -2),
                    ],
                ),
                (
                    (3, 0),
                    vec![
                        Vec2D::ZERO,
                        Vec2D::new(1, 0),
                        Vec2D::new(-2, 0),
                        Vec2D::new(1, 2),
                        Vec2D::new(-2, -1),
                    ],
                ),
                (
                    (0, 3),
                    vec![
                        Vec2D::ZERO,
                        Vec2D::new(-1, 0),
                        Vec2D::new(2, 0),
                        Vec2D::new(-1, -2),
                        Vec2D::new(2, 1),
                    ],
                ),
            ]),
            BlockType::O => HashMap::new(),
        }
    }
}

impl From<BlockType> for BlockData {
    #[allow(clippy::too_many_lines)]
    fn from(block_shape: BlockType) -> Self {
        match block_shape {
            BlockType::O => Self::new(
                vec![vec![
                    Vec2D::new(0, 0),
                    Vec2D::new(1, 0),
                    Vec2D::new(0, -1),
                    Vec2D::new(1, -1),
                ]],
                Colour::rgb(255, 255, 0),
                Self::get_wall_kick_data(block_shape),
            ),
            BlockType::I => Self::new(
                vec![
                    vec![
                        Vec2D::new(-1, 0),
                        Vec2D::new(0, 0),
                        Vec2D::new(1, 0),
                        Vec2D::new(2, 0),
                    ],
                    vec![
                        Vec2D::new(1, -1),
                        Vec2D::new(1, 0),
                        Vec2D::new(1, 1),
                        Vec2D::new(1, 2),
                    ],
                    vec![
                        Vec2D::new(-1, 1),
                        Vec2D::new(0, 1),
                        Vec2D::new(1, 1),
                        Vec2D::new(2, 1),
                    ],
                    vec![
                        Vec2D::new(0, -1),
                        Vec2D::new(0, 0),
                        Vec2D::new(0, 1),
                        Vec2D::new(0, 2),
                    ],
                ],
                Colour::rgb(0, 255, 255),
                Self::get_wall_kick_data(block_shape),
            ),
            BlockType::T => Self::new(
                vec![
                    vec![
                        Vec2D::new(0, -1),
                        Vec2D::new(-1, 0),
                        Vec2D::new(0, 0),
                        Vec2D::new(1, 0),
                    ],
                    vec![
                        Vec2D::new(0, -1),
                        Vec2D::new(0, 0),
                        Vec2D::new(1, 0),
                        Vec2D::new(0, 1),
                    ],
                    vec![
                        Vec2D::new(-1, 0),
                        Vec2D::new(0, 0),
                        Vec2D::new(1, 0),
                        Vec2D::new(0, 1),
                    ],
                    vec![
                        Vec2D::new(0, -1),
                        Vec2D::new(-1, 0),
                        Vec2D::new(0, 0),
                        Vec2D::new(0, 1),
                    ],
                ],
                Colour::rgb(255, 0, 255),
                Self::get_wall_kick_data(block_shape),
            ),
            BlockType::S => Self::new(
                vec![
                    vec![
                        Vec2D::new(-1, 0),
                        Vec2D::new(0, 0),
                        Vec2D::new(0, -1),
                        Vec2D::new(1, -1),
                    ],
                    vec![
                        Vec2D::new(0, -1),
                        Vec2D::new(0, 0),
                        Vec2D::new(1, 0),
                        Vec2D::new(1, 1),
                    ],
                    vec![
                        Vec2D::new(0, 0),
                        Vec2D::new(1, 0),
                        Vec2D::new(-1, 1),
                        Vec2D::new(0, 1),
                    ],
                    vec![
                        Vec2D::new(0, 0),
                        Vec2D::new(0, 1),
                        Vec2D::new(-1, -1),
                        Vec2D::new(-1, 0),
                    ],
                ],
                Colour::rgb(0, 255, 0),
                Self::get_wall_kick_data(block_shape),
            ),
            BlockType::Z => Self::new(
                vec![
                    vec![
                        Vec2D::new(-1, -1),
                        Vec2D::new(0, -1),
                        Vec2D::new(0, 0),
                        Vec2D::new(1, 0),
                    ],
                    vec![
                        Vec2D::new(0, 0),
                        Vec2D::new(0, 1),
                        Vec2D::new(1, -1),
                        Vec2D::new(1, 0),
                    ],
                    vec![
                        Vec2D::new(-1, 0),
                        Vec2D::new(0, 0),
                        Vec2D::new(0, 1),
                        Vec2D::new(1, 1),
                    ],
                    vec![
                        Vec2D::new(0, -1),
                        Vec2D::new(0, 0),
                        Vec2D::new(-1, 0),
                        Vec2D::new(-1, 1),
                    ],
                ],
                Colour::rgb(255, 0, 0),
                Self::get_wall_kick_data(block_shape),
            ),
            BlockType::L => Self::new(
                vec![
                    vec![
                        Vec2D::new(-1, 0),
                        Vec2D::new(0, 0),
                        Vec2D::new(1, -1),
                        Vec2D::new(1, 0),
                    ],
                    vec![
                        Vec2D::new(0, -1),
                        Vec2D::new(0, 0),
                        Vec2D::new(0, 1),
                        Vec2D::new(1, 1),
                    ],
                    vec![
                        Vec2D::new(-1, 0),
                        Vec2D::new(-1, 1),
                        Vec2D::new(0, 0),
                        Vec2D::new(1, 0),
                    ],
                    vec![
                        Vec2D::new(-1, -1),
                        Vec2D::new(0, -1),
                        Vec2D::new(0, 0),
                        Vec2D::new(0, 1),
                    ],
                ],
                Colour::rgb(255, 165, 0),
                Self::get_wall_kick_data(block_shape),
            ),
            BlockType::J => Self::new(
                vec![
                    vec![
                        Vec2D::new(-1, -1),
                        Vec2D::new(-1, 0),
                        Vec2D::new(0, 0),
                        Vec2D::new(1, 0),
                    ],
                    vec![
                        Vec2D::new(0, -1),
                        Vec2D::new(1, -1),
                        Vec2D::new(0, 0),
                        Vec2D::new(0, 1),
                    ],
                    vec![
                        Vec2D::new(-1, 0),
                        Vec2D::new(0, 0),
                        Vec2D::new(1, 0),
                        Vec2D::new(1, 1),
                    ],
                    vec![
                        Vec2D::new(0, -1),
                        Vec2D::new(0, 0),
                        Vec2D::new(-1, 1),
                        Vec2D::new(0, 1),
                    ],
                ],
                Colour::rgb(0, 0, 255),
                Self::get_wall_kick_data(block_shape),
            ),
        }
    }
}
