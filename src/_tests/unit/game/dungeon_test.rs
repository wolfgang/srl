use crate::game::creature::Creature;
use crate::game::dungeon::{Dungeon, DungeonCoords, DungeonObjectTuple};
use crate::game::object_type::ObjectType::{Enemy, Wall};
use crate::input::move_direction::MoveDirection::*;

#[test]
fn initially_has_no_objects() {
    let dungeon = Dungeon::new();
    let empty_vec: Vec<DungeonObjectTuple> = Vec::new();
    assert_eq!(empty_vec, dungeon.get_object_types());
}

#[test]
fn add_enemy_adds_given_creature_as_enemy() {
    let mut dungeon = Dungeon::new();
    dungeon.add_enemy((1, 2), Creature { hp: 15 });
    assert_eq!(dungeon.damage_enemy((1, 2), 7), 8);
}

#[test]
fn can_add_walls_and_enemies() {
    let mut dungeon = Dungeon::new();
    dungeon.add_walls(&vec![(1, 2), (3, 4)]);
    add_enemies(&mut dungeon, &vec![(5, 6), (7, 8)]);
    let objects = dungeon.get_object_types();
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
    add_enemies(&mut dungeon, &vec![(1, 1)]);
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
    add_enemies(&mut dungeon, &vec![(0, 3)]);
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
    add_enemies(&mut dungeon, &vec![(1, 0), (2, 1), (1, 2), (0, 1)]);
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

#[test]
fn remove_enemy_removes_enemy() {
    let mut dungeon = Dungeon::new();
    add_enemies(&mut dungeon, &vec![(1, 2), (3, 4)]);
    dungeon.add_walls(&vec![(5, 6)]);

    let objects = dungeon.get_object_types();
    assert_eq!(objects.len(), 3);

    dungeon.remove_enemy((1, 2));
    let objects = dungeon.get_object_types();
    assert_eq!(objects.len(), 2);
    assert!(objects.contains(&((3, 4), Enemy)));
    assert!(objects.contains(&((5, 6), Wall)));


    dungeon.remove_enemy((3, 4));
    let objects = dungeon.get_object_types();
    assert_eq!(objects.len(), 1);
    assert!(objects.contains(&((5, 6), Wall)));
}

#[test]
fn apply_damage_reduces_hp_and_returns_remaining() {
    let mut dungeon = Dungeon::new();
    add_enemies(&mut dungeon, &vec![(1, 2), (3, 4)]);
    assert_eq!(60, dungeon.damage_enemy((1, 2), 40));
    assert_eq!(-10, dungeon.damage_enemy((1, 2), 70));
}

fn add_enemies(dungeon: &mut Dungeon, enemies: &Vec<DungeonCoords>) {
    for coords in enemies {
        dungeon.add_enemy(*coords, Creature { hp: 100 });
    }

}


// #[test]
// #[should_panic(expected="No enemy")]
// fn apply_damage_should_panic_if_no_enemy_at_coords() {
//     let mut dungeon = Dungeon::new();
//     dungeon.apply_damage((1, 2), 40);
// }
