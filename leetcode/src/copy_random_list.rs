#[allow(dead_code)]
pub struct Node {
    pub val: usize,
    pub next: Option<Box<Node>>,
    pub random: Option<Box<Node>>,
}

use std::collections::HashMap;

#[allow(dead_code)]
pub fn copy_random_list(
    head: Option<Box<Node>>,
    cached_node: HashMap<Node, Node>,
) -> Option<Box<Node>> {
    if head.is_none() {
        None
    }else{
      todo!("")
    }
    todo!("")
}

#[test]
fn test_copy_random_list() {}
