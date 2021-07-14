use crate::game::creature::Creature;
use crate::game::dungeon::{Dungeon, DungeonCoords, DungeonObjectTuple};
use crate::game::object_type::ObjectType::{Enemy, Wall};

#[test]
fn initially_has_no_objects() {
    let dungeon = Dungeon::new();
    let empty_vec: Vec<DungeonObjectTuple> = Vec::new();
    assert_eq!(empty_vec, dungeon.get_object_types());
}

#[test]
fn add_enemy_adds_given_creature_as_enemy() {
    let mut dungeon = Dungeon::new();
    const ENEMY_COORDS: DungeonCoords = (1, 2);
    dungeon.add_enemy(ENEMY_COORDS, Creature { hp: 15 });
    assert_eq!(dungeon.damage_enemy(ENEMY_COORDS, 7), 8);
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

#[test]
fn get_player_hp_returns_current_player_hp() {
    let mut dungeon = Dungeon::new();
    assert_eq!(dungeon.get_player_hp(), 100);
    dungeon.damage_player(20);
    assert_eq!(dungeon.get_player_hp(), 80);
}

#[test]
fn get_num_enemies_returns_number_of_enemies() {
    let mut dungeon = Dungeon::new();
    assert_eq!(dungeon.get_num_enemies(), 0);
    add_enemies(&mut dungeon, &vec![(1, 2), (3, 4)]);
    assert_eq!(dungeon.get_num_enemies(), 2);

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
