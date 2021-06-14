use crate::combat::combat_engine::CombatEngine;
use crate::game::dungeon::DungeonCoords;

pub struct ControlledCombatEngine {

}

impl ControlledCombatEngine {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn say_is_hit(&mut self, _attacker: DungeonCoords, _attackee: DungeonCoords) {

    }

    pub fn say_damage(&mut self, _victim: DungeonCoords, _amount: u32) {

    }
}

impl CombatEngine for ControlledCombatEngine {

}