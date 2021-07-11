use crate::combat::combat_event::CombatEvent;
use crate::game::ObjectType;

pub struct CombatEventDeath {
    victim: ObjectType,
}

impl CombatEventDeath {
    pub fn new(victim: ObjectType) -> Self {
        Self { victim }
    }
}

impl CombatEvent for CombatEventDeath {
    fn log_string(&self) -> String {
        format!("{} dies!", self.victim)
    }
}