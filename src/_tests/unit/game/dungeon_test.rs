use crate::game::dungeon::Dungeon;

#[test]
fn initially_has_no_walls_or_enemies() {
    let dungeon = Dungeon::new();
    let empty_vec: Vec<(u32, u32)> = Vec::new();
    assert_eq!(&empty_vec, dungeon.get_walls());
    assert_eq!(&empty_vec, dungeon.get_enemies());
}