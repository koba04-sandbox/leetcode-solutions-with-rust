pub struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{Solution, ListNode};

    #[test]
    fn it_works() {
        // Input: 1->2->4, 1->3->4
        // Output: 1->1->2->3->4->4

        let mut anode1 = ListNode::new(1);
        let mut anode2 = ListNode::new(2);
        let anode3 = ListNode::new(3);
        anode2.next = Some(Box::new(anode3));
        anode1.next = Some(Box::new(anode2));

        let mut bnode1 = ListNode::new(1);
        let mut bnode2 = ListNode::new(3);
        let bnode3 = ListNode::new(4);
        bnode2.next = Some(Box::new(bnode3));
        bnode1.next = Some(Box::new(bnode2));

        let mut cnode1 = ListNode::new(1);
        let mut cnode2 = ListNode::new(1);
        let mut cnode3 = ListNode::new(2);
        let mut cnode4 = ListNode::new(3);
        let mut cnode5 = ListNode::new(4);
        let cnode6 = ListNode::new(4);

        cnode5.next = Some(Box::new(cnode6));
        cnode4.next = Some(Box::new(cnode5));
        cnode3.next = Some(Box::new(cnode4));
        cnode2.next = Some(Box::new(cnode3));
        cnode1.next = Some(Box::new(cnode2));

        assert_eq!(
            Solution::merge_two_lists(Some(Box::new(anode1)), Some(Box::new(bnode1))),
            Some(Box::new(cnode1))
        );
    }
}
