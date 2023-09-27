use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    terminal::enable_raw_mode,
};
use gemini_engine::elements::{
    containers::CollisionContainer,
    view::{ColChar, Modifier, Wrapping},
    PixelContainer, Sprite, Text, Vec2D, View,
};
use rand::Rng;
use tetris::blocks::{Block as TetrisBlock, BlockType};
use tetris::event_gameloop;

const FPS: f32 = 30.0;
const BLOCK_PLACE_COOLDOWN: u32 = 15;
const PIECE_PREVIEW_COUNT: usize = 3;
const CONTROLS_HELP_TEXT: &str = "Controls:
C to hold
Left/Right to shift
Space hard | Down soft
Z AC | Up/X C rotation
Esc to pause";

fn main() {
    let mut view = View::new(50, 21, ColChar::EMPTY);
    let game_boundaries = tetris::generate_borders();
    let mut stationary_blocks = PixelContainer::new();

    let mut held_piece = None;
    let mut has_held = false;

    let mut active_block: Option<TetrisBlock> = None;
    let mut ghost_block: TetrisBlock = TetrisBlock::DEFAULT;
    let mut i = 0;

    enable_raw_mode().unwrap();

    let mut bag = BlockType::bag()[0..rand::thread_rng().gen_range(1..8)].to_vec();
    let mut block_speed = 12;

    let mut placing_cooldown = 0;

    let level = 1;
    let mut score = 0;

    event_gameloop!(
        |event: Option<Event>| {
            block_speed = 12;

            let collision =
                CollisionContainer::from(vec![&game_boundaries as _, &stationary_blocks as _]);

            let mut block = match active_block {
                Some(ref block) => block.clone(),
                None => {
                    let next_piece = bag.pop().unwrap();
                    if bag.len() <= PIECE_PREVIEW_COUNT {
                        bag.extend(BlockType::bag());
                    }

                    TetrisBlock::new(next_piece)
                }
            };

            // Handle user input
            if let Some(Event::Key(key_event)) = event {
                match key_event {
                    KeyEvent {
                        code: KeyCode::Esc,
                        kind: KeyEventKind::Press,
                        ..
                    } => {
                        view.clear();
                        view.display_render().unwrap();
                        tetris::pause()
                    }

                    KeyEvent {
                        code: KeyCode::Left, // Shift left
                        kind: KeyEventKind::Press,
                        ..
                    } => {
                        if tetris::try_move_block(&collision, &mut block, Vec2D::new(-1, 0)) {
                            placing_cooldown = BLOCK_PLACE_COOLDOWN;
                        }
                    }

                    KeyEvent {
                        code: KeyCode::Right, // Shift right
                        kind: KeyEventKind::Press,
                        ..
                    } => {
                        if tetris::try_move_block(&collision, &mut block, Vec2D::new(1, 0)) {
                            placing_cooldown = BLOCK_PLACE_COOLDOWN;
                        }
                    }

                    KeyEvent {
                        code: KeyCode::Char('z'), // Rotate Anti-clockwise
                        kind: KeyEventKind::Press,
                        ..
                    } => {
                        if tetris::try_rotate_block(&collision, &mut block, false) {
                            placing_cooldown = BLOCK_PLACE_COOLDOWN;
                        }
                    }

                    KeyEvent {
                        code: KeyCode::Up | KeyCode::Char('x'), // Rotate Clockwise
                        kind: KeyEventKind::Press,
                        ..
                    } => {
                        if tetris::try_rotate_block(&collision, &mut block, true) {
                            placing_cooldown = BLOCK_PLACE_COOLDOWN;
                        }
                    }

                    KeyEvent {
                        code: KeyCode::Down, // Soft Drop
                        kind: KeyEventKind::Press,
                        ..
                    } => block_speed = 2,

                    KeyEvent {
                        code: KeyCode::Char(' '), // Hard drop
                        kind: KeyEventKind::Press,
                        ..
                    } => {
                        ghost_block = tetris::generate_ghost_block(&collision, &block);
                        score += ghost_block.pos.y - block.pos.y;
                        block = ghost_block.clone();
                        i = block_speed - 1;
                        placing_cooldown = 1;
                    }

                    KeyEvent {
                        code: KeyCode::Char('c'), // Hold
                        modifiers: KeyModifiers::NONE,
                        kind: KeyEventKind::Press,
                        ..
                    } => {
                        if !has_held {
                            let current_held_piece = held_piece;
                            held_piece = Some(block.block_shape);
                            match current_held_piece {
                                Some(piece) => block = TetrisBlock::new(piece),
                                None => {
                                    active_block = None;
                                    return false;
                                }
                            }
                            has_held = true;
                        }
                    }

                    KeyEvent {
                        code: KeyCode::Char('c'), // Close
                        modifiers: KeyModifiers::CONTROL,
                        kind: KeyEventKind::Press,
                        ..
                    } => tetris::exit(),

                    _ => (),
                }
            }

            ghost_block = tetris::generate_ghost_block(&collision, &block);

            let is_above_block = collision.will_overlap_element(&block, Vec2D::new(0, 1));

            i += 1;
            active_block = if i % block_speed == 0 || is_above_block {
                if tetris::try_move_block(&collision, &mut block, Vec2D::new(0, 1)) {
                    if block_speed == 2 {
                        score += 1;
                    }
                    Some(block)
                } else {
                    placing_cooldown -= 1;
                    if placing_cooldown == 0 {
                        has_held = false;
                        stationary_blocks.blit(&block);
                        if block.pos.y < 1 {
                            return true;
                        }
                        let cleared_lines = tetris::clear_filled_lines(&mut stationary_blocks);
                        if cleared_lines > 0 {
                            score += (cleared_lines * 2 - 1) * 100 * level;
                        }
                        None
                    } else {
                        Some(block)
                    }
                }
            } else {
                Some(block)
            };

            false
        },
        {
            view.clear();
            view.blit(&game_boundaries, Wrapping::Panic);
            view.blit(&stationary_blocks, Wrapping::Ignore);
            view.blit(&ghost_block, Wrapping::Ignore);
            if let Some(ref block) = active_block {
                view.blit(block, Wrapping::Ignore);
            }

            // Next piece display
            view.blit(
                &Text::new(Vec2D::new(29, 9), "Next:", Modifier::None),
                Wrapping::Panic,
            );

            for i in 0..PIECE_PREVIEW_COUNT {
                let mut next_block_display = TetrisBlock::new(bag[bag.len() - i - 1]);
                next_block_display.pos = Vec2D::new(15, 12 + i as isize * 3);
                view.blit(&next_block_display, Wrapping::Ignore);
            }

            // Held piece display
            if let Some(piece) = held_piece {
                view.blit(
                    &Text::new(Vec2D::new(29, 1), "Hold", Modifier::None),
                    Wrapping::Panic,
                );
                let mut held_block_display = TetrisBlock::new(piece);
                held_block_display.pos = Vec2D::new(15, 4);
                view.blit(&held_block_display, Wrapping::Panic);
            } else {
                view.blit(
                    &Sprite::new(Vec2D::new(26, 0), CONTROLS_HELP_TEXT, Modifier::None),
                    Wrapping::Panic,
                );
            }

            // Score display
            let mut score_display = String::from("Score: ");
            score_display.push_str(&score.to_string());
            view.blit(
                &Text::new(Vec2D::new(26, 7), &score_display, Modifier::None),
                Wrapping::Panic,
            );

            view.display_render().unwrap();
        },
        FPS
    );

    println!("Game over!\r");
    tetris::exit()
}
