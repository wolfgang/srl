use crate::combat::combat_event_trait::CombatEventTrait;
use crate::game::ObjectType;

pub struct CombatEventMiss {
    attacker: ObjectType,
    victim: ObjectType,
}

impl CombatEventMiss {
    pub fn new(attacker: ObjectType, victim: ObjectType) -> Self {
        Self {attacker, victim}
    }
}

impl CombatEventTrait for CombatEventMiss {
    fn log_string(&self) -> String {
        format!("{} misses {}!", self.attacker, self.victim)
    }
}