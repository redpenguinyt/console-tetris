use super::BlockType;
use gemini_engine::elements::{view::Colour, Vec2D};

pub(super) struct BlockData {
    pub rotation_states: Vec<Vec<Vec2D>>,
    pub colour: Colour,
}
impl BlockData {
    fn new(rotation_states: Vec<Vec<Vec2D>>, colour: Colour) -> BlockData {
        BlockData {
            rotation_states,
            colour,
        }
    }
}
impl From<BlockType> for BlockData {
    fn from(value: BlockType) -> Self {
        match value {
            BlockType::O => BlockData::new(
                vec![vec![
                    Vec2D::new(0, 0),
                    Vec2D::new(1, 0),
                    Vec2D::new(0, -1),
                    Vec2D::new(1, -1),
                ]],
                Colour::rgb(255, 255, 0),
            ),
            BlockType::I => BlockData::new(
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
            ),
            BlockType::T => BlockData::new(
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
            ),
            BlockType::S => BlockData::new(
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
            ),
            BlockType::Z => BlockData::new(
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
            ),
            BlockType::L => BlockData::new(
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
            ),
            BlockType::J => BlockData::new(
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
            ),
        }
    }
}
