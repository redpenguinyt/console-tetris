use gemini_engine::elements::view::{utils, ColChar, Colour, Point, Vec2D, ViewElement};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BlockType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl BlockType {
    fn random() -> BlockType {
        let mut rng = rand::thread_rng();
        return match rng.gen_range(1..=2) {
            1 => BlockType::I,
            2 => BlockType::O,
            _ => panic!("Error getting random block type"),
        };

        match rng.gen_range(1..=7) {
            1 => BlockType::I,
            2 => BlockType::J,
            3 => BlockType::L,
            4 => BlockType::O,
            5 => BlockType::S,
            6 => BlockType::T,
            7 => BlockType::Z,
            _ => panic!("Error getting random block type"),
        }
    }

    fn get_data(&self) -> (Colour, Vec<Vec2D>) {
        match self {
            BlockType::O => (
                Colour::rgb(255, 255, 0),
                vec![
                    Vec2D::new(0, 0),
                    Vec2D::new(1, 0),
                    Vec2D::new(0, 1),
                    Vec2D::new(1, 1),
                ],
            ),
			BlockType::I => (
				Colour::rgb(0, 255, 255),
				vec![
					Vec2D::new(-1, 1),
					Vec2D::new(0, 1),
					Vec2D::new(1, 1),
					Vec2D::new(2, 1),
				]
			),
            _ => unimplemented!(),
        }
    }
    fn get_points(&self) -> Vec<Vec2D> {
        self.get_data().1.clone()
    }
    fn get_colour(&self) -> ColChar {
        ColChar::SOLID.with_colour(self.get_data().0)
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub pos: Vec2D,
    block_shape: BlockType,
    rotation: f32,
}

impl Block {
    pub fn new() -> Block {
        Block {
            pos: Vec2D::new(5, 0),
            block_shape: BlockType::random(),
            rotation: 0.0,
        }
    }

    pub fn rot_c(&mut self) {
        self.rotation += 90.0
    }
    pub fn rot_ac(&mut self) {
        self.rotation -= 90.0
    }
}

impl ViewElement for Block {
    fn active_pixels(&self) -> Vec<Point> {
        let block_points = self.block_shape.get_points();
        let block_colour = self.block_shape.get_colour();

        // Rotate points around (0.5, 0.5)
        let cr = self.rotation.to_radians().cos().round();
        let sr = self.rotation.to_radians().sin().round();
        let block_points = block_points
            .iter()
            .flat_map(|p| {
                let pf = (p.x as f32 - 0.5, p.y as f32 - 0.5);
                let rotated = Vec2D::new(
                    (pf.0 * cr - pf.1 * sr + 0.5).floor() as isize,
                    (pf.1 * cr + pf.0 * sr + 0.5).floor() as isize,
                );
                let mut positioned = rotated + self.pos;
				positioned.x *= 2;
				vec![positioned, positioned + Vec2D::new(1, 0)]
            })
            .collect();

        utils::points_to_pixels(block_points, block_colour)
    }
}
