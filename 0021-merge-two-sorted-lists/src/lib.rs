pub struct Solution {}

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

impl ListNode {
  fn append(&mut self, val: i32) {
    if self.next.is_none() {
      self.next = Some(Box::new(ListNode::new(val)));
    } else {
      let mut current = self.next.as_mut();
      while let Some(a) = current {
        if a.next.is_none() {
          a.next = Some(Box::new(ListNode::new(val)));
          break;
        }
        current = a.next.as_mut();
      }
    }
  }
}

impl Solution {
  pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    Solution::merge(&l1, &l2)
  }
  fn merge(l1: &Option<Box<ListNode>>, l2: &Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut current1 = l1;
    let mut current2 = l2;
    let mut tmp = vec![];

    loop {
      if current1.is_none() && current2.is_none() {
        break;
      }
      if current1.is_some() && current2.is_some() {
        let a = current1.as_ref().unwrap().val;
        let b = current2.as_ref().unwrap().val;
        if a < b {
          tmp.push(a);
          current1 = &current1.as_ref().unwrap().next;
        } else {
          tmp.push(b);
          current2 = &current2.as_ref().unwrap().next;
        }
      } else if current1.is_none() {
        let b = current2.as_ref().unwrap().val;
        tmp.push(b);
        current2 = &current2.as_ref().unwrap().next;
      } else {
        let a = current1.as_ref().unwrap().val;
        tmp.push(a);
        current1 = &current1.as_ref().unwrap().next;
      }
    }

    if tmp.len() == 0 {
      return None;
    }

    // println!("{:?}", tmp);
    let mut iter = tmp.iter();
    let mut result = Some(Box::new(ListNode {
      val: *iter.next().unwrap(),
      next: None,
    }));
    for val in iter {
      result.as_mut().unwrap().append(*val);
    }
    result
  }
}

#[cfg(test)]
mod tests {
  use super::{ListNode, Solution};

  #[test]
  fn example1() {
    // Input: 1->2->4, 1->3->4
    // Output: 1->1->2->3->4->4

    let mut anode1 = ListNode::new(1);
    let mut anode2 = ListNode::new(2);
    let anode3 = ListNode::new(4);
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

  #[test]
  fn example2() {
    assert_eq!(
      Solution::merge_two_lists(None, None),
      None
    );
  }

}
