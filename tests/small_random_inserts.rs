extern crate rand;
extern crate ropey;

use rand::Rng;
use ropey::Rope;

#[test]
#[cfg_attr(miri, ignore)]
fn small_random_inserts() {
    let mut rng = rand::thread_rng();
    let mut tree = Rope::new();

    // Do a bunch of random incoherent inserts
    for _ in 0..(1 << 10) {
        let len = tree.len_chars().max(1);
        tree.insert(rng.r#gen::<usize>() % len, "Hello ");
        tree.insert(rng.r#gen::<usize>() % len, "world! ");
        tree.insert(rng.r#gen::<usize>() % len, "How are ");
        tree.insert(rng.r#gen::<usize>() % len, "you ");
        tree.insert(rng.r#gen::<usize>() % len, "doing?\r\n");
        tree.insert(rng.r#gen::<usize>() % len, "Let's ");
        tree.insert(rng.r#gen::<usize>() % len, "keep ");
        tree.insert(rng.r#gen::<usize>() % len, "inserting ");
        tree.insert(rng.r#gen::<usize>() % len, "more ");
        tree.insert(rng.r#gen::<usize>() % len, "items.\r\n");
        tree.insert(rng.r#gen::<usize>() % len, "こんいちは、");
        tree.insert(rng.r#gen::<usize>() % len, "みんなさん！");
    }

    // Make sure the tree is sound
    tree.assert_integrity();
    tree.assert_invariants();
}
