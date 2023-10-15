use std::io::stdout;

use console_input::keypress::{exit_raw_mode, Input};
use crossterm::{
    cursor::MoveTo,
    event::{Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{Clear, ClearType},
};
use gemini_engine::{
    elements::{
        containers::CollisionContainer,
        view::{ColChar, Modifier, Wrapping},
        Sprite, Text, Vec2D, View,
    },
    gameloop::MainLoopRoot,
};
mod alerts;
mod block_manager;
mod collision_manager;
mod pause;
use alerts::AlertDisplay;
use block_manager::{tetris_core, BlockManager};
use collision_manager::CollisionManager;
use pause::pause;

use self::alerts::generate_alert_for_filled_lines;

pub struct Game {
    view: View,
    alert_display: AlertDisplay,
    block_manager: BlockManager,
    collision_manager: CollisionManager,
    score: isize,
    t: usize,
    // Constants
    controls_help_text: String,
}

impl Game {
    pub fn new(
        block_place_cooldown: u32,
        piece_preview_count: usize,
        controls_help_text: &str,
    ) -> Game {
        Game {
            view: View::new(50, 21, ColChar::EMPTY),
            alert_display: AlertDisplay::new(Vec2D::new(12, 7)),
            block_manager: BlockManager::new(block_place_cooldown, piece_preview_count),
            collision_manager: CollisionManager::new(),
            score: 0,
            t: 0,
            // Constants
            controls_help_text: controls_help_text.to_string(),
        }
    }
}

impl MainLoopRoot for Game {
    type InputDataType = Event;

    fn frame(&mut self, input_data: Option<Self::InputDataType>) {
        let mut block_speed = 12;

        let collision = self.collision_manager.get();

        // If the event is a keypres...
        if let Some(Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        })) = input_data
        {
            match code {
                // Pause
                KeyCode::Esc => {
                    self.view.clear();
                    self.view.display_render().unwrap();
                    pause();
                }

                // Shift left
                KeyCode::Left => {
                    self.block_manager
                        .try_move_block(&collision, Vec2D::new(-1, 0));
                }

                // Shift right
                KeyCode::Right => {
                    self.block_manager
                        .try_move_block(&collision, Vec2D::new(1, 0));
                }

                // Rotate anti-clockwise
                KeyCode::Char('z') => {
                    self.block_manager.try_rotate_block(&collision, false);
                }

                // Rotate clockwise
                KeyCode::Up | KeyCode::Char('x') => {
                    self.block_manager.try_rotate_block(&collision, true);
                }

                // Soft drop
                KeyCode::Down => block_speed = 2,

                // Hard drop
                KeyCode::Char(' ') => {
                    self.block_manager.generate_ghost_block(&collision);
                    self.score +=
                        self.block_manager.ghost_block.pos.y - self.block_manager.block.pos.y;
                    self.block_manager.block = self.block_manager.ghost_block.clone();
                    self.t = block_speed - 1;
                    self.block_manager.placing_cooldown = 1;
                }

                KeyCode::Char('c') => {
                    if self.block_manager.hold() {
                        return;
                    }
                }

                _ => (),
            }
        }

        self.block_manager.generate_ghost_block(&collision);

        let is_above_block =
            collision.will_overlap_element(&self.block_manager.block, Vec2D::new(0, 1));

        self.t += 1;
        if self.t % block_speed == 0 || is_above_block {
            if self
                .block_manager
                .try_move_block(&collision, Vec2D::new(0, 1))
            {
                if block_speed == 2 {
                    self.score += 1;
                }
            } else {
                self.block_manager.placing_cooldown -= 1;
                if self.block_manager.placing_cooldown == 0 {
                    // Placing a block
                    let pre_clear_blocks = self.collision_manager.stationary_blocks.clone();

                    self.block_manager.reset_placing_cooldown();
                    self.block_manager.has_held = false;
                    self.collision_manager.blit(&self.block_manager.block);
                    if self.block_manager.block.pos.y < 1 {
                        println!("Game over!\r");
                        exit_raw_mode()
                    }

                    let cleared_lines = self.collision_manager.clear_filled_lines();

                    // Generate alert
                    let mut alert = generate_alert_for_filled_lines(cleared_lines);
                    if let Some(t_spin_alert) = tetris_core::handle_t_spin(
                        &CollisionContainer::from(vec![&pre_clear_blocks as _]),
                        &self.block_manager.block,
                        cleared_lines,
                    ) {
                        alert = Some(t_spin_alert)
                    }
                    self.alert_display.handle_with_score(&mut self.score, alert);
                    self.block_manager.generate_new_block();
                }
            }
        }
    }

    fn render_frame(&mut self) {
        self.view.clear();

        self.view
            .blit_double_width(&self.collision_manager.get(), Wrapping::Ignore);

        self.view
            .blit_double_width(&self.block_manager.ghost_block, Wrapping::Ignore);
        self.view
            .blit_double_width(&self.block_manager.block, Wrapping::Ignore);

        // Next piece display
        self.view.blit(
            &Text::new(Vec2D::new(29, 9), "Next:", Modifier::None),
            Wrapping::Panic,
        );
        self.view
            .blit_double_width(&self.block_manager.next_piece_display(), Wrapping::Ignore);

        // Held piece display
        if self.block_manager.held_piece.is_some() {
            self.view.blit(
                &Text::new(Vec2D::new(29, 1), "Hold", Modifier::None),
                Wrapping::Panic,
            );
            self.view.blit_double_width(
                &self.block_manager.held_piece_display().unwrap(),
                Wrapping::Ignore,
            );
        } else {
            self.view.blit(
                &Sprite::new(Vec2D::new(26, 0), &self.controls_help_text, Modifier::None),
                Wrapping::Panic,
            );
        }

        // Score display
        self.view.blit(
            &Text::new(
                Vec2D::new(26, 7),
                &format!("Score: {}", self.score),
                Modifier::None,
            ),
            Wrapping::Panic,
        );

        // Alerts display
        self.view.blit(&self.alert_display, Wrapping::Ignore);
        self.alert_display.frame();

        execute!(stdout(), MoveTo(0, 0)).unwrap();
        execute!(stdout(), Clear(ClearType::FromCursorDown)).unwrap();
        self.view.display_render().unwrap();
    }

    fn sleep_and_get_input_data(
        &self,
        fps: f32,
        elapsed: std::time::Duration,
    ) -> (bool, Option<Self::InputDataType>) {
        Input::sleep_fps_and_get_input(fps, elapsed)
            .exit_on_kb_interrupt()
            .as_tuple()
    }
}
