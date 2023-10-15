use console_input::keypress::enable_raw_mode;
use gemini_engine::gameloop::MainLoopRoot;
use tetris::Game;

const FPS: f32 = 60.0;
const BLOCK_PLACE_COOLDOWN: u32 = 15;
const PIECE_PREVIEW_COUNT: usize = 3;
const CONTROLS_HELP_TEXT: &str = "Controls:
C to hold
Left/Right to shift
Space hard | Down soft
Z AC | Up/X C rotation
Esc to pause";

fn main() {
    enable_raw_mode();

    let mut game = Game::new(
        BLOCK_PLACE_COOLDOWN,
        PIECE_PREVIEW_COUNT,
        CONTROLS_HELP_TEXT,
    );
    game.main_loop(FPS);
}
