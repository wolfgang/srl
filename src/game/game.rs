use crate::combat::combat_engine::{CombatEngine, NullCombatEngine};
use crate::game::dungeon::Dungeon;
use crate::game::object_type::ObjectType::{Enemy, Player};
use crate::game::ObjectType;
use crate::gen::dungeon_generator::DungeonGenerator;
use crate::gfx::renderer::Renderer;
use crate::input::Input;

struct CombatEvent {
    attacker: ObjectType,
    victim: ObjectType,
    damage: u32,
}

impl CombatEvent {
    pub fn text(&self) -> String {
        let attacker = if self.attacker == Player { "Player" } else { "Enemy" };
        let victim = if self.victim == Player { "Player" } else { "Enemy" };
        if self.damage > 0 {
        format!("{} hits {} for {} damage!", attacker, victim, self.damage)
        } else {
            format!("{} misses {}!", attacker, victim)
        }

    }
}

pub struct Game {
    dungeon: Dungeon,
    combat_engine: Box<dyn CombatEngine>,
    combat_events: Vec<CombatEvent>,
}

impl Game {
    pub fn generate_with<T: DungeonGenerator>(generator: &T) -> Self {
        Self {
            combat_engine: Box::from(NullCombatEngine {}),
            dungeon: generator.generate(),
            combat_events: Vec::new(),
        }
    }

    pub fn override_combat_engine<T: 'static + CombatEngine>(&mut self, engine: T) {
        self.combat_engine = Box::from(engine);
    }

    pub fn add_enemies(&mut self, positions: &Vec<(u32, u32)>) {
        self.dungeon.add_enemies(positions);
    }

    pub fn add_walls(&mut self, positions: &Vec<(u32, u32)>) {
        self.dungeon.add_walls(positions);
    }

    pub fn set_player_position(&mut self, x: u32, y: u32) {
        self.dungeon.set_player_position(x, y);
    }

    pub fn tick<T: Input>(&mut self, input: &T) {
        if input.move_left() { self.dungeon.move_player_left(); }

        if input.move_right() {
            let result = self.dungeon.move_player_right();

            match result {
                Some((coords, _object_type)) => {
                    if self.combat_engine.is_hit(self.dungeon.get_player_position(), coords) {
                        let player_damage = self.combat_engine.roll_damage(self.dungeon.get_player_position());
                        let enemy_damage = self.combat_engine.roll_damage(coords);
                        self.combat_events.push(CombatEvent {
                            attacker: Player,
                            victim: Enemy,
                            damage: player_damage,
                        });

                        self.combat_events.push(CombatEvent {
                            attacker: Enemy,
                            victim: Player,
                            damage: enemy_damage,
                        });
                    } else {
                        self.combat_events.push(CombatEvent {
                            attacker: Player,
                            victim: Enemy,
                            damage: 0,
                        });

                        self.combat_events.push(CombatEvent {
                            attacker: Enemy,
                            victim: Player,
                            damage: 0,
                        });

                    }
                }
                None => {}
            }
        }

        if input.move_up() { self.dungeon.move_player_up(); }

        if input.move_down() { self.dungeon.move_player_down(); }
    }


    pub fn render<T: Renderer>(&self, renderer: &mut T) {
        renderer.clear();
        for ((x, y), object_type) in self.dungeon.get_objects() {
            renderer.render_at(*x, *y, *object_type);
        }

        let (player_x, player_y) = self.dungeon.get_player_position();
        renderer.render_at(player_x, player_y, Player);

        for evt in self.combat_events.iter() {
            renderer.append_combat_log(evt.text().as_str())
        }
    }
}