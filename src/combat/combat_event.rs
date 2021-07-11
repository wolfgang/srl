pub trait CombatEvent {
    fn log_string(&self) -> String;
}