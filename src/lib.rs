use gemini_engine::elements::{
    view::ColChar,
    PixelContainer, Rect, Vec2D,
};

mod game;
pub use game::Game;

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