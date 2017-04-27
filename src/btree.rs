use std::collections::LinkedList;
use std::cmp::Ordering;

use variables::{ VARIABLEMAP, VariableState };

use MyOption::MyOption;

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

            let t = self.t.clone() + if self.n && &self.t != "=>" { "!" } else { "" };

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

    pub fn solve(&self, closed: String) -> MyOption<VariableState> {

        let mut result = {

            if self.t == "=>" {

                let left = self.l.as_ref().unwrap();

                left.solve(closed)
            }
            else if self.is_leaf() {

                let var = {
                    let variables = VARIABLEMAP.lock().unwrap();
                    variables[&self.t[..]].clone()
                };
                if var.state == VariableState::Undefined {
                    if closed.contains(&self.t[..]) {
                        MyOption::Error("Inconsistant state leading to infinite loop".to_string())
                    } else {
                        match var.solve(closed + &self.t[..]) {
                            MyOption::Some(state) => {
                                MyOption::Some(
                                    match state {
                                        VariableState::Undefined => VariableState::False,
                                        _ => state
                                    }
                                )
                            },
                            err => err
                        }
                    }
                } else {
                    MyOption::Some(var.state.clone())
                }
            }
            else {

                let left = self.l.as_ref().unwrap();
                let right = self.r.as_ref().unwrap();

                let left = match left.solve(closed.clone()) {
                    MyOption::Some(state) => state,
                    MyOption::Error(s) => return MyOption::Error(s)
                };
                let right = match right.solve(closed.clone()) {
                    MyOption::Some(state) => state,
                    MyOption::Error(s) => return MyOption::Error(s)
                };

                let state = match &self.t[..] {
                    "+" => BTreeNode::solve_and(left, right),
                    "|" => BTreeNode::solve_or(left, right),
                    "^" => BTreeNode::solve_xor(left, right),
                    _ => panic!("Unexpected token {}", self.t)
                };

                MyOption::Some(state)
            }

        };

        if self.n && &self.t[..] != "=>" {
            result = match result {
                MyOption::Some(state) => MyOption::Some(BTreeNode::reverse(state)),
                _ => result
            };
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
    pub root: Option<BTreeNode>,
	root_list: LinkedList<SubRoot>,
	neg: bool,
}

impl BTree {
	pub fn new() -> BTree {
		BTree {
            root: None,
		    root_list: LinkedList::new(),
			neg: false,
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

    pub fn close(&mut self) {

        assert!(self.root_list.len() == 1, "Error building the tree, root_list.len: {}", self.root_list.len());

        let subroot = self.root_list.pop_front().unwrap();

        assert!(subroot.root.is_some(), "Error building the tree, subroot.root is None");

        self.root = subroot.root;
    }

    pub fn to_string(&self) -> String {

        match self.root {

            Some(ref root) => root.to_string(&"".to_string(), true),
            None => "".to_string()
        }
    }

    pub fn display(&self) {
        
        let s = self.to_string();
        
        println!("{}", s);
    }

    pub fn get_rhs(&mut self) -> BTree {

        let root = self.root.as_ref().unwrap();

        let mut rhs = BTree::new();

        rhs.root = Some(*root.r.clone().unwrap());

        rhs
    }

    pub fn solve(&self, closed: String) -> MyOption<VariableState> {

        let root = self.root.as_ref().unwrap();

        root.solve(closed)
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
