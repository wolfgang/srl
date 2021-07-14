use std::cell::RefCell;
use std::rc::Rc;

use crate::game::creature::Creature;
use crate::game::dungeon::{Dungeon, DungeonCoords, DungeonRef};
use crate::game::object_type::ObjectType::{Enemy, Wall};
use crate::game::ObjectType;
use crate::game::player_controller::PlayerController;
use crate::input::move_direction::MoveDirection::*;

const PLAYER: DungeonCoords = (1, 1);
const LEFT_FROM_PLAYER: DungeonCoords = (0, 1);
const RIGHT_FROM_PLAYER: DungeonCoords = (2, 1);
const ABOVE_PLAYER: DungeonCoords = (1, 0);
const BELOW_PLAYER: DungeonCoords = (1, 2);

#[test]
fn move_player_with_nothing_in_the_way() {
    let dungeon = Rc::new(RefCell::new(Dungeon::new()));
    dungeon.borrow_mut().set_player_position(2, 2);

    let mut controller = PlayerController::new(dungeon.clone());

    assert_eq!(None, controller.move_player(Left));
    assert_eq!(dungeon.borrow().get_player_position(), (1, 2));
    assert_eq!(None, controller.move_player(Right));
    assert_eq!(dungeon.borrow().get_player_position(), (2, 2));
    assert_eq!(None, controller.move_player(Up));
    assert_eq!(dungeon.borrow().get_player_position(), (2, 1));
    assert_eq!(None, controller.move_player(Down));
    assert_eq!(dungeon.borrow().get_player_position(), (2, 2));

}

#[test]
fn move_player_surrounded_by_enemies() {
    let dungeon = Rc::new(RefCell::new(Dungeon::new()));
    add_enemies(&dungeon, &vec![
        LEFT_FROM_PLAYER,
        RIGHT_FROM_PLAYER,
        ABOVE_PLAYER,
        BELOW_PLAYER]);

    verify_collision_surrounded_by(Enemy, &dungeon);
}

#[test]
fn move_player_surrounded_by_walls() {
    let dungeon = Rc::new(RefCell::new(Dungeon::new()));
    dungeon.borrow_mut().add_walls(&vec![
        LEFT_FROM_PLAYER,
        RIGHT_FROM_PLAYER,
        ABOVE_PLAYER,
        BELOW_PLAYER]);

    verify_collision_surrounded_by(Wall, &dungeon);
}


fn add_enemies(dungeon: &DungeonRef, enemies: &Vec<DungeonCoords>) {
    for coords in enemies {
        dungeon.borrow_mut().add_enemy(*coords, Creature { hp: 100 });
    }
}

fn verify_collision_surrounded_by(object_type: ObjectType, dungeon: &DungeonRef) {
    let mut controller = PlayerController::new(dungeon.clone());

    dungeon.borrow_mut().set_player_position(PLAYER.0, PLAYER.1);

    assert_eq!(Some((LEFT_FROM_PLAYER, object_type)), controller.move_player(Left));
    assert_eq!(dungeon.borrow().get_player_position(), PLAYER);

    assert_eq!(Some((RIGHT_FROM_PLAYER, object_type)), controller.move_player(Right));
    assert_eq!(dungeon.borrow().get_player_position(), PLAYER);

    assert_eq!(Some((ABOVE_PLAYER, object_type)), controller.move_player(Up));
    assert_eq!(dungeon.borrow().get_player_position(), PLAYER);

    assert_eq!(Some((BELOW_PLAYER, object_type)), controller.move_player(Down));
    assert_eq!(dungeon.borrow().get_player_position(), PLAYER);

}