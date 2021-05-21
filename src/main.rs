use console::Term;

use srl::game::Game;
use srl::game::GameConfig;
use srl::gfx::terminal_renderer::TerminalRenderer;
use srl::input::{Input, TerminalInput};

const GAME_WIDTH: usize = 5;
const GAME_HEIGHT: usize = 4;

fn main() {

    let config = GameConfig {
        dungeon_size: (GAME_WIDTH, GAME_HEIGHT)
    };

    let mut game = Game::new(config);
    game.add_enemies(&vec![(1, 1), (4, 2), (2, 3)]);
    game.add_walls(&vec![(1, 0), (2, 0), (3, 1), (1, 2)]);
    game.set_player_position(3, 2);
    let mut renderer = TerminalRenderer::new(GAME_WIDTH, GAME_HEIGHT);
    let mut input = TerminalInput::new();

    let mut term = Term::stdout();
    term.hide_cursor().unwrap();
    while !input.quit_game() {
        game.render(&mut renderer);
        renderer.flush(&mut term);
        input.on_key(term.read_key().unwrap());
        game.tick(&input);
        term.clear_last_lines(GAME_HEIGHT-1).unwrap();
    }

}
