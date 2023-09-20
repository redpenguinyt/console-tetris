use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use gemini_engine::elements::{
    containers::CollisionContainer,
    view::{ColChar, Wrapping},
    PixelContainer, Vec2D, View,
};
use rand::Rng;
use tetris::blocks::{Block as TetrisBlock, BlockType};
use tetris::event_gameloop;

const FPS: f32 = 30.0;

fn try_move_block(collision: &CollisionContainer, block: &mut TetrisBlock, offset: Vec2D) -> bool {
    let did_move = !collision.will_overlap_element(block, offset);
    if did_move {
        block.pos += offset
    }

    did_move
}

fn main() {
    let mut view = View::new(30, 21, ColChar::EMPTY);
    let game_boundaries = tetris::generate_borders();
    let mut stationary_blocks = PixelContainer::new();

    let mut active_block: Option<TetrisBlock> = None;
    let mut i = 0;

    enable_raw_mode().unwrap();

    let mut bag = BlockType::bag()[0..rand::thread_rng().gen_range(0..8)].to_vec();
    let mut block_speed = 12;

    event_gameloop!(
        |event: Option<Event>| {
            block_speed = 12;

            let collision =
                CollisionContainer::from(vec![&game_boundaries as _, &stationary_blocks as _]);

            let mut block = match active_block {
                Some(ref block) => block.clone(),
                None => {
                    if bag.is_empty() {
                        bag = BlockType::bag().to_vec();
                    }
                    TetrisBlock::new(bag.pop().unwrap())
                }
            };

            if let Some(Event::Key(key_event)) = event {
                match key_event {
                    KeyEvent {
                        code: KeyCode::Left,
                        modifiers: _,
                        kind: _,
                        state: _,
                    } => {
                        try_move_block(&collision, &mut block, Vec2D::new(-1, 0));
                    }
                    KeyEvent {
                        code: KeyCode::Right,
                        modifiers: _,
                        kind: _,
                        state: _,
                    } => {
                        try_move_block(&collision, &mut block, Vec2D::new(1, 0));
                    }
                    KeyEvent {
                        code: KeyCode::Up,
                        modifiers: _,
                        kind: _,
                        state: _,
                    } => {
                        let mut hypothetical_block = block.clone();
                        hypothetical_block.rotate();
                        if !collision.overlaps_element(&hypothetical_block) {
                            block.rotate();
                        }
                    }
                    KeyEvent {
                        code: KeyCode::Down,
                        modifiers: _,
                        kind: _,
                        state: _,
                    } => {
                        block_speed = 2;
                    }
                    KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        kind: _,
                        state: _,
                    } => return true,
                    _ => (),
                }
            }

            i += 1;
            active_block = if i % block_speed == 0 {
                if try_move_block(&collision, &mut block, Vec2D::new(0, 1)) {
                    Some(block)
                } else {
                    stationary_blocks.blit(&block);
                    if block.pos.y < 1 {
                        return true;
                    }
                    tetris::clear_filled_lines(&mut stationary_blocks);
                    None
                }
            } else {
                Some(block)
            };

            false
        },
        {
            view.clear();
            view.blit(&game_boundaries, Wrapping::Panic);
            view.blit(&stationary_blocks, Wrapping::Panic);
            if let Some(ref block) = active_block {
                view.blit(block, Wrapping::Ignore);
            }

            view.display_render().unwrap();
            println!("{:?}\r", stationary_blocks.pixels
            .iter()
            .filter(|p| p.pos.y == 19)
            .map(|p| p.pos.x)
            .collect::<Vec<isize>>());
        },
        FPS
    );

    disable_raw_mode().unwrap();
    println!("Game over!\r");
}
