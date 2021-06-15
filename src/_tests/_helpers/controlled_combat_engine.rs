use std::collections::HashMap;

use crate::combat::combat_engine::CombatEngine;
use crate::game::dungeon::DungeonCoords;

pub struct ControlledCombatEngine {
    hits: HashMap<DungeonCoords, DungeonCoords>,
}

impl ControlledCombatEngine {
    pub fn new() -> Self {
        Self {
            hits: HashMap::new()
        }
    }

    pub fn say_is_hit(&mut self, attacker: DungeonCoords, victim: DungeonCoords) {
        self.hits.insert(attacker, victim);
    }

    pub fn say_damage(&mut self, _attacker: DungeonCoords, _amount: u32) {}
}

impl CombatEngine for ControlledCombatEngine {
    fn is_hit(&self, attacker: (u32, u32), victim: (u32, u32)) -> bool {
        if !self.hits.contains_key(&attacker) { return false; }
        self.hits.get(&attacker).unwrap() == &victim
    }

    fn roll_damage(&self, _attacker: (u32, u32)) -> u32 {
        0
    }
}