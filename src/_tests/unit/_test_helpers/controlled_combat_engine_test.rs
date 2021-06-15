use crate::_tests::_helpers::controlled_combat_engine::ControlledCombatEngine;
use crate::combat::combat_engine::CombatEngine;
use crate::game::dungeon::DungeonCoords;

const ATTACKER: DungeonCoords = (1, 2);
const VICTIM: DungeonCoords = (3, 4);

#[test]
fn initially_does_not_hit_and_does_not_roll_damage() {
    let engine = ControlledCombatEngine::new();
    assert!(!engine.is_hit(ATTACKER, VICTIM));
    assert_eq!(engine.roll_damage(ATTACKER), 0);
}