use crate::game::dungeon::Dungeon;

#[test]
fn initially_has_no_walls_or_enemies() {
    let dungeon = Dungeon::new();
    let empty_vec: Vec<(u32, u32)> = Vec::new();
    assert_eq!(&empty_vec, dungeon.get_walls());
    assert_eq!(&empty_vec, dungeon.get_enemies());
}

#[test]
fn can_add_walls() {
    let mut dungeon = Dungeon::new();
    dungeon.add_walls(&vec![(1, 2), (3, 4)]);
    assert_eq!(&vec![(1, 2), (3, 4)], dungeon.get_walls());

}

#[test]
fn can_add_enemies() {
    let mut dungeon = Dungeon::new();
    dungeon.add_enemies(&vec![(1, 2), (3, 4)]);
    assert_eq!(&vec![(1, 2), (3, 4)], dungeon.get_enemies());

}