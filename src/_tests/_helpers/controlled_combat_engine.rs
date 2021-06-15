use std::collections::HashMap;

use crate::combat::combat_engine::CombatEngine;
use crate::game::dungeon::DungeonCoords;

pub struct ControlledCombatEngine {
    hits: HashMap<DungeonCoords, DungeonCoords>,
    damages: HashMap<DungeonCoords, u32>
}

impl ControlledCombatEngine {
    pub fn new() -> Self {
        Self {
            hits: HashMap::new(),
            damages: HashMap::new()
        }
    }

    pub fn say_is_hit(&mut self, attacker: DungeonCoords, victim: DungeonCoords) {
        self.hits.insert(attacker, victim);
    }

    pub fn say_damage(&mut self, attacker: DungeonCoords, amount: u32) {
        self.damages.insert(attacker, amount);
    }
}

impl CombatEngine for ControlledCombatEngine {
    fn is_hit(&self, attacker: (u32, u32), victim: (u32, u32)) -> bool {
        if !self.hits.contains_key(&attacker) { return false; }
        self.hits.get(&attacker).unwrap() == &victim
    }

    fn roll_damage(&self, attacker: (u32, u32)) -> u32 {
        *self.damages.get(&attacker).unwrap_or(&0)
    }
}