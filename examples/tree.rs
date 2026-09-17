use spudbox::arena::*;

#[derive(Clone, Debug)]
enum Node {
    Branch{n: u32, left: Option<Index>, right: Option<Index>},
    Leaf(u32)
}

#[derive(Debug)]
struct BinarySearchTree {
    arena: Arena<Node>,
    root: Option<Index>
}

impl BinarySearchTree {
    fn new() -> BinarySearchTree {
        BinarySearchTree { arena: Arena::new(), root: None }
    }

    fn add(&mut self, data: u32) {
        if self.root.is_none() {
            self.root = Some(self.arena.alloc(Node::Leaf(data)));
            return
        }

        let mut actual = self.root;

        let leaf = Some(self.arena.alloc(Node::Leaf(data)));

        while let Some(act) = actual {
            let node = &mut self.arena[act];

            match node {
                Node::Branch { n, left, right } => {
                    if *n == data {
                        return
                    } else if data < *n {
                        if left.is_none() {
                            *left = leaf; break;
                        }
                        actual = *left;
                    } else {
                        if right.is_none() {
                            *right = leaf; break;
                        }
                        actual = *right;
                    }
                },
                Node::Leaf(n) => {
                    if *n == data { return }

                    let branch = if data < *n {
                        Node::Branch { n: *n, left: leaf, right: None }
                    } else {
                        Node::Branch { n: *n, left: None, right: leaf }
                    };

                    self.arena[act] = branch;
                    return
                },
            }
        }
    }

    fn search(&self, num: u32) -> bool {
        let mut actual = self.root;

        while let Some(act) = actual {
            match &self.arena[act] {
                Node::Branch { n, left, right } => {
                    if *n == num {
                        return true
                    } else if num < *n {
                        actual = *left;
                    } else {
                        actual = *right;
                    }
                },
                Node::Leaf(n) => {
                    return *n == num
                },
            }
        }
        false
    }
}

pub fn main() {
    let mut tree = BinarySearchTree::new();

    tree.add(123);
    tree.add(1);
    tree.add(222);
    tree.add(2);
    tree.add(34234);
    tree.add(23);
    tree.add(13);
    tree.add(5);
    tree.add(25);

    println!("{:#?}", tree);

    for n in [5, 25, 123] {
        assert!(tree.search(n))
    }
}