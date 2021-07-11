use crate::combat::combat_event_trait::CombatEventTrait;
use crate::game::ObjectType;

pub struct CombatEventDeath {
    victim: ObjectType,
}

impl CombatEventDeath {
    pub fn new(victim: ObjectType) -> Self {
        Self { victim }
    }
}

impl CombatEventTrait for CombatEventDeath {
    fn log_string(&self) -> String {
        format!("{} dies!", self.victim)
    }
}