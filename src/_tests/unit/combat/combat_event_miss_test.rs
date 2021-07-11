use crate::combat::combat_event_miss::CombatEventMiss;
use crate::combat::combat_event_trait::CombatEventTrait;
use crate::game::object_type::ObjectType::{Enemy, Player};

#[test]
fn log_string() {
    let event_1 = CombatEventMiss::new(Player, Enemy);
    let event_2 = CombatEventMiss::new(Enemy, Player);
    assert_eq!(event_1.log_string(), "Player misses Enemy!");
    assert_eq!(event_2.log_string(), "Enemy misses Player!");
}