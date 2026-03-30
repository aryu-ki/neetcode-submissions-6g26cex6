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
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut head_ref = &mut head;
        while head_ref.is_some() {
            len += 1;
            if len >= k {
                break;
            }
            head_ref = &mut head_ref.as_mut().unwrap().next;
        }

        if len >= k {
            let next_group = Self::reverse_k_group(head_ref.as_mut().unwrap().next.take(), k);
            let mut prev: Option<Box<ListNode>> = next_group;
            let mut curr = head;
            let mut i = 0;
            while let Some(mut c) = curr {
                if i >= k {
                    break;
                }
                let tmp = c.next.take();
                c.next = prev;
                prev = Some(c);
                curr = tmp;
                i += 1;
            }
            prev.take()
        } else {
            head
        }
    }
}
