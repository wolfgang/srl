use crate::_tests::_helpers::testable_game::TestableGame;
use crate::game::dungeon::DungeonCoords;
use crate::input::move_direction::MoveDirection::Right;

const PLAYER: DungeonCoords = (1, 0);
const ENEMY: DungeonCoords = (2, 0);

#[test]
fn enemy_is_removed_after_two_hits() {
    let mut game = TestableGame::from_strings(vec![". @ E ."]);
    player_hits_for(60, &mut game);

    game.input.simulate_move(Right);
    game.verify_next_tiles(vec![". @ E ."]);
    game.verify_next_tiles(vec![". . @ ."]);
}

#[test]
fn combat_log_reflects_enemy_death() {
    let mut game = TestableGame::from_strings(vec![". @ E ."]);
    player_hits_for(1000, &mut game);

    game.input.simulate_move(Right);
    game.verify_next_combat_log(vec![
        "Player hits Enemy for 1000 damage!",
        "Enemy dies!"
    ])
}

#[test]
fn game_over_after_player_death() {
    let mut game = TestableGame::from_strings(vec![". @ E ."]);
    enemy_hits_for(60, &mut game);
    game.input.simulate_move(Right);
    game.tick();
    assert!(!game.game.player_died());
    game.tick();
    assert!(game.game.player_died());
}

#[test]
fn combat_log_reflects_player_death() {
    let mut game = TestableGame::from_strings(vec![". @ E ."]);
    enemy_hits_for(1000, &mut game);

    game.input.simulate_move(Right);
    game.verify_next_combat_log(vec![
        "Player misses Enemy!",
        "Enemy hits Player for 1000 damage!",
        "Player dies!"
    ])
}


fn player_hits_for(damage: u32, game: &mut TestableGame) {
    game.configure_combat(|combat_engine| {
        combat_engine.say_is_hit(PLAYER, ENEMY);
        combat_engine.say_damage(PLAYER, damage);
    });
}

fn enemy_hits_for(damage: u32, game: &mut TestableGame) {
    game.configure_combat(|combat_engine| {
        combat_engine.say_is_hit(ENEMY, PLAYER);
        combat_engine.say_damage(ENEMY, damage);
    });
}


