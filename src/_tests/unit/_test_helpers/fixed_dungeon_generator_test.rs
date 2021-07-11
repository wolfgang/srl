use crate::_tests::_helpers::fixed_dungeon_generator::FixedDungeonGenerator;
use crate::game::object_type::ObjectType::{Enemy, Wall};
use crate::gen::dungeon_generator::DungeonGenerator;

#[test]
fn generates_given_objects() {
    let mut generator = FixedDungeonGenerator::new();
    generator.generate_walls(vec![(1, 2), (3, 4)]);
    generator.generate_enemies(vec![(5, 6), (7, 8)]);
    generator.generate_player(9, 10);

    let dungeon = generator.generate();
    let objects = dungeon.get_object_types();
    assert_eq!(4, objects.len());
    assert!(objects.contains(&((1, 2), Wall)));
    assert!(objects.contains(&((3, 4), Wall)));
    assert!(objects.contains(&((5, 6), Enemy)));
    assert!(objects.contains(&((7, 8), Enemy)));
    assert_eq!((9, 10), dungeon.get_player_position())
}