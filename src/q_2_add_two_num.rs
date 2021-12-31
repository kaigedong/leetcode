// Definition for singly-linked list.

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

// 给你两个 非空 的链表，表示两个非负的整数。它们每位数字都是按照 逆序 的方式存储的，并且每个节点只能存储 一位 数字。
// 请你将两个数相加，并以相同形式返回一个表示和的链表。
// 你可以假设除了数字 0 之外，这两个数都不会以 0 开头。

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(l1), Some(l2)) => {
                add_two_listnode(Some(l1), Some(l2));

            }
            (Some(l1), None) => return Some(l1),
            (None, Some(l2)) => return Some(l2),
            (None, None) => return None,
        }

        return None;
    }
}

// 返回一个bool表示是否进位
fn add_two_listnode(node1: Option<Box<ListNode>>, node2: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, i32) {
    let mut carry = 0;
    let mut node = ListNode::new(0);
    node.val = match (node1, node2) {
        (Some(node1), Some(node2)) => {
            let val = node1.val + node2.val;
            if val >= 10 {
                carry = 1;
                val - 10
            } else {
                val
            }
        }
        (Some(node1), None) => node1.val,
        (None, Some(node2)) => node2.val,
        (None, None) => return (None, 0),
    };

    return (Some(Box::new(node)), carry);
}

#[test]
fn test_add_two() {
    // 输入：l1 = [2,4,3], l2 = [5,6,4]
    // 输出：[7,0,8]
    // 解释：342 + 465 = 807.

    // 输入：l1 = [0], l2 = [0]
    // 输出：[0]

    // 输入：l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
    // 输出：[8,9,9,9,0,0,0,1]
}
