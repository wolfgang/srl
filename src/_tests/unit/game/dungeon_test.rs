use crate::game::dungeon::{Dungeon, DungeonObjectTuple};
use crate::game::object_type::ObjectType::{Enemy, Wall};
use crate::input::move_direction::MoveDirection::*;

#[test]
fn initially_has_no_objects() {
    let dungeon = Dungeon::new();
    let empty_vec: Vec<DungeonObjectTuple> = Vec::new();
    assert_eq!(empty_vec, dungeon.get_objects());
}

#[test]
fn can_add_walls_and_enemies() {
    let mut dungeon = Dungeon::new();
    dungeon.add_walls(&vec![(1, 2), (3, 4)]);
    dungeon.add_enemies(&vec![(5, 6), (7, 8)]);
    let objects = dungeon.get_objects();
    assert_eq!(objects.len(), 4);
    assert!(objects.contains(&((1, 2), Wall)));
    assert!(objects.contains(&((3, 4), Wall)));
    assert!(objects.contains(&((5, 6), Enemy)));
    assert!(objects.contains(&((7, 8), Enemy)));
}

#[test]
fn initially_has_zero_player_position() {
    let dungeon = Dungeon::new();
    assert_eq!((0, 0), dungeon.get_player_position());
}

#[test]
fn can_set_player_position() {
    let mut dungeon = Dungeon::new();
    dungeon.set_player_position(1, 2);
    assert_eq!((1, 2), dungeon.get_player_position());
}

#[test]
fn move_player_without_obstacles() {
    let mut dungeon = Dungeon::new();
    dungeon.move_player(Right);
    assert_eq!((1, 0), dungeon.get_player_position());
    dungeon.move_player(Down);
    assert_eq!((1, 1), dungeon.get_player_position());
    dungeon.move_player(Left);
    assert_eq!((0, 1), dungeon.get_player_position());
    dungeon.move_player(Up);
    assert_eq!((0, 0), dungeon.get_player_position());
}


#[test]
fn move_player_right_stopped_by_obstacle() {
    let mut dungeon = Dungeon::new();
    dungeon.add_walls(&vec![(1, 0)]);
    dungeon.add_enemies(&vec![(1, 1)]);
    dungeon.move_player(Right);
    assert_eq!((0, 0), dungeon.get_player_position());
    dungeon.move_player(Down);
    dungeon.move_player(Right);
    assert_eq!((0, 1), dungeon.get_player_position());
}

#[test]
fn move_player_left_stopped_by_obstacle() {
    let mut dungeon = Dungeon::new();
    dungeon.set_player_position(1, 0);
    dungeon.add_walls(&vec![(0, 0)]);
    dungeon.move_player(Left);
    assert_eq!((1, 0), dungeon.get_player_position());
}

#[test]
fn move_player_up_down_stopped_by_obstacle() {
    let mut dungeon = Dungeon::new();
    dungeon.set_player_position(0, 2);
    dungeon.add_walls(&vec![(0, 1)]);
    dungeon.add_enemies(&vec![(0, 3)]);
    dungeon.move_player(Up);
    assert_eq!((0, 2), dungeon.get_player_position());
    dungeon.move_player(Down);
    assert_eq!((0, 2), dungeon.get_player_position());
}

#[test]
fn move_player_returns_no_collision_if_no_collision() {
    let mut dungeon = Dungeon::new();
    dungeon.set_player_position(2, 2);
    assert_eq!(None, dungeon.move_player(Left));
    assert_eq!(None, dungeon.move_player(Right));
    assert_eq!(None, dungeon.move_player(Up));
    assert_eq!(None, dungeon.move_player(Down));
}

#[test]
fn move_player_returns_collisions_with_enemy() {
    let mut dungeon = Dungeon::new();
    dungeon.set_player_position(1, 1);
    dungeon.add_enemies(&vec![(1, 0), (2, 1), (1, 2), (0, 1)]);
    assert_eq!(Some(((0, 1), Enemy)), dungeon.move_player(Left));
    assert_eq!(Some(((2, 1), Enemy)), dungeon.move_player(Right));
    assert_eq!(Some(((1, 0), Enemy)), dungeon.move_player(Up));
    assert_eq!(Some(((1, 2), Enemy)), dungeon.move_player(Down));
}


#[test]
fn move_player_returns_collisions_with_wall() {
    let mut dungeon = Dungeon::new();
    dungeon.set_player_position(1, 1);
    dungeon.add_walls(&vec![(1, 0), (2, 1), (1, 2), (0, 1)]);
    assert_eq!(Some(((0, 1), Wall)), dungeon.move_player(Left));
    assert_eq!(Some(((2, 1), Wall)), dungeon.move_player(Right));
    assert_eq!(Some(((1, 0), Wall)), dungeon.move_player(Up));
    assert_eq!(Some(((1, 2), Wall)), dungeon.move_player(Down));
}