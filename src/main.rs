use console::Term;

use srl::game::Game;
use srl::game::game::GameState::{PlayerDied, Running};
use srl::gen::random_dungeon_generator::RandomDungeonGenerator;
use srl::gfx::terminal_renderer::TerminalRenderer;
use srl::input::{Input, TerminalInput};

const GAME_WIDTH: usize = 40;
const GAME_HEIGHT: usize = 20;

fn main() -> std::io::Result<()> {
    let mut game = make_example_game();
    let mut renderer = TerminalRenderer::new(GAME_WIDTH, GAME_HEIGHT);
    let mut input = TerminalInput::new();
    let mut term = Term::buffered_stdout();
    while !input.quit_game() && game.game_state() == Running {
        game.render(&mut renderer);
        let rendered_lines = renderer.flush(&mut term);
        input.on_key(term.read_key()?);
        game.tick(&input);
        term.clear_last_lines(rendered_lines - 1)?;
    }
    if game.game_state() == PlayerDied {
        game.render(&mut renderer);
        renderer.flush(&mut term);
        println!("\nYou died. Thanks for playing!");
    }
    term.clear_to_end_of_screen()
}

fn make_example_game() -> Game {
    let generator = RandomDungeonGenerator::new(GAME_WIDTH, GAME_HEIGHT, 60, 20);
    Game::generate_with(&generator)
}
