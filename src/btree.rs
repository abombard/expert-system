use std::collections::LinkedList;
use std::cmp::Ordering;

use variables;
use variables::{ VariableMap, VariableState };

#[derive(Clone)]
pub struct BTreeNode {
    v: u8,
    t: String,
    pub n: bool,
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

    pub fn to_string(&self, prev_token: &String, prev_neg: bool) -> String {

        let mut s = String::new();

        if self.is_node() {

            let left = self.l.as_ref().unwrap();
            let right = self.r.as_ref().unwrap();

            if self.t != "=>" && left.is_leaf() && right.is_node() {

                s += &right.to_string(&self.t, self.n);
                s += &left.to_string(&self.t, self.n);
            }
            else {

                s += &left.to_string(&self.t, self.n);
                s += &right.to_string(&self.t, self.n);
            }
        }

        if *prev_token != self.t || prev_neg != self.n {

            let t = self.t.clone() + if self.n { "!" } else { "" };

            s += &t;
        }

        s
    }

    fn solve_and(lhs: VariableState, rhs: VariableState) -> VariableState {
        match (lhs, rhs) {
            (VariableState::True, VariableState::True) => VariableState::True,

            (_, VariableState::False) |
            (VariableState::False, _) => VariableState::False,

            _ => VariableState::Undefined
        }
    }

    fn solve_or(lhs: VariableState, rhs: VariableState) -> VariableState {
        match (lhs, rhs) {
            (VariableState::True, _) |
            (_, VariableState::True) => VariableState::True,

            (VariableState::False, VariableState::False) => VariableState::False,

            _ => VariableState::Undefined
        }
    }

    fn solve_xor(lhs: VariableState, rhs: VariableState) -> VariableState {
        match (lhs, rhs) {
            (VariableState::True, VariableState::False) |
            (VariableState::False, VariableState::True) => VariableState::True,

            (VariableState::True, VariableState::True) |
            (VariableState::False, VariableState::False) => VariableState::False,

            _ => VariableState::Undefined
        }
    }

    fn reverse(v: VariableState) -> VariableState {
        match v {
            VariableState::True => VariableState::False,
            VariableState::False => VariableState::True,
            _ => v
        }
    }

    pub fn solve(&self) -> VariableState {

        let mut result = {

            if self.t == "=>" {

                let left = self.l.as_ref().unwrap();

                left.solve()
            }
            else if self.is_leaf() {

                let variables = VariableMap.lock().unwrap();

                variables[&self.t[..]].state.clone()
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

        };

        if self.n {
            result = BTreeNode::reverse(result);
        }

        result
    }

}   

#[derive(Clone)]
pub struct SubRoot {
    pub root: Option<BTreeNode>,
    neg: bool
}

#[derive(Clone)]
pub struct BTree {
	pub root_list: LinkedList<SubRoot>,
	neg: bool,
	pub state: VariableState
}

impl BTree {
	pub fn new() -> BTree {
		BTree {
		    root_list: LinkedList::new(),
			neg: false,
			state: VariableState::Undefined
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

    pub fn to_string(&self) -> String {

        if let Some(root_option) = self.root_list.front() {
			if let Some(root) = root_option.root.as_ref() {

                let s: String = root.to_string(&"".to_string(), true);
                
                return s;
			}
        }

        "".to_string()
    }

    pub fn display(&self) {
        
        let s = self.to_string();
        
        println!("{}", s);
    }

    pub fn extract_rhs(&mut self) -> String {

        let sub_root = self.root_list.pop_front().unwrap();
        let root = sub_root.root.unwrap();

        let rhs = root.r.unwrap();

        self.root_list.push_front(
            SubRoot {
                root: Some(BTreeNode {
                    t: root.t,
                    v: root.v,
                    n: root.n,
                    l: root.l,
                    r: None
                }),
                neg: sub_root.neg
            }
        );

        rhs.to_string(&rhs.t, rhs.n)
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

    let s = tree.to_string();

    println!("{}", s);

    tree.display();

}
