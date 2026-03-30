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
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }
        if lists.len() == 1 {
            return lists.remove(0);
        }
        if lists.len() == 2 {
            let mut dum = Box::new(ListNode::new(0));
            let mut tail = &mut dum;
            let (mut left, mut right) = (lists.remove(0), lists.remove(0));
            let mut should_break = false;
            while let (Some(_), Some(_)) = (left.as_mut(), right.as_mut()) {
                let mut l = left.unwrap();
                let mut r = right.unwrap();
                let mut l_next = None;
                let mut r_next = None;
                if l.val < r.val {
                    l_next = l.next.take();
                    r_next = Some(r);
                    tail.next = Some(l);
                } else {
                    l_next = Some(l);
                    r_next = r.next.take();
                    tail.next = Some(r);
                }
                tail = tail.next.as_mut().unwrap();
                (left, right) = (l_next, r_next)
            }
            tail.next = left.or(right);
            return dum.next;
        }
        let mid = (lists.len() - 1) / 2;
        return Self::merge_k_lists(vec![Self::merge_k_lists(lists[..mid].to_vec()), Self::merge_k_lists(lists[mid..].to_vec())]);
    }
}
