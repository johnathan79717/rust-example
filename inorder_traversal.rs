// https://leetcode.com/problems/binary-tree-inorder-traversal/

struct TreeNode<'a> {
  val: i32,
  left: Option<&'a TreeNode<'a>>,
  right: Option<&'a TreeNode<'a>>,
}

impl<'a> TreeNode<'a> {
  fn new(val: i32) -> TreeNode<'a> {
    TreeNode { val: val, left: None, right: None }
  }
}

fn inorder_traversal<'a>(root: Option<&'a TreeNode<'a>>) {
  match root {
    None => (),
    Some(tree_node) => {
      inorder_traversal(tree_node.left);
      println!("{}", tree_node.val);
      inorder_traversal(tree_node.right);
    },
  }
}

fn main() {
  let a = TreeNode::new(1);
  let b = TreeNode::new(3);
  let t = TreeNode {
    val: 2,
    left: Some(&a),
    right: Some(&b),
  };
  inorder_traversal(Some(&t));
  inorder_traversal(None);
  inorder_traversal(Some(&t));
}

