// Define the Binary Tree Node
#[derive(Debug)]
struct ParseNode {
    value: i32,
    left: Option<Box<ParseNode>>,
    right: Option<Box<ParseNode>>,
}

// Implementation of methods for TreeNode
impl ParseNode {
    fn new(value: i32) -> Self {
        ParseNode {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: i32) {
        if value <= self.value {
            if let Some(ref mut left) = self.left {
                left.insert(value);
            } else {
                self.left = Some(Box::new(ParseNode::new(value)));
            }
        } else {
            if let Some(ref mut right) = self.right {
                right.insert(value);
            } else {
                self.right = Some(Box::new(ParseNode::new(value)));
            }
        }
    }

    // Inorder traversal of the tree
    fn inorder(&self) {
        if let Some(ref left) = self.left {
            left.inorder();
        }
        println!("{}", self.value);
        if let Some(ref right) = self.right {
            right.inorder();
        }
    }
}

fn main() {
    let mut root = ParseNode::new(5);
    root.insert(3);
    root.insert(7);
    root.insert(2);
    root.insert(4);
    root.insert(6);
    root.insert(8);

    println!("Inorder traversal:");
    root.inorder();
}
