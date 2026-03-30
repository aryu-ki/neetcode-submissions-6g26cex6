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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let None = head { return None; }
        let mut head = head;
        let mut prev = None;
        while let Some(mut h) = head {
            let tmp = h.next;
            h.next = prev;
            prev = Some(h);
            head = tmp;
        }
        prev
    }

}
