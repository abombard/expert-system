use std::cmp::Ordering;
use std::collections::LinkedList;

struct BTreeNode {
    v: u8,
    t: String,
    l: Option<Box<BTreeNode>>,
    r: Option<Box<BTreeNode>>
}

impl BTreeNode {
	fn new(token: &str, value: u8) -> BTreeNode {
		BTreeNode { t: token.to_string(), v: value, l: None, r: None }
	}

    pub fn insert(mut self, new_node: BTreeNode) -> BTreeNode {
        match self.v.cmp(&new_node.v) {
            Ordering::Less |
            Ordering::Equal => {
                assert!((self.l.is_some() && self.r.is_some()) ||
                        (self.v == 1 && new_node.v != 1),
                        "Unexpected token near '{}': '{}'", self.t, new_node.t);
                BTreeNode {
                    v: new_node.v,
                    t: new_node.t,
                    l: Some(Box::new(self)),
                    r: None
                }
            },
            Ordering::Greater => {
                self.r = match self.r {
                    Some(node) => {
                        Some(Box::new(node.insert(new_node)))
                    },
                    None => {
                        assert!(new_node.v <= 1,
                                "Unexpected token near '{}': '{}'", self.t, new_node.t);
                        Some(Box::new(new_node))
                    }
                };
                self
            }
        }
    }

    pub fn display(&self, prev_token: &String) {
        if let Some(ref left) = self.l {
            left.display(&self.t);
        }
		print!("{}", self.t);
        if let Some(ref right) = self.r {
            right.display(&self.t);
        }
//        if *prev_token != self.t {
//          print!("{}", self.t);
// }
    }
}   

struct BTree {
	root_list: LinkedList<Option<BTreeNode>>
}

impl BTree {
	fn new() -> BTree {
		BTree { root_list: LinkedList::new() }
	}

    pub fn insert(&mut self, token: &str) {
		match token {
			"(" => self.root_list.push_front(None),
			")" => {
				let mut arthur_root = self.root_list.pop_front().unwrap().unwrap();

				arthur_root.v = 0;

				if let Some(target_option) = self.root_list.pop_front() {
						let mut target = target_option.unwrap();
						println!("bonjour arthur");
						target = target.insert(arthur_root);
						self.root_list.push_front(Some(target));
				}
				else {
					self.root_list.push_front(Some(arthur_root));
				}
			},
			_ => {
				let old_root = self.root_list.pop_front();

				let value = match token {
					"=>" => 5,
					"|"  => 4,
					"^"  => 4,
					"+"  => 3,
					_    => 1
				};

				let node = BTreeNode::new(token, value);
				let new_root =
				match old_root {
					Some(root) => match root {
						Some(root) => root.insert(node),
						None => {
							assert!(value == 1, "Unexpected token: '{}'", token);
							node
						}
					},
					None => {
						assert!(value == 1, "Unexpected token: '{}'", token);
						node
					}
				};

				self.root_list.push_front(Some(new_root));
			}
		}
	}

    pub fn display(&self) {
        if let Some(root_option) = self.root_list.front() {
				if let Some(root) = root_option.as_ref() {
						root.display(&"".to_string());
					println!();
				}
        }
    }
}

fn main() {
    let mut tree = BTree::new();

    tree.insert("A");
    tree.insert("|");
    tree.insert("B");
    tree.insert("+");
	tree.insert("(");
    tree.insert("F");
    tree.insert("|");
    tree.insert("G");
	tree.insert(")");
	tree.insert("+");
    tree.insert("C");
    tree.insert("|");
    tree.insert("D");
    tree.insert("=>");
    tree.insert("E");

/*
	tree.insert("(");
    tree.insert("F");
    tree.insert("|");
    tree.insert("G");
    tree.insert(")");
    tree.insert("+");
    tree.insert("C");
    tree.insert("|");
    tree.insert("D");
    tree.insert("=>");
	tree.insert("E");
*/

    tree.display();
    tree.display();
}
