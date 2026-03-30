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
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let None = list1 {
            return list2;
        }
        if let None = list2 {
            return list1;
        }
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        while let(Some(first), Some(second)) = (list1.as_ref(), list2.as_ref()) {
            if second.val > first.val {
                tail.next = list1;
                tail = tail.next.as_mut().unwrap();
                list1 = tail.next.take();
            } else {
                tail.next = list2;
                tail = tail.next.as_mut().unwrap();
                list2 = tail.next.take();
            }
        }

        tail.next = if list1.is_some() { list1 } else if list2.is_some() { list2 } else { None };
        dummy.next
    }
}
