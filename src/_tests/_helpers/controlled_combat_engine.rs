use crate::combat::combat_engine::CombatEngine;
use crate::game::dungeon::DungeonCoords;

pub struct ControlledCombatEngine {}

impl ControlledCombatEngine {
    pub fn new() -> Self {
        Self {}
    }

    pub fn say_is_hit(&mut self, _attacker: DungeonCoords, _attackee: DungeonCoords) {}

    pub fn say_damage(&mut self, _attacker: DungeonCoords, _amount: u32) {}
}

impl CombatEngine for ControlledCombatEngine {
    fn is_hit(&self, _attacker: (u32, u32), _victim: (u32, u32)) -> bool {
        false
    }

    fn roll_damage(&self, _attacker: (u32, u32)) -> u32 {
        0
    }
}