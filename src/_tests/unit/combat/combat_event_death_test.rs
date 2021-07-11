use crate::combat::combat_event::CombatEvent;
use crate::combat::combat_event_death::CombatEventDeath;
use crate::game::object_type::ObjectType::{Enemy, Player};

#[test]
fn log_string() {
    let event_1 = CombatEventDeath::new(Player);
    let event_2 = CombatEventDeath::new(Enemy);
    assert_eq!(event_1.log_string(), "Player dies!");
    assert_eq!(event_2.log_string(), "Enemy dies!");
}