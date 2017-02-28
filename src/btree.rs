use std::cmp::Ordering;

struct BTreeNode {
    v: u8,
    t: String,
    l: Option<Box<BTreeNode>>,
    r: Option<Box<BTreeNode>>
}

impl BTreeNode {
    pub fn insert(mut self, token: String, value: u8) -> BTreeNode {
        match self.v.cmp(&value) {
            Ordering::Less |
            Ordering::Equal => {
                assert!((self.l.is_some() && self.r.is_some()) ||
                        (self.v == 1 && value != 1),
                        "Unexpected token near '{}': '{}'", self.t, token);
                BTreeNode {
                    v: value,
                    t: token,
                    l: Some(Box::new(self)),
                    r: None
                }
            },
            Ordering::Greater => {
                self.r = match self.r {
                    Some(node) => {
                        Some(Box::new(node.insert(token, value)))
                    },
                    None => {
                        assert!(value == 1,
                                "Unexpected token near '{}': '{}'", self.t, token);
                        Some(Box::new(BTreeNode { v: value, t: token, l: None, r: None }))
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
        if let Some(ref right) = self.r {
            right.display(&self.t);
        }
        if *prev_token != self.t {
            print!("{}", self.t);
        }
    }

}   

struct BTree {
    root: Option<BTreeNode>
}

impl BTree {
    pub fn insert(&mut self, token: &str) {

        let old_root = self.root.take();

        let value = match token {
            "=>" => 5,
            "|"  => 4,
            "^"  => 4,
            "+"  => 3,
            _    => 1
        };

        let new_root =
            match old_root {
                Some(root) => root.insert(token.to_string(), value),
                None => {
                    assert!(value == 1, "Unexpected token: '{}'", token);
                    BTreeNode { v: value, t: token.to_string(), l: None, r: None }
                }
            };

        self.root = Some(new_root);
    }

    pub fn display(&self) {
        if let Some(ref root) = self.root {
            root.display(&"".to_string());
        }
    }
}

fn main() {
    let mut tree = BTree { root: None };

    tree.insert("A");
    tree.insert("+");
    tree.insert("B");
    tree.insert("+");
    tree.insert("C");
    tree.insert("|");
    tree.insert("D");
    tree.insert("=>");
    tree.insert("E");

    tree.display();
    tree.display();
}
