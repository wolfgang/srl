use crate::game::ObjectType;

pub struct CombatEvent {
    attacker: ObjectType,
    victim: ObjectType,
    is_death: bool,
    damage: u32,
}

impl CombatEvent {
    pub fn miss(attacker: ObjectType, victim: ObjectType) -> Self {
        Self::hit(attacker, victim, 0)
    }

    pub fn hit(attacker: ObjectType, victim: ObjectType, damage: u32) -> Self {
        Self { attacker, victim, damage, is_death: false }
    }

    pub fn death(victim: ObjectType) -> Self {
        Self { attacker: victim, victim, damage: 0, is_death: true }
    }


    pub fn log_string(&self) -> String {
        if self.is_death {
            format!("{} dies!", self.attacker)
        } else {
            if self.damage > 0 {
                format!("{} hits {} for {} damage!", self.attacker, self.victim, self.damage)
            } else {
                format!("{} misses {}!", self.attacker, self.victim)
            }
        }
    }
}