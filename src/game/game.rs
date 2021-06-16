use crate::combat::combat_engine::CombatEngine;
use crate::combat::combat_event::CombatEvent;
use crate::combat::randomized_combat_engine::RandomizedCombatEngine;
use crate::game::dungeon::Dungeon;
use crate::game::object_type::ObjectType::{Enemy, Player};
use crate::gen::dungeon_generator::DungeonGenerator;
use crate::gfx::renderer::Renderer;
use crate::input::Input;
use crate::input::move_direction::MoveDirection;

pub struct Game {
    dungeon: Dungeon,
    combat_engine: Box<dyn CombatEngine>,
    combat_events: Vec<CombatEvent>,
}

impl Game {
    pub fn generate_with<T: DungeonGenerator>(generator: &T) -> Self {
        Self {
            combat_engine: Box::from(RandomizedCombatEngine {}),
            dungeon: generator.generate(),
            combat_events: Vec::new(),
        }
    }

    pub fn override_combat_engine<T: 'static + CombatEngine>(&mut self, engine: T) {
        self.combat_engine = Box::from(engine);
    }

    pub fn tick<T: Input>(&mut self, input: &T) {
        self.combat_events.clear();
        for direction in MoveDirection::iter() {
            if input.wants_to_move(*direction) {
                if let Some((coords, Enemy)) = self.dungeon.move_player(*direction) {
                    self.record_combat_with(coords)
                }
            }
        }
    }

    pub fn render<T: Renderer>(&self, renderer: &mut T) {
        renderer.clear();
        for ((x, y), object_type) in self.dungeon.get_objects() {
            renderer.render_tile(*x, *y, *object_type);
        }

        let (player_x, player_y) = self.dungeon.get_player_position();
        renderer.render_tile(player_x, player_y, Player);

        for evt in self.combat_events.iter() {
            renderer.append_combat_log(evt.log_string().as_str())
        }
    }


    fn record_combat_with(&mut self, coords: (u32, u32)) {
        if self.combat_engine.is_hit(self.dungeon.get_player_position(), coords) {
            let player_damage = self.combat_engine.roll_damage(self.dungeon.get_player_position());
            self.combat_events.push(CombatEvent::hit(Player, Enemy, player_damage));
        } else {
            self.combat_events.push(CombatEvent::miss(Player, Enemy));
        }

        if self.combat_engine.is_hit(coords, self.dungeon.get_player_position()) {
            let enemy_damage = self.combat_engine.roll_damage(coords);
            self.combat_events.push(CombatEvent::hit(Enemy, Player, enemy_damage));
        } else {
            self.combat_events.push(CombatEvent::miss(Enemy, Player));
        }
    }
}