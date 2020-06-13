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

pub struct Solution {}

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{Solution, ListNode};

    fn create_list_node(lists: Vec<i32>) -> Option<Box<ListNode>> {
        let mut target: Option<Box<ListNode>> = None;
        for val in lists.into_iter().rev() {
            let mut node = Box::new(ListNode::new(val));
            if target.is_some() {
                node.next = target;
            }
            target = Some(node);
        }
        target
    }

    #[test]
    fn it_works() {
        // Given 1->2->3->4, you should return the list as 2->1->4->3.
        assert_eq!(
            Solution::swap_pairs(create_list_node(vec![1, 2, 3, 4])),
            create_list_node(vec![2, 1, 4, 3])
        );
    }
}
