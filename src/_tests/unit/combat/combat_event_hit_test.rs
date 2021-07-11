use crate::combat::combat_event_hit::CombatEventHit;
use crate::combat::combat_event_trait::CombatEventTrait;
use crate::game::object_type::ObjectType::{Enemy, Player};

#[test]
fn combat_evt_hit_as_string() {
    let event_1 = CombatEventHit::new(Player, Enemy, 1234);
    let event_2 = CombatEventHit::new(Enemy, Player, 5678);
    assert_eq!(event_1.log_string(), "Player hits Enemy for 1234 damage!");
    assert_eq!(event_2.log_string(), "Enemy hits Player for 5678 damage!");
}