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
    let answer = str1.parse::<i32>().unwrap() + str2.parse::<i32>().unwrap();

    let mut digits: Vec<u32> = answer.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    digits.reverse();

    let mut nodes = vec![];
    for d in digits {
        println!("digit {}", d);
        nodes.push(Box::new(ListNode::new(d as i32)));
    }

    // How can I create a linked-list?
    // (digits[0] -> digits[1] -> digits[2])
    None
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
}
