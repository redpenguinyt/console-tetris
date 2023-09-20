use gemini_engine::{
    elements::{
        containers::CollisionContainer,
        view::{ColChar, Wrapping},
        PixelContainer, Rect, Vec2D, View,
    },
    fps_gameloop,
};
mod blocks;
use blocks::Block as TetrisBlock;

const FPS: f32 = 15.0;

fn main() {
    let mut view = View::new(30, 21, ColChar::EMPTY);
    let game_boundaries = generate_borders();
    let mut stationary_blocks = PixelContainer::new();

    let mut active_block: Option<TetrisBlock> = None;
    fps_gameloop!(
        {
            let mut collision = CollisionContainer::new();
            collision.push(&game_boundaries);
            collision.push(&stationary_blocks);

            let mut block = match active_block {
                Some(ref block) => block.clone(),
                None => TetrisBlock::new(),
            };

            let offset = Vec2D::new(0, 1);
            if collision.will_overlap_element(&block, offset) {
                stationary_blocks.blit(&block);
                if block.pos.y < 1 {
                    break;
                }
                active_block = None;
            } else {
                block.rot_c();
                block.pos += offset;
                active_block = Some(block);
            }
        },
        {
            view.clear();
            view.blit(&game_boundaries, Wrapping::Panic);
            view.blit(&stationary_blocks, Wrapping::Panic);
            if let Some(ref block) = active_block {
                view.blit(block, Wrapping::Ignore);
            }

            view.display_render().unwrap();
        },
        FPS
    );

    println!("Game over!")
}

fn generate_borders() -> PixelContainer {
    let mut borders = PixelContainer::new();
    borders.blit(&Rect::new(
        Vec2D::new(0, 0),
        Vec2D::new(1, 21),
        ColChar::SOLID,
    ));
    borders.blit(&Rect::new(
        Vec2D::new(21, 0),
        Vec2D::new(1, 21),
        ColChar::SOLID,
    ));
    borders.blit(&Rect::new(
        Vec2D::new(1, 20),
        Vec2D::new(20, 1),
        ColChar::SOLID,
    ));

    borders
}
