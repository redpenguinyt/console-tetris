use std::io::stdout;

use console_input::keypress::{exit_raw_mode, Input};
use crossterm::{
    cursor::MoveTo,
    event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
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
use block_manager::{tetris_core, Block, BlockManager};
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

        let mut block = self.block_manager.get_or_spawn_block();

        // Handle user input
        if let Some(Event::Key(key_event)) = input_data {
            match key_event {
                KeyEvent {
                    code: KeyCode::Esc,
                    kind: KeyEventKind::Press,
                    ..
                } => {
                    self.view.clear();
                    self.view.display_render().unwrap();
                    pause();
                }

                KeyEvent {
                    code: KeyCode::Left, // Shift left
                    kind: KeyEventKind::Press,
                    ..
                } => {
                    if tetris_core::try_move_block(&collision, &mut block, Vec2D::new(-1, 0)) {
                        self.block_manager.reset_placing_cooldown();
                    }
                }

                KeyEvent {
                    code: KeyCode::Right, // Shift right
                    kind: KeyEventKind::Press,
                    ..
                } => {
                    if tetris_core::try_move_block(&collision, &mut block, Vec2D::new(1, 0)) {
                        self.block_manager.reset_placing_cooldown();
                    }
                }

                KeyEvent {
                    code: KeyCode::Char('z'), // Rotate Anti-clockwise
                    kind: KeyEventKind::Press,
                    ..
                } => {
                    if tetris_core::try_rotate_block(&collision, &mut block, false) {
                        self.block_manager.reset_placing_cooldown();
                    }
                }

                KeyEvent {
                    code: KeyCode::Up | KeyCode::Char('x'), // Rotate Clockwise
                    kind: KeyEventKind::Press,
                    ..
                } => {
                    if tetris_core::try_rotate_block(&collision, &mut block, true) {
                        self.block_manager.reset_placing_cooldown();
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
                    self.block_manager.ghost_block =
                        tetris_core::generate_ghost_block(&collision, &block);
                    self.score += self.block_manager.ghost_block.pos.y - block.pos.y;
                    block = self.block_manager.ghost_block.clone();
                    self.t = block_speed - 1;
                    self.block_manager.placing_cooldown = 1;
                }

                KeyEvent {
                    code: KeyCode::Char('c'), // Hold
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                } => {
                    if self.block_manager.hold(&mut block) {
                        return;
                    }
                }

                _ => (),
            }
        }

        self.block_manager.generate_ghost_block(&collision, &block);

        let is_above_block = collision.will_overlap_element(&block, Vec2D::new(0, 1));

        self.t += 1;
        self.block_manager.active_block = if self.t % block_speed == 0 || is_above_block {
            if tetris_core::try_move_block(&collision, &mut block, Vec2D::new(0, 1)) {
                if block_speed == 2 {
                    self.score += 1;
                }
                Some(block)
            } else {
                self.block_manager.placing_cooldown -= 1;
                if self.block_manager.placing_cooldown == 0 {
                    // Placing a block
                    let pre_clear_blocks = self.collision_manager.stationary_blocks.clone();
                    self.block_manager.reset_placing_cooldown();
                    self.block_manager.has_held = false;
                    self.collision_manager.stationary_blocks.blit(&block);
                    if block.pos.y < 1 {
                        println!("Game over!\r");
                        exit_raw_mode()
                    }
                    let cleared_lines = tetris_core::clear_filled_lines(
                        &mut self.collision_manager.stationary_blocks,
                    );

                    let mut alert = generate_alert_for_filled_lines(cleared_lines);

                    if let Some(t_spin_alert) = tetris_core::handle_t_spin(
                        &CollisionContainer::from(vec![&pre_clear_blocks as _]),
                        &block,
                        cleared_lines,
                    ) {
                        alert = Some(t_spin_alert)
                    }

                    self.alert_display.handle_with_score(&mut self.score, alert);
                    None
                } else {
                    Some(block)
                }
            }
        } else {
            Some(block)
        };
    }

    fn render_frame(&mut self) {
        self.view.clear();
        self.view
            .blit_double_width(&self.collision_manager.game_boundaries, Wrapping::Panic);
        self.view
            .blit_double_width(&self.collision_manager.stationary_blocks, Wrapping::Ignore);
        self.view
            .blit_double_width(&self.block_manager.ghost_block, Wrapping::Ignore);
        if let Some(ref block) = self.block_manager.active_block {
            self.view.blit_double_width(block, Wrapping::Ignore);
        }

        // Next piece display
        self.view.blit(
            &Text::new(Vec2D::new(29, 9), "Next:", Modifier::None),
            Wrapping::Panic,
        );
        self.view
            .blit_double_width(&self.block_manager.next_piece_display(), Wrapping::Ignore);

        // Held piece display
        if let Some(piece) = self.block_manager.held_piece {
            self.view.blit(
                &Text::new(Vec2D::new(29, 1), "Hold", Modifier::None),
                Wrapping::Panic,
            );
            let mut held_block_display = Block::new(piece);
            held_block_display.pos = Vec2D::new(15, 4);
            self.view
                .blit_double_width(&held_block_display, Wrapping::Panic);
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
