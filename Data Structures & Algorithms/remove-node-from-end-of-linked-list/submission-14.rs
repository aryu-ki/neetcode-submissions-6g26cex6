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
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut len = 0;

        let mut head_ref = &mut head;
        while head_ref.is_some() {
            len += 1;
            head_ref = &mut head_ref.as_mut().unwrap().next;
        }

        if len <= 1 {
            return None;
        }


        
        let mut i = 0;
        let mut head_ref = &mut head;
        if n == len {
            return head_ref.as_mut().unwrap().next.take();
        }
        while head_ref.is_some() {
            if i == len - n - 1 {
                let mut removed = head_ref.as_mut().unwrap().next.take();
                head_ref.as_mut().unwrap().next = removed.as_mut().unwrap().next.take();
                break;
            } else {
                head_ref = &mut head_ref.as_mut().unwrap().next;
            }
            i += 1;
        }

        head 
    }
}
