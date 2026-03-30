// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

impl Solution {
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dum = Box::new(ListNode::new(0));
        let mut tail = &mut dum;

        let mut add_next = 0;
        loop {
            let sum = l1.as_ref().map(|n| n.val).unwrap_or_default() + l2.as_ref().map(|n| n.val).unwrap_or_default() + add_next;
            if l1.is_none() && l2.is_none() && sum == 0 {
                break;
            }
            if sum >= 10 {
                add_next = 1;
            } else {
                add_next = 0;
            }
            tail.next = Some(Box::new(ListNode::new(sum%10)));
            tail = tail.next.as_mut().unwrap();
            l1 = if l1.is_some() { l1.as_mut().unwrap().next.take() } else { None };
            l2 = if l2.is_some() { l2.as_mut().unwrap().next.take() } else { None };
        }

        dum.next
    }
}
