#[allow(dead_code)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Node {
    pub val: usize,
    pub next: Option<Box<Node>>,
    pub random: Option<Box<Node>>,
}

impl Node {
    #[allow(dead_code)]
    fn new(val:usize,next:Option<Box<Node>>,random:Option<Box<Node>>) -> Node {
        Node{val, next,random}
    }
}

// use std::collections::HashMap;

/* pub fn copy_random_list(
    head: Option<Box<Node>>,
    mut cached_node: HashMap<Option<Box<Node>>,Option<Box<Node>>>
) -> Option<Box<Node>> {
    if head.is_none() {
        None()
    }
    if cached_node.get(&head).is_none() {
        let mut head = *head.unwrap();
        cached_node.insert(
            head,
            Node::new(head.val.clone(),
            copy_random_list(head.next,cached_node),
             copy_random_list(head.random,cached_node),)
        );
    }
    Some(cached_node.get(&head))

}
 */
