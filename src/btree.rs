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
                    None => Some(Box::new(BTreeNode { v: value, t: token, l: None, r: None }))
                };
                self
            }
        }
    }
}   

struct BTree {
    root: Option<BTreeNode>
}

impl BTree {
    pub fn insert(&mut self, token: &str) {

        let oldRoot = self.root.take();

        let value = match token {
            "=>" => 5,
            "|"  => 4,
            "^"  => 4,
            "+"  => 3,
            _    => 1
        };

        let newRoot =
            match oldRoot {
                Some(root) => root.insert(token.to_string(), value),
                None => BTreeNode { v: value, t: token.to_string(), l: None, r: None }
            };

        self.root = Some(newRoot);
    }
}

fn main() {
    let mut root = BTree { root: None };

    root.insert("A");
    root.insert("+");
    root.insert("B");
    root.insert("|");
    root.insert("C");
}
