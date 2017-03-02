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
          print!("{}{}", if self.n { "!" } else { "" }, self.t);
        }
    }

}   

use std::collections::LinkedList;

struct SubRoot {
    root: Option<BTreeNode>,
    neg: bool
}

pub struct BTree {
	root_list: LinkedList<SubRoot>,
	neg: bool
}

impl BTree {
	pub fn new() -> BTree {
		BTree {
		    root_list: LinkedList::new(),
		    neg: false
		}
	}

    pub fn insert(&mut self, token: &str) {

		match token {
			"!" => {
                self.neg = !self.neg;
                return ;
            }
			"(" => {
			    self.root_list.push_front(
			        SubRoot {
			            root: None,
			            neg: self.neg
			        }
			    );
			    self.neg = false;
            },
			")" => {
				let root1 = self.root_list.pop_front().unwrap();
                let mut root1_node = root1.root.unwrap();

                if root1.neg {
                    root1_node.n = !root1.neg;
                }
				root1_node.v = 0;

				if let Some(root2) = self.root_list.pop_front() {
					match root2.root {
						Some(mut root2_node) => {
							root2_node = root2_node.insert(root1_node);
							self.root_list.push_front(
							    SubRoot {
							        root: Some(root2_node),
							        neg: root2.neg
							    }
							);
						},
						None => self.root_list.push_front(
						    SubRoot {
						        root: Some(root1_node),
						        neg: root2.neg
						    }
						)
					};
				}
				else {
					self.root_list.push_front(
					    SubRoot {
					        root: Some(root1_node),
					        neg: false
					    }
					);
				}
			},
			_ => {
				let value = match token {
					"=>" => 5,
					"|"  => 4,
					"^"  => 4,
					"+"  => 3,
					_    => 1
				};

				let node = BTreeNode::new(token, value, self.neg);
                self.neg = true;

                if self.root_list.len() == 0 {
                    self.root_list.push_front(
                        SubRoot {
                            root: Some(node),
                            neg: false
                        }
                    );
                }
                else {
                    let old_root = self.root_list.pop_front().unwrap();

                    let new_root = match old_root.root {
                        Some(root) => root.insert(node),
                        None => node
                    };

                    self.root_list.push_front(
                        SubRoot {
                            root: Some(new_root),
                            neg: old_root.neg
                        }
                    );
                }
			}
		}
	}

    pub fn display(&self) {
        if let Some(root_option) = self.root_list.front() {
			if let Some(root) = root_option.root.as_ref() {
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
