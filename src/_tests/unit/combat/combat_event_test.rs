use crate::combat::combat_event::CombatEvent;
use crate::game::object_type::ObjectType::{Enemy, Player};

#[test]
fn log_string_for_hit() {
    let event_1 = CombatEvent::hit(Player, Enemy, 1234);
    let event_2 = CombatEvent::hit(Enemy, Player, 5678);
    assert_eq!(event_1.log_string(), "Player hits Enemy for 1234 damage!");
    assert_eq!(event_2.log_string(), "Enemy hits Player for 5678 damage!");
}

#[test]
fn log_string_for_miss() {
    let event_1 = CombatEvent::hit(Player, Enemy, 0);
    let event_2 = CombatEvent::hit(Enemy, Player, 0);
    assert_eq!(event_1.log_string(), "Player misses Enemy!");
    assert_eq!(event_2.log_string(), "Enemy misses Player!");
}
