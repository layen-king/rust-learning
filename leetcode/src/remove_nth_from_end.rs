#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/// ## 删除链表的倒数第 N 个结点

/// ### 中等

/// ### 给你一个链表，删除链表的倒数第 n 个结点，并且返回链表的头结点。

/// ###

/// ### 示例 1：

/// ### 输入：head = [1,2,3,4,5], n = 2
/// ### 输出：[1,2,3,5]
/// ### 示例 2：

/// ### 输入：head = [1], n = 1
/// ### 输出：[]
/// ### 示例 3：

/// ### 输入：head = [1,2], n = 1
/// ### 输出：[1]
/// ###

/// ### 提示：

/// ### 链表中结点的数目为 sz
/// ### 1 <= sz <= 30
/// ### 0 <= Node.val <= 100
/// ### 1 <= n <= sz

/// ### 来源：力扣（LeetCode）
/// ### 链接：https://leetcode-cn.com/problems/remove-nth-node-from-end-of-list
/// ### 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
/// ### Definition for singly-linked list.
/// ### Definition for singly-linked list.
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let (mut fast, mut slow) = (&head, &head);
    let mut root = ListNode::new(0);
    let mut curr = &mut root;
    let mut count = 0;
    while count < n {
        fast = &fast.as_ref().unwrap().next;
        count += 1;
    }
    while fast.is_some() {
        fast = &fast.as_ref().unwrap().next;
        let val = slow.as_ref().unwrap().val;
        slow = &slow.as_ref().unwrap().next;
        // 无法共享指针,必须复制
        curr.next = Some(Box::new(ListNode::new(val)));
        curr = curr.next.as_mut().unwrap();
    }
    curr.next = slow.as_ref().unwrap().next.clone();
    root.next
}

#[test]
fn test_remove_nth_from_end() {}
