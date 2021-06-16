use crate::combat::combat_engine::CombatEngine;

pub struct FixedCombatEngine {

}

impl CombatEngine for FixedCombatEngine {
    fn is_hit(&self, _attacker: (u32, u32), _victim: (u32, u32)) -> bool {
        true
    }

    fn roll_damage(&self, _attacker: (u32, u32)) -> u32 {
        17
    }
}