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
      let mut prev: Option<i32> = None;
      let mut current = head;
      let mut acc = vec![];
      while let Some(n) = current {
        if let Some(v) = prev {
          acc.push(n.val);
          acc.push(v);
          prev = None;
        } else {
          prev = Some(n.val);
        }
        current = n.next;
      }
      if let Some(v) = prev {
        acc.push(v)
      }
      Solution::create_list_node(acc)
    }
    pub fn create_list_node(lists: Vec<i32>) -> Option<Box<ListNode>> {
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
}

#[cfg(test)]
mod tests {
    use super::{Solution};
    #[test]
    fn example1() {
        // Given 1->2->3->4, you should return the list as 2->1->4->3.
        assert_eq!(
            Solution::swap_pairs(Solution::create_list_node(vec![1, 2, 3, 4])),
            Solution::create_list_node(vec![2, 1, 4, 3])
        );
    }
    #[test]
    fn example2() {
      // Given 1->2->3->4, you should return the list as 2->1->4->3.
      assert_eq!(
          Solution::swap_pairs(Solution::create_list_node(vec![1])),
          Solution::create_list_node(vec![1])
      );
  }
  }
