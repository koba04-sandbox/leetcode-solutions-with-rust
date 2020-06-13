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
        let mut results = vec![];
        let mut current: Vec<&Option<Box<ListNode>>> = lists.iter().map(|n| n).collect::<Vec<_>>();

        while current.iter().any(|n| n.is_some()) {
            let i = Solution::get_latest_nodes(&current);
            if let Some(target) = &current[i].as_ref() {
                results.push(target.val);
                current[i] = &target.next;
            }
            // println!("{:?}", current);
        }
        Solution::create_list_node(results)
    }
    pub fn create_list_node(vals: Vec<i32>) -> Option<Box<ListNode>> {
        let mut last: Option<Box<ListNode>> = None;
        for i in vals.into_iter().rev() {
            let mut node = ListNode::new(i);
            node.next = last;
            last = Some(Box::new(node));
        }
        last
    }
    fn get_latest_nodes(lists: &Vec<&Option<Box<ListNode>>>) -> usize {
        let mut min = None;
        let mut min_index: usize = 0;
        let mut i = 0;
        for node in lists {
            if let Some(n) = node {
                let val = n.val;
                match min {
                    Some(m) => {
                        if m > val {
                            min = Some(val);
                            min_index = i;
                        }
                    },
                    None => {
                        min = Some(val);
                        min_index = i;
                    }
                }
            }
            i += 1;
        }
        min_index
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        // Input:
        // [
        // 1->4->5,
        // 1->3->4,
        // 2->6
        // ]
        // Output: 1->1->2->3->4->4->5->6
        assert_eq!(
            Solution::merge_k_lists(vec![
                Solution::create_list_node(vec![1, 4, 5]),
                Solution::create_list_node(vec![1, 3, 4]),
                Solution::create_list_node(vec![2, 6])
            ]),
            Solution::create_list_node(vec![1, 1, 2, 3, 4, 4, 5, 6])
        );
    }
    #[test]
    fn example2() {
        assert_eq!(
            Solution::merge_k_lists(vec![
                Solution::create_list_node(vec![0]),
                Solution::create_list_node(vec![1])
            ]),
            Solution::create_list_node(vec![0, 1])
        );
    }
    #[test]
    fn example3() {
        // [[],[-1,5,11],[],[6,10]]
        assert_eq!(
            Solution::merge_k_lists(vec![
                Solution::create_list_node(vec![]),
                Solution::create_list_node(vec![-1,5,11]),
                Solution::create_list_node(vec![]),
                Solution::create_list_node(vec![6, 10])
            ]),
            Solution::create_list_node(vec![-1,5,6,10,11])
        );

    }
}
