use gemini_engine::elements::{
    containers::CollisionContainer,
    view::{ColChar, ViewElement},
    PixelContainer, Rect, Vec2D,
};

pub fn generate_borders() -> PixelContainer {
    let mut borders = PixelContainer::new();
    borders.blit(&Rect::new(
        // Left wall
        Vec2D::new(0, 0),
        Vec2D::new(1, 21),
        ColChar::SOLID,
    ));
    borders.blit(&Rect::new(
        // Right wall
        Vec2D::new(11, 0),
        Vec2D::new(1, 21),
        ColChar::SOLID,
    ));
    borders.blit(&Rect::new(
        // Floor
        Vec2D::new(1, 20),
        Vec2D::new(10, 1),
        ColChar::SOLID,
    ));

    borders
}

pub struct CollisionManager {
    pub game_boundaries: PixelContainer,
    pub stationary_blocks: PixelContainer,
}

impl CollisionManager {
    pub fn new() -> CollisionManager {
        CollisionManager {
            game_boundaries: generate_borders(),
            stationary_blocks: PixelContainer::new(),
        }
    }

    pub fn get(&self) -> CollisionContainer<'_> {
        CollisionContainer::from(vec![
            &self.game_boundaries as _,
            &self.stationary_blocks as _,
        ])
    }

    pub fn blit<E: ViewElement>(&mut self, element: &E) {
        self.stationary_blocks.blit(element)
    }

    // Remove all filled lines and return the number of lines filled and removed
    pub fn clear_filled_lines(&mut self) -> isize {
        let mut pixels = self.stationary_blocks.pixels.clone();
        if pixels.is_empty() {
            return 0;
        }

        let mut cleared_lines = 0;

        let mut min_y = pixels.iter().map(|p| p.pos.y).min().unwrap();
        let max_y = pixels.iter().map(|p| p.pos.y).max().unwrap();

        'row: for y in min_y..=max_y {
            let row_pixels: Vec<isize> = pixels
                .iter()
                .filter(|p| p.pos.y == y)
                .map(|p| p.pos.x)
                .collect();

            for x in 1..11 {
                if !row_pixels.contains(&x) {
                    continue 'row;
                }
            }

            cleared_lines += 1;
            pixels.retain(|p| p.pos.y != y);
        }

        let mut y = max_y + 1;
        loop {
            y -= 1;
            if y < min_y {
                break;
            }

            let row_pixels: Vec<isize> = pixels
                .iter()
                .filter(|p| p.pos.y == y)
                .map(|p| p.pos.x)
                .collect();

            if row_pixels.is_empty() {
                pixels = pixels
                    .iter()
                    .map(|p| {
                        if p.pos.y < y {
                            let mut moved_p = *p;
                            moved_p.pos.y += 1;
                            moved_p
                        } else {
                            *p
                        }
                    })
                    .collect();

                y += 1;
                min_y += 1;
            }
        }

        self.stationary_blocks.pixels = pixels;

        cleared_lines
    }

    /// Add an element to the stationary blocks and clear all full lines
    ///
    /// Returns the number of cleared lines
    pub fn blit_and_clear_lines<E: ViewElement>(&mut self, block: &E) -> isize {
        self.blit(block);
        self.clear_filled_lines()
    }
}
