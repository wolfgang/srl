use crate::combat::combat_engine::CombatEngine;
use crate::combat::combat_event::CombatEvent;
use crate::combat::combat_event_death::CombatEventDeath;
use crate::combat::combat_event_hit::CombatEventHit;
use crate::combat::combat_event_miss::CombatEventMiss;
use crate::combat::randomized_combat_engine::RandomizedCombatEngine;
use crate::game::dungeon::{DungeonCoords, DungeonRef};
use crate::game::object_type::ObjectType::{Enemy, Player};
use crate::input::move_direction::MoveDirection;

pub struct CombatResolver {
    dungeon_ref: DungeonRef,
    combat_engine: Box<dyn CombatEngine>,
    combat_events: Vec<Box<dyn CombatEvent>>,
}

impl CombatResolver {
    pub fn new(dungeon_ref: DungeonRef) -> Self {
        Self {
            dungeon_ref,
            combat_events: Vec::with_capacity(16),
            combat_engine: Box::from(RandomizedCombatEngine {}),
        }
    }

    pub fn override_combat_engine<T: 'static + CombatEngine>(&mut self, engine: T) {
        self.combat_engine = Box::from(engine);
    }

    pub fn reset(&mut self) {
        self.combat_events.clear();
    }

    pub fn get_combat_events(&self) -> &Vec<Box<dyn CombatEvent>> {
        &self.combat_events
    }


    pub fn handle_combat_with(&mut self, coords: DungeonCoords, direction: MoveDirection) {
        let player_pos = self.dungeon_ref.borrow().get_player_position();
        if self.combat_engine.is_hit(player_pos, coords) {
            let player_damage = self.combat_engine.roll_damage(player_pos);
            self.add_combat_event(CombatEventHit::new(Player, Enemy, player_damage));
            let remaining_hp = self.dungeon_ref.borrow_mut().apply_damage(coords, player_damage);
            if remaining_hp <= 0 {
                self.dungeon_ref.borrow_mut().remove_enemy(coords);
                self.dungeon_ref.borrow_mut().move_player(direction);
                self.add_combat_event(CombatEventDeath::new(Enemy));

                return;
            }
        } else {
            self.add_combat_event(CombatEventMiss::new(Player, Enemy));
        }

        if self.combat_engine.is_hit(coords, player_pos) {
            let enemy_damage = self.combat_engine.roll_damage(coords);
            self.add_combat_event(CombatEventHit::new(Enemy, Player, enemy_damage));
        } else {
            self.add_combat_event(CombatEventMiss::new(Enemy, Player));
        }
    }

    fn add_combat_event<T: CombatEvent + 'static>(&mut self, event: T) {
        self.combat_events.push(Box::from(event));
    }
}