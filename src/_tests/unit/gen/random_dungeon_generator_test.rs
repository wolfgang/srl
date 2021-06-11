use std::collections::HashSet;

use crate::game::dungeon::{CellCoords, Dungeon, DungeonObject};
use crate::game::object_type::ObjectType::{Enemy, Wall};
use crate::game::ObjectType;
use crate::gen::dungeon_generator::DungeonGenerator;
use crate::gen::random_dungeon_generator::RandomDungeonGenerator;

#[test]
fn generates_n_distinct_walls_and_enemies() {
    let generator = RandomDungeonGenerator::new(3, 2);
    let dungeon = generator.generate();
    let mut walls = get_distinct_objects_of_type(Wall, &dungeon);
    let mut enemies = get_distinct_objects_of_type(Enemy, &dungeon);
    assert_eq!(walls.len(), 3);
    assert_eq!(enemies.len(), 2);

    let mut all_coords: HashSet<CellCoords> = vec![walls, enemies]
        .iter()
        .flat_map(|v| { v.iter() })
        .map(|(pos, _)| { *pos })
        .collect();

    assert_eq!(all_coords.len(), 5);
}

fn get_distinct_objects_of_type(object_type: ObjectType, dungeon: &Dungeon) -> HashSet<DungeonObject> {
    dungeon.get_objects()
        .iter()
        .filter_map(|(pos, ty)| {
            if *ty == object_type { Some((*pos, *ty)) } else { None }
        })
        .collect()
}