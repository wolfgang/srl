use rand::{Rng, thread_rng};

use crate::combat::combat_engine::CombatEngine;
use crate::game::dungeon::DungeonCoords;

pub struct RandomizedCombatEngine {

}

impl RandomizedCombatEngine {
    pub fn new() -> Self {
        Self {}
    }
}

impl CombatEngine for RandomizedCombatEngine {
    fn is_hit(&self, _attacker: DungeonCoords, _victim: DungeonCoords) -> bool {
        return thread_rng().gen_bool(0.5)
    }

    fn roll_damage(&self, _attacker: DungeonCoords) -> u32 {
        thread_rng().gen_range(1..=10)
    }
}