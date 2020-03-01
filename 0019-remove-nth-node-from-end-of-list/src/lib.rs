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

pub struct Solution {}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let mut node1 = ListNode::new(1);
        let mut node2 = ListNode::new(2);
        let mut node3 = ListNode::new(3);
        let mut node4 = ListNode::new(4);
        let node5 = ListNode::new(5);
        node4.next = Some(Box::new(node5));
        node3.next = Some(Box::new(node4));
        node2.next = Some(Box::new(node3));
        node1.next = Some(Box::new(node2));

        let mut expected_node1 = ListNode::new(1);
        let mut node3 = ListNode::new(3);
        let mut node4 = ListNode::new(4);
        let node5 = ListNode::new(5);
        node4.next = Some(Box::new(node5));
        node3.next = Some(Box::new(node4));
        expected_node1.next = Some(Box::new(node3));

        assert_eq!(
            Solution::remove_nth_from_end(Some(Box::new(node1)), 2),
            Some(Box::new(expected_node1))
        );
    }
}
