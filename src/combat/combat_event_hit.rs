use crate::combat::combat_event_trait::CombatEventTrait;
use crate::game::ObjectType;

pub struct CombatEventHit {
    attacker: ObjectType,
    victim: ObjectType,
    damage: u32,
}

impl CombatEventHit {
    pub fn new(attacker: ObjectType, victim: ObjectType, damage: u32) -> Self {
        Self { attacker, victim, damage }
    }
}

impl CombatEventTrait for CombatEventHit {
    fn log_string(&self) -> String {
        format!("{} hits {} for {} damage!", self.attacker, self.victim, self.damage)
    }
}