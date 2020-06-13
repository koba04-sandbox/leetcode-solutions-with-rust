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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};

    fn create_nodes(vals: Vec<i32>) -> Option<Box<ListNode>> {
        let mut reverse_values = vals.clone();
        reverse_values.reverse();

        let mut prev: Option<Box<ListNode>> = None;
        for i in reverse_values {
            let mut node = ListNode::new(i);
            if prev.is_some() {
                // ...
                node.next = prev;
            }
            prev = Some(Box::new(node));
        }
        prev
    }

    #[test]
    fn it_works() {
        // Input:
        // [
        // 1->4->5,
        // 1->3->4,
        // 2->6
        // ]
        // Output: 1->1->2->3->4->4->5->6
        assert_eq!(
            Solution::merge_k_lists(vec![
                create_nodes(vec![1, 4, 5]),
                create_nodes(vec![1, 3, 4]),
                create_nodes(vec![2, 6])
            ]),
            create_nodes(vec![1, 1, 2, 3, 4, 4, 5, 6])
        );
    }
}
