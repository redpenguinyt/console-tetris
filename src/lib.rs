use crossterm::event::{poll, read, Event};
use gemini_engine::elements::{view::ColChar, PixelContainer, Rect, Vec2D};
use std::{
    thread,
    time::{Duration, Instant},
};
pub mod blocks;

pub fn wait_fps_with_event(fps: f32, elapsed: Option<Duration>) -> (bool, Option<Event>) {
    let elapsed = elapsed.unwrap_or_default();
    let mut fps_reciprocal = Duration::from_secs_f32(1.0 / fps);
    let frame_skip = if elapsed < fps_reciprocal {
        fps_reciprocal = fps_reciprocal - elapsed;
        false
    } else {
        true
    };

    let now = Instant::now();
    let does_event_exist = poll(fps_reciprocal).unwrap_or(false);
    let event_elapsed = now.elapsed();

    if event_elapsed < fps_reciprocal {
        thread::sleep(fps_reciprocal - event_elapsed);
    } else {
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
