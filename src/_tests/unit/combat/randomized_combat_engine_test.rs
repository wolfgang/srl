use crate::combat::combat_engine::CombatEngine;
use crate::combat::randomized_combat_engine::RandomizedCombatEngine;

#[test]
fn is_hit_produces_some_hits_and_some_misses() {
    let mut results: Vec<bool> = Vec::new();
    let combat_engine = RandomizedCombatEngine::new();
    for _ in 0 .. 50 {
        results.push(combat_engine.is_hit((0, 0), (0, 0)));
    }

    let num_hits = results.iter().filter(|r| **r == true).count();
    let num_misses =  results.iter().filter(|r| **r == false).count();
    assert!(num_hits > 0);
    assert!(num_misses > 0);
 }

#[test]
fn roll_damage_produces_values_in_some_interval() {
    let combat_engine = RandomizedCombatEngine::new();
    for _ in 0 .. 50 {
        let damage = combat_engine.roll_damage((0, 0));
        assert!(damage >= 1);
        assert!(damage <= 10);
    }

}