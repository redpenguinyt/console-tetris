use gemini_engine::elements::{
    containers::CollisionContainer, view::{ColChar, ViewElement}, PixelContainer, Rect, Vec2D,
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
}
