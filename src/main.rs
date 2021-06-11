use console::Term;

use srl::game::Game;
use srl::gen::random_dungeon_generator::RandomDungeonGenerator;
use srl::gfx::terminal_renderer::TerminalRenderer;
use srl::input::{Input, TerminalInput};

const GAME_WIDTH: usize = 20;
const GAME_HEIGHT: usize = 10;

fn main() -> std::io::Result<()> {
    let mut game = make_example_game();
    let mut renderer = TerminalRenderer::new(GAME_WIDTH, GAME_HEIGHT);
    let mut input = TerminalInput::new();
    let mut term = Term::buffered_stdout();
    while !input.quit_game() {
        game.render(&mut renderer);
        renderer.flush(&mut term);
        input.on_key(term.read_key()?);
        game.tick(&input);
        term.clear_last_lines(GAME_HEIGHT - 1)?;
    }
    term.clear_to_end_of_screen()
}

fn make_example_game() -> Game {
    let generator = RandomDungeonGenerator::new(GAME_WIDTH, GAME_HEIGHT, 20, 10);
    let mut game = Game::generate_with(&generator);
    let mut walls: Vec<(u32, u32)> = Vec::new();
    for y in 0..GAME_HEIGHT {
        walls.push((0, y as u32));
        walls.push((GAME_WIDTH as u32 - 1, y as u32));
        for x in 0..GAME_WIDTH {
            walls.push((x as u32, 0));
            walls.push((x as u32, GAME_HEIGHT as u32 - 1));
        }
    }

    game.add_walls(&walls);
    game.set_player_position(3, 2);
    game
}
