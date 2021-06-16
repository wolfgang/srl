use crate::_tests::_helpers::controlled_combat_engine::ControlledCombatEngine;
use crate::_tests::_helpers::testable_game::TestableGame;
use crate::game::dungeon::DungeonCoords;
use crate::input::move_direction::MoveDirection::*;
use crate::input::move_direction::MoveDirection;

const PLAYER: DungeonCoords = (1, 1);

const ENEMY_RIGHT: DungeonCoords = (2, 1);
const ENEMY_LEFT: DungeonCoords = (0, 1);
const ENEMY_ABOVE: DungeonCoords = (1, 0);
const ENEMY_BELOW: DungeonCoords = (1, 2);

const DUNGEON: [&str; 3] = [
    ". E .",
    "E @ E",
    ". E ."
];

#[test]
fn combat_with_enemy_to_the_right() {
    verify_combat_scenarios_when_moving(Right, ENEMY_RIGHT);
}

#[test]
fn combat_with_enemy_to_the_left() {
    verify_combat_scenarios_when_moving(Left, ENEMY_LEFT);
}

#[test]
fn combat_with_enemy_above() {
    verify_combat_scenarios_when_moving(Up, ENEMY_ABOVE);
}

#[test]
fn combat_with_enemy_below() {
    verify_combat_scenarios_when_moving(Down, ENEMY_BELOW);
}


#[test]
fn collision_with_non_enemy_does_nothing() {
    let mut game = TestableGame::from_strings(vec![
        ". # .",
        "# @ #",
        ". # ."
    ]);

    for direction in MoveDirection::iter() {
        game.input.simulate_move(*direction);
        game.tick();
        game.render();
        game.renderer.assert_combat_log(vec![]);
    }

}

fn verify_combat_scenarios_when_moving(direction: MoveDirection, enemy: DungeonCoords) {
    verify_next_combat_log(
        direction,
        |combat_engine| {
            combat_engine.say_is_hit(PLAYER, enemy);
            combat_engine.say_is_hit(enemy, PLAYER);
            combat_engine.say_damage(PLAYER, 6);
            combat_engine.say_damage(enemy, 2);
        },
        vec![
            "Player hits Enemy for 6 damage!",
            "Enemy hits Player for 2 damage!"
        ]);

    verify_next_combat_log(
        direction, |_| {},
        vec![
            "Player misses Enemy!",
            "Enemy misses Player!"
        ]);

    verify_next_combat_log(
        direction,
        |combat_engine| {
            combat_engine.say_is_hit(PLAYER, enemy);
            combat_engine.say_damage(PLAYER, 6);
        },
        vec![
            "Player hits Enemy for 6 damage!",
            "Enemy misses Player!"
        ]);

    verify_next_combat_log(
        direction,
        |combat_engine| {
            combat_engine.say_is_hit(enemy, PLAYER);
            combat_engine.say_damage(enemy, 5);
        },
        vec![
            "Player misses Enemy!",
            "Enemy hits Player for 5 damage!"
        ]);
}

fn verify_next_combat_log<F: Fn(&mut ControlledCombatEngine)>(move_direction: MoveDirection, combat_setup_fn: F, expected: Vec<&str>) {
    let mut combat_engine = ControlledCombatEngine::new();
    combat_setup_fn(&mut combat_engine);
    let mut game = TestableGame::from_strings(Vec::from(DUNGEON));
    game.game.override_combat_engine(combat_engine);

    game.input.simulate_move(move_direction);
    game.tick();
    game.render();
    game.renderer.assert_combat_log(expected)
}