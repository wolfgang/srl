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

#[test]
fn set_hp_for_all_enemies_causes_all_enemies_to_get_this_hp() {
    let mut generator = FixedDungeonGenerator::new();
    generator.generate_enemies(vec![(1, 2), (3, 4)]);
    generator.set_hp_for_all_enemies(20);
    let mut dungeon = generator.generate();
    assert_eq!(dungeon.damage_enemy((1, 2), 5), 15);
    assert_eq!(dungeon.damage_enemy((3, 4), 10), 10);
}