use blocks::{Block, BlockType};
use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    terminal::disable_raw_mode,
};
use gemini_engine::elements::{
    containers::CollisionContainer, view::ColChar, PixelContainer, Rect, Vec2D,
};
use std::{
    process, thread,
    time::{Duration, Instant},
};
pub mod blocks;

pub fn wait_fps_with_event(fps: f32, elapsed: Option<Duration>) -> (bool, Option<Event>) {
    let elapsed = elapsed.unwrap_or_default();
    let mut fps_reciprocal = Duration::from_secs_f32(1.0 / fps);
    let frame_skip = if elapsed < fps_reciprocal {
        fps_reciprocal -= elapsed;
        false
    } else {
        true
    };

    let now = Instant::now();
    let does_event_exist = poll(fps_reciprocal).unwrap_or(false);
    let event_elapsed = now.elapsed();

    if event_elapsed < fps_reciprocal {
        thread::sleep(fps_reciprocal - event_elapsed);
    };

    match does_event_exist {
        true => (frame_skip, Some(read().unwrap())),
        false => (frame_skip, None),
    }
}

#[macro_export]
macro_rules! event_gameloop {
    ($logic:expr, $render:block, $fps:expr) => {
        event_gameloop!($logic, $render, $fps, |_, _| ());
    };
    ($logic:expr, $render:block, $fps:expr, $handle_elapsed:expr) => {
        use gemini_engine::gameloop::Instant;
        use tetris::wait_fps_with_event;
        let mut frame_skip = false;
        let mut event = None;

        loop {
            let now = Instant::now();

            if $logic(event) {
                break;
            }; // Logic

            match frame_skip {
                true => frame_skip = false,
                false => {
                    $render;
                } // Rendering
            }

            // Debug info and such
            ($handle_elapsed)(now.elapsed(), frame_skip);

            let elapsed = now.elapsed();
            (frame_skip, event) = wait_fps_with_event($fps, Some(elapsed));
        }
    };
}

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

pub fn generate_ghost_block(collision: &CollisionContainer, block: &Block) -> Block {
    let mut ghost_block = block.clone();
    ghost_block.is_ghost = true;

    while try_move_block(collision, &mut ghost_block, Vec2D::new(0, 1)) {}

    ghost_block
}

pub fn clear_filled_lines(blocks: &mut PixelContainer) -> isize {
    let mut pixels = blocks.pixels.clone();
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

    blocks.pixels = pixels;

    cleared_lines
}

pub fn pause() {
    println!("-- Paused (Esc to unpause) --\r");
    loop {
        let pressed_key = Some(read().unwrap());
        if let Some(Event::Key(event_key)) = pressed_key {
            match event_key {
                KeyEvent {
                    code: KeyCode::Esc,
                    kind: KeyEventKind::Press,
                    ..
                } => break,
                KeyEvent {
                    code: KeyCode::Char('c'), // Close
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                    ..
                } => exit(),
                _ => (),
            }
        }
    }
}

pub fn exit() {
    disable_raw_mode().unwrap();
    process::exit(0);
}
