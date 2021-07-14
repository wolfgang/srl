use std::cell::RefCell;
use std::rc::Rc;

use crate::combat::combat_engine::CombatEngine;
use crate::combat::combat_resolver::CombatResolver;
use crate::game::dungeon::DungeonRef;
use crate::game::game::GameState::*;
use crate::game::object_type::ObjectType::{Enemy, Player};
use crate::gen::dungeon_generator::DungeonGenerator;
use crate::gfx::renderer::Renderer;
use crate::input::Input;
use crate::input::move_direction::MoveDirection;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GameState {
    Running,
    PlayerDied,
    AllEnemiesDied,
    PlayerQuit
}

pub struct Game {
    game_state: GameState,
    dungeon_ref: DungeonRef,
    combat_resolver: CombatResolver,
}

impl Game {
    pub fn generate_with<T: DungeonGenerator>(generator: &T) -> Self {
        let dungeon_ref = Rc::new(RefCell::new(generator.generate()));
        Self {
            game_state: Running,
            dungeon_ref: dungeon_ref.clone(),
            combat_resolver: CombatResolver::new(dungeon_ref.clone()),
        }
    }

    pub fn override_combat_engine<T: 'static + CombatEngine>(&mut self, engine: T) {
        self.combat_resolver.override_combat_engine(engine)
    }

    pub fn game_state(&self) -> GameState {
        self.game_state
    }

    pub fn tick<T: Input>(&mut self, input: &T) {
        self.combat_resolver.reset();
        for direction in MoveDirection::iter() {
            if input.wants_to_move(*direction) {
                self.resolve_possible_combat(direction)
            }
        }

        if input.quit_game() {
            self.game_state = PlayerQuit
        }
    }

    pub fn render<T: Renderer>(&self, renderer: &mut T) {
        renderer.clear();

        renderer.render_player_hp(self.dungeon_ref.borrow().get_player_hp());

        for ((x, y), object_type) in self.dungeon_ref.borrow().get_object_types() {
            renderer.render_tile(x, y, object_type);
        }

        let (player_x, player_y) = self.dungeon_ref.borrow().get_player_position();
        renderer.render_tile(player_x, player_y, Player);

        for evt in self.combat_resolver.get_combat_events() {
            renderer.append_combat_log(evt.log_string().as_str())
        }
    }

    fn resolve_possible_combat(&mut self, direction: &MoveDirection) {
        let result = self.dungeon_ref.borrow_mut().move_player(*direction);
        if let Some((coords, Enemy)) = result {
            self.combat_resolver.handle_combat_with(coords);
            if self.dungeon_ref.borrow().get_player_hp() <= 0 {
                self.game_state = PlayerDied;
            }

            if self.dungeon_ref.borrow_mut().get_num_enemies() == 0 {
                self.game_state = AllEnemiesDied;
            }
        }
    }
}