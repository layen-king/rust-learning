use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq, Default)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[allow(dead_code)]
    pub fn get_val(&self) -> i32 {
        self.val
    }
    pub fn insert(&mut self, dir: &str, val: TreeNode) {
        assert!(dir == "left" || dir == "right");
        match dir.as_ref() {
            "left" => self.left = Some(Rc::new(RefCell::new(val))),
            "right" => self.right = Some(Rc::new(RefCell::new(val))),
            _ => {
                panic!("error");
            }
        }
    }
}

/// ## 二叉树中第二小的节点

/// ### 给定一个非空特殊的二叉树，每个节点都是正数，并且每个节点的子节点数量只能为 2 或 0。如果一个节点有两个子节点的话，那么该节点的值等于两个子节点中较小的一个。

/// ### 更正式地说，root.val = min(root.left.val, root.right.val) 总成立。

/// ### 给出这样的一个二叉树，你需要输出所有节点中的第二小的值。如果第二小的值不存在的话，输出 -1
pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn pre_order(node: Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
        if let Some(node) = node {
            let v = node.borrow().val;
            vec.push(v);
            pre_order(node.borrow_mut().left.take(), vec);
            pre_order(node.borrow_mut().right.take(), vec);
        }
    }
    let mut vec = Vec::new();
    pre_order(root, &mut vec);
    vec.sort();
    vec.dedup();
    if vec.len() < 2 {
        return -1;
    }
    vec[1]
}

#[test]
fn test_find_second_minimum_value() {
    let mut tree = TreeNode {
        val: 2,
        ..Default::default()
    };
    let left = TreeNode {
        val: 2,
        ..Default::default()
    };
    let right = TreeNode {
        val: 5,
        ..Default::default()
    };
    tree.insert("left", left);
    tree.insert("right", right);

    println!("tree: {:?}", tree);

    let result = find_second_minimum_value(Some(Rc::new(RefCell::new(tree))));

    assert_eq!(result, 5);
}
