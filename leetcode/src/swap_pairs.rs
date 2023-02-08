#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

/// ## 两两交换链表中的节点

/// ### 中等

/// ### 给你一个链表，两两交换其中相邻的节点，并返回交换后链表的头节点。你必须在不修改节点内部的值的情况下完成本题（即，只能进行节点交换）。

/// ###

/// ### 示例 1：

/// ### 输入：head = [1,2,3,4]
/// ### 输出：[2,1,4,3]
/// ### 示例 2：

/// ### 输入：head = []
/// ### 输出：[]
/// ### 示例 3：

/// ### 输入：head = [1]
/// ### 输出：[1]
/// ###

/// ### 提示：

/// ### 链表中节点的数目在范围 [0, 100] 内
/// ### 0 <= Node.val <= 100
/// ### 通过次数418,866提交次数592,253

/// ### 来源：力扣（LeetCode）
/// ### 链接：https://leetcode-cn.com/problems/swap-nodes-in-pairs
/// ### 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    // 迭代方法
    let mut dummy_node = Box::new(ListNode::new(0));
    let mut tem = &mut dummy_node;
    #[allow(unused)]
    let mut nxt_nxt = None;
    while let Some(mut head_node) = head {
        let nxt = head_node.next.take();
        if let Some(mut next_node) = nxt {
            nxt_nxt = next_node.next.take();
            next_node.next = Some(head_node);
            tem.next = Some(next_node);
            tem = tem.next.as_mut().unwrap().next.as_mut().unwrap();
        } else {
            nxt_nxt = None;
            tem.next = Some(head_node);
        }
        head = nxt_nxt;
    }
    dummy_node.next
}

pub fn swap_pairs1(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    head.and_then(|mut node| match node.next {
        Some(mut nxt) => {
            node.next = swap_pairs1(nxt.next);
            nxt.next = Some(node);
            Some(nxt)
        }
        None => Some(node),
    })
}

#[test]
fn test_swap_pairs() {}
