use crate::_tests::_helpers::controlled_combat_engine::ControlledCombatEngine;
use crate::combat::combat_engine::CombatEngine;
use crate::game::dungeon::DungeonCoords;

const ATTACKER_1: DungeonCoords = (1, 2);
const VICTIM_1: DungeonCoords = (3, 4);
const ATTACKER_2: DungeonCoords = (5, 6);
const VICTIM_2: DungeonCoords = (7, 8);

#[test]
fn initially_does_not_hit_and_does_not_roll_damage() {
    let engine = ControlledCombatEngine::new();
    assert!(!engine.is_hit(ATTACKER_1, VICTIM_1));
    assert!(!engine.is_hit(ATTACKER_2, VICTIM_2));
    assert_eq!(engine.roll_damage(ATTACKER_1), 0);
    assert_eq!(engine.roll_damage(ATTACKER_2), 0);
}

#[test]
fn say_is_hit_causes_hit_for_given_attacker() {
    let mut engine = ControlledCombatEngine::new();

    engine.say_is_hit(ATTACKER_1, VICTIM_1);
    assert!(engine.is_hit(ATTACKER_1, VICTIM_1));
    assert!(!engine.is_hit(ATTACKER_2, VICTIM_2));

    engine.say_is_hit(ATTACKER_2, VICTIM_2);
    assert!(engine.is_hit(ATTACKER_1, VICTIM_1));
    assert!(engine.is_hit(ATTACKER_2, VICTIM_2));
}

#[test]
fn say_damage_determines_damage_for_given_attacker() {
    let mut engine = ControlledCombatEngine::new();

    engine.say_damage(ATTACKER_1, 1111);
    assert_eq!(engine.roll_damage(ATTACKER_1), 1111);
    assert_eq!(engine.roll_damage(ATTACKER_2), 0);

    engine.say_damage(ATTACKER_2, 2222);
    assert_eq!(engine.roll_damage(ATTACKER_1), 1111);
    assert_eq!(engine.roll_damage(ATTACKER_2), 2222);

}