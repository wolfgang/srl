use std::collections::HashSet;

use crate::game::dungeon::{Dungeon, DungeonCoords, DungeonObjectTuple};
use crate::game::object_type::ObjectType::{Enemy, Wall};
use crate::game::ObjectType;
use crate::gen::dungeon_generator::DungeonGenerator;
use crate::gen::random_dungeon_generator::RandomDungeonGenerator;

#[test]
fn generates_distinct_walls_and_enemies() {
    let generator = RandomDungeonGenerator::new(10, 20, 3, 2);
    let dungeon = generator.generate();
    let walls = get_distinct_objects_of_type(Wall, &dungeon);
    let enemies = get_distinct_objects_of_type(Enemy, &dungeon);
    let all_coords = get_distinct_coords(&dungeon);
    assert_eq!(walls.len(), 3);
    assert_eq!(enemies.len(), 2);
    assert_eq!(all_coords.len(), walls.len() + enemies.len());
}

#[test]
fn generates_different_dungeons() {
    let generator = RandomDungeonGenerator::new(10, 20, 3, 2);
    let dungeon1 = generator.generate();
    let dungeon2 = generator.generate();
    assert_ne!(get_distinct_objects_of_type(Wall, &dungeon1), get_distinct_objects_of_type(Wall, &dungeon2));
    assert_ne!(get_distinct_objects_of_type(Enemy, &dungeon1), get_distinct_objects_of_type(Enemy, &dungeon2));
    assert_ne!(dungeon1.get_player_position(), dungeon2.get_player_position());
}

#[test]
fn generates_player_position_distinct_from_objects() {
    let generator = RandomDungeonGenerator::new(10, 20, 3, 2);
    let dungeon = generator.generate();
    let all_coords = get_distinct_coords(&dungeon);
    let player_position = dungeon.get_player_position();
    assert!(!all_coords.contains(&player_position))

}

fn get_distinct_objects_of_type(object_type: ObjectType, dungeon: &Dungeon) -> HashSet<DungeonObjectTuple> {
    dungeon.get_object_types()
        .iter()
        .filter_map(|(pos, ty)| {
            if *ty == object_type { Some((*pos, *ty)) } else { None }
        })
        .collect()
}

fn get_distinct_coords(dungeon: &Dungeon) -> HashSet<DungeonCoords> {
    dungeon.get_object_types()
        .iter()
        .map(|(pos, _)| { *pos })
        .collect()
}