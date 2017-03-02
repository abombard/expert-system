use std::collections::LinkedList;
use std::cmp::Ordering;

use variables;
use variables::{ VariableState };

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

        if self.is_node() {
            let left = self.l.as_ref().unwrap();
            let right = self.r.as_ref().unwrap();

            if self.t != "=>" && left.is_leaf() && right.is_node() {
                right.display(&self.t, self.n);
                left.display(&self.t, self.n);
            }
            else {
                left.display(&self.t, self.n);
                right.display(&self.t, self.n);
            }
        }

        if *prev_token != self.t || prev_neg != self.n {
          print!("{}{}", self.t, if self.n { "!" } else { "" });
        }
    }

    fn solve_and(lhs: VariableState, rhs: VariableState) -> VariableState {
        match (lhs, rhs) {
            (VariableState::True, VariableState::True) => VariableState::True,
            (_, VariableState::False) => VariableState::False,
            (VariableState::False, _) => VariableState::False,
            (VariableState::Unsolved, _) => VariableState::Unsolved,
            (_, VariableState::Unsolved) => VariableState::Unsolved,
            _ => VariableState::Undefined
        }
    }

    fn solve_or(lhs: VariableState, rhs: VariableState) -> VariableState {
        match (lhs, rhs) {
            (VariableState::True, _) => VariableState::True,
            (_, VariableState::True) => VariableState::True,
            (VariableState::Unsolved, _) => VariableState::Unsolved,
            (_, VariableState::Unsolved) => VariableState::Unsolved,
            (VariableState::Undefined, _) => VariableState::Undefined,
            (_, VariableState::Undefined) => VariableState::Undefined,
            _ => VariableState::False
        }
    }

    fn solve_xor(lhs: VariableState, rhs: VariableState) -> VariableState {
        match (lhs, rhs) {
            _ => VariableState::Undefined
        }
    }

    pub fn solve(&self) -> VariableState {
        if self.is_leaf() {
            variables::State[&self.t[..]].clone()
        }
        else {
            let left = self.l.as_ref().unwrap();
            let right = self.r.as_ref().unwrap();

            match &self.t[..] {
                "+" => BTreeNode::solve_and(left.solve(), right.solve()),
                "|" => BTreeNode::solve_or(left.solve(), right.solve()),
                "^" => BTreeNode::solve_xor(left.solve(), right.solve()),
                _ => panic!("Unexpected token {}", self.t)
            }

        }
    }

}   


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
                    root1_node.n = !root1_node.n;
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
                self.neg = false;

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

    pub fn solve(&self) -> VariableState {
        let root = self.root_list.front().unwrap().root.as_ref().unwrap();
        root.solve()
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
