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

/// ### 合并两个有序链表

/// ### 简单

/// ### 将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的

/// ### 示例 2：

/// ### 输入：l1 = [], l2 = []
/// ### 输出：[]
/// ### 示例 3：

/// ### 输入：l1 = [], l2 = \[0\]
/// ### 输出：\[0\]
/// ###

/// ### 提示：

/// ### 两个链表的节点数目范围是 \[0, 50\]
/// ### -100 <= Node.val <= 100
/// ### l1 和 l2 均按 非递减顺序 排列

/// ### 来源：力扣（LeetCode）
/// ### 链接：https://leetcode-cn.com/problems/merge-two-sorted-lists
pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(-1);
    let (mut tem, mut p1, mut p2) = (&mut dummy, l1, l2);
    while let (Some(n1), Some(n2)) = (p1.as_ref(), p2.as_ref()) {
        if n1.val <= n2.val {
            tem.next = p1;
            tem = tem.next.as_mut().unwrap();
            p1 = tem.next.take();
        } else {
            tem.next = p2;
            tem = tem.next.as_mut().unwrap();
            p2 = tem.next.take();
        }
    }
    tem.next = if p1.is_some() { p1 } else { p2 };
    dummy.next
}

pub fn merge_two_lists1(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (Some(n1), Some(n2)) => {
            if n1.val <= n2.val {
                Some(Box::new(ListNode {
                    val: n1.val,
                    next: merge_two_lists1(n1.next, Some(n2)),
                }))
            } else {
                Some(Box::new(ListNode {
                    val: n2.val,
                    next: merge_two_lists1(Some(n1), n2.next),
                }))
            }
        }
        (Some(n1), None) => Some(n1),
        (None, Some(n2)) => Some(n2),
        _ => None,
    }
}

#[test]
pub fn test_merge_two_lists() {
    let mut k1 = ListNode::new(1);
    let k2 = ListNode::new(2);
    k1.next = Some(Box::new(k2));

    let mut x1 = ListNode::new(1);
    let x2 = ListNode::new(3);
    x1.next = Some(Box::new(x2));

    // let result = merge_two_lists(Some(Box::new(k1)), Some(Box::new(x1)));
    // println!("{:?}", result);

    let result = merge_two_lists1(Some(Box::new(k1)), Some(Box::new(x1)));
    println!("{:?}", result);
}
