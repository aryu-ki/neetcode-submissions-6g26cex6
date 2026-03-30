// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: *mut ListNode,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: std::ptr::null_mut(), val }
//     }
// }

impl Solution {
    pub fn has_cycle(mut head: *mut ListNode) -> bool {
        if head.is_null() {
            return false;
        }
        unsafe {
            while !(*head).next.is_null() {
                if (*(*head).next).val == -1001i32 {
                    return true;
                }
                (*head).val = -1001i32;
                head = (*head).next;
            }
            return false;
        }
    }
}
