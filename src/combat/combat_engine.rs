use crate::game::dungeon::DungeonCoords;

pub trait CombatEngine {
    fn is_hit(&self, attacker: DungeonCoords, victim: DungeonCoords) -> bool;
    fn roll_damage(&self, attacker: DungeonCoords) -> u32;
    fn get_hp(&self, _coords: DungeonCoords) -> u32 { 100 }
}

pub struct NullCombatEngine {}

impl CombatEngine for NullCombatEngine {
    fn is_hit(&self, _attacker: (u32, u32), _victim: (u32, u32)) -> bool { false }
    fn roll_damage(&self, _attacker: (u32, u32)) -> u32 { 0 }
}