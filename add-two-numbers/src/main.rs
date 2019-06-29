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

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    /*
    let mut str1 = String::from("");
    let mut str2 = String::from("");
    let mut lnode = l1;
    while let Some(l) = lnode {
        str1 = String::from(format!("{}", l.val)) + &str1;
        lnode = l.next;
    }
    let mut rnode = l2;
    while let Some(r) = rnode {
        str2 = String::from(format!("{}", r.val)) + &str2;
        rnode = r.next;
    }
    // TODO: Should calculate each digits of lnode and rnode
    let answer = str1.parse::<u64>().unwrap() + str2.parse::<u64>().unwrap();
    */

    let mut lnode = l1;
    let mut rnode = l2;
    let mut digits: Vec<i32> = vec![];
    let mut prev = 0;
    while lnode != None || rnode != None {
        let l = lnode.unwrap();
        let r = rnode.unwrap();
        let a = l.val + r.val + prev;
        digits.push(a % 10);
        prev = if a >= 10 { 1 } else { 0 };

        lnode = l.next;
        rnode = r.next;
    }
    digits.reverse();

    // let digits: Vec<u32> = answer.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    println!("digit {:?}", digits);

    fn add_node(node: Option<Box<ListNode>>, mut target: Box<ListNode>) -> Option<Box<ListNode>> {
        match node {
            Some(n) => {
                // println!("node {:?}", n);
                target.next = Some(n);
                Some(target)
            },
            None => Some(target)
        }
    }
   let mut node = None;
    for d in digits {
        node = add_node(node, Box::new(ListNode::new(d as i32)));
    }
    node
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let mut lnode1 = Box::new(ListNode::new(2));
        let mut lnode2 = Box::new(ListNode::new(4));
        let lnode3 = Box::new(ListNode::new(3));

        lnode2.next = Some(lnode3);
        lnode1.next = Some(lnode2);

        let mut rnode1 = Box::new(ListNode::new(5));
        let mut rnode2 = Box::new(ListNode::new(6));
        let rnode3 = Box::new(ListNode::new(4));

        rnode2.next = Some(rnode3);
        rnode1.next = Some(rnode2);

        let mut anode1 = Box::new(ListNode::new(7));
        let mut anode2 = Box::new(ListNode::new(0));
        let anode3 = Box::new(ListNode::new(8));

        anode2.next = Some(anode3);
        anode1.next = Some(anode2);
        assert_eq!(add_two_numbers(Some(lnode1), Some(rnode1)), Some(anode1));
    }

    /*
    #[test]
    fn test_add_two_numbers2() {
        let mut lnode1 = Box::new(ListNode::new(2));
        let mut lnode2 = Box::new(ListNode::new(4));
        let lnode3 = Box::new(ListNode::new(3));

        lnode2.next = Some(lnode3);
        lnode1.next = Some(lnode2);

        let mut rnode1 = Box::new(ListNode::new(5));
        let rnode2 = Box::new(ListNode::new(6));

        rnode1.next = Some(rnode2);

        let mut anode1 = Box::new(ListNode::new(7));
        let mut anode2 = Box::new(ListNode::new(0));
        let anode3 = Box::new(ListNode::new(4));

        anode2.next = Some(anode3);
        anode1.next = Some(anode2);
        assert_eq!(add_two_numbers(Some(lnode1), Some(rnode1)), Some(anode1));
    }
    */
}
