use std::cmp::Ordering;

struct BTreeNode {
    v: u8,
    t: String,
    n: bool,
    l: Option<Box<BTreeNode>>,
    r: Option<Box<BTreeNode>>
}

impl BTreeNode {
	fn new(token: &str, value: u8, neg: bool) -> BTreeNode {
		BTreeNode {
		    t: token.to_string(),
		    v: value,
		    n: neg,
		    l: None,
		    r: None
		}
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
                    n: new_node.n,
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

    fn is_leaf(&self) -> bool {
        self.l.is_none() && self.r.is_none()
    }
    fn is_node(&self) -> bool {
        !self.is_leaf()
    }

    pub fn display(&self, prev_token: &String, prev_neg: bool) {

        if let Some(ref left) = self.l {
            if let Some(ref right) = self.r {
                if self.t != "=>" && left.is_leaf() && right.is_node() {
					right.display(&self.t, self.n);
					left.display(&self.t, self.n);
                }
                else {
                    left.display(&self.t, self.n);
                    right.display(&self.t, self.n);
                }
            }
            else {
                left.display(&self.t, self.n);
            }
        }
        if *prev_token != self.t || prev_neg != self.n {
          print!("{}{}", if self.n { "" } else { "!" }, self.t);
        }
    }

}   

use std::collections::LinkedList;

pub struct BTree {
	root_list: LinkedList<Option<BTreeNode>>,
	neg_root: bool,
	neg: bool
}

impl BTree {
	pub fn new() -> BTree {
		BTree {
		    root_list: LinkedList::new(),
		    neg_root: true,
		    neg: true
		}
	}

    pub fn insert(&mut self, token: &str) {

		match token {
			"!" => {
                self.neg = !self.neg;
                return ;
            }
			"(" => {
			    self.neg_root = self.neg;
			    self.neg = true;

			    self.root_list.push_front(None);
            },
			")" => {
				let mut sub_root = self.root_list.pop_front().unwrap().unwrap();

				sub_root.n = self.neg_root;
				sub_root.v = 0;

				self.neg_root = true;

				if let Some(target_option) = self.root_list.pop_front() {
					match target_option {
						Some(mut target) => {
							target = target.insert(sub_root);
							self.root_list.push_front(Some(target));
						},
						None => self.root_list.push_front(Some(sub_root))
					};
				}
				else {
					self.root_list.push_front(Some(sub_root));
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

				let node = BTreeNode::new(token, value, self.neg);
                self.neg = true;

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
				root.display(&"".to_string(), true);
				println!("");
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
