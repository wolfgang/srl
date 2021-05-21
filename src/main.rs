use console::Term;

use srl::game::Game;
use srl::game::GameConfig;
use srl::gfx::terminal_renderer::TerminalRenderer;

fn main() {
    let mut term = Term::stdout();

    let config = GameConfig {
        dungeon_size: (5, 4)
    };

    let mut game = Game::new(config);
    game.add_enemies(&vec![(1, 1), (4, 2), (2, 3)]);
    game.add_walls(&vec![(1, 0), (2, 0), (3, 1), (1, 2)]);
    game.set_player_position(3, 2);
    let mut renderer = TerminalRenderer::new(5, 4);
    game.render(&mut renderer);
    renderer.flush(&mut term);
}
