// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution {}

impl ListNode {
    fn remove(&mut self) {
        if let Some(node) = self.next.take() {
            self.next = node.next;
        }
    }
}

impl Solution {
    fn get_target_index(head: &Option<Box<ListNode>>, n: i32) -> i32 {
        let mut i = 0;
        let mut a = head;
        while let Some(aa) = a {
            i += 1;
            if aa.next.is_some() {
                a = &aa.next;
            } else {
                break;
            }
        }
        let target = i - n - 1;
        target
    }
    fn remove_node(head: &mut Option<Box<ListNode>>, target: i32) -> bool {
        let mut j = 0;
        let mut h = head;
        loop {
            if let Some(a) = h {
                println!("val: {}, j: {}", a.val, j);
                if j == target {
                    a.remove();
                    return true;
                } else {
                    h = &mut a.next;
                }
            } else {
                break;
            }
            j += 1;
        }
        false
    }
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let target = Solution::get_target_index(&head, n);
        println!("target is {}", target);
        if Solution::remove_node(&mut head, target) {
            return head;
        }
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
        let mut node3 = ListNode::new(2);
        let mut node4 = ListNode::new(3);
        let node5 = ListNode::new(5);
        node4.next = Some(Box::new(node5));
        node3.next = Some(Box::new(node4));
        expected_node1.next = Some(Box::new(node3));

        assert_eq!(
            Solution::remove_nth_from_end(Some(Box::new(node1)), 2),
            Some(Box::new(expected_node1))
        );
    }

    #[test]
    fn example2() {
        let node1 = ListNode::new(1);
        assert_eq!(
            Solution::remove_nth_from_end(Some(Box::new(node1)), 1),
            None
        );
    }

    #[test]
    fn example3() {
        let mut node1 = ListNode::new(1);
        let node2 = ListNode::new(2);
        node1.next = Some(Box::new(node2));

        let expected_node1 = ListNode::new(2);

        assert_eq!(
            Solution::remove_nth_from_end(Some(Box::new(node1)), 2),
            Some(Box::new(expected_node1))
        );

    }
}
