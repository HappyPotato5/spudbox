
use spudbox::arena::*;

const RESULTS: [i32; 5] = [-12, 11112, -13, 422, -12];

#[test]
fn arena_alloc_free(){
    let mut arena: Arena<i32> = Arena::new();
    let mut stack = Vec::new();

    stack.push(arena.alloc(-12));
    stack.push(arena.alloc(11112));
    stack.push(arena.alloc(-13));

    stack.push(arena.alloc(422));
    stack.push(arena.alloc(-12));

    let mut j = 0usize;

    assert_eq!(arena.len(), 5);

    for i in stack {
        if j == 3 {
            arena.free(i);
        } else {
            assert_eq!(arena[i], RESULTS[j]);
        }
        j += 1;
    }

    assert_eq!(arena.len(), 4);

    arena.alloc(-13);

    assert_eq!(arena.len(), 5);
}