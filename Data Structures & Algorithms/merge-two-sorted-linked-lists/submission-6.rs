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
        // inserting list1 into list2
        // let's have a temporary node to add first els to
        // let's also store pointer to the first element to return later in dum
        // consider list1 and list2 current elements of corresponding lists
        // do 
            // while list2 is None || list1 < list2 
            //     temp.next is now list1 (link previous and new)
            //     temp is now temp.next (or list1)
            //     list1 is now list1.next (iteration)
            // temp.next is now list2 (since we've added all els from list1 that are slmaller or equal to this one) 
            // temp is now list2
            // list2 is now list.next (next element to compare to) 
        // while list2 is Some (intentional late check to add all remaining elements to the end)
        // return dum
        if let (None, None) = (list1.as_ref(), list2.as_ref()) {
            return None;
        }
        let mut dum = Box::new(ListNode::new(0));
        let mut temp = &mut dum;

        loop {
            while let (Some(l1), l2) = (list1.as_ref(), list2.as_ref()) {
                if l2.is_none() || l1.val <= l2.unwrap().val {
                    temp.next = list1;
                    temp = temp.next.as_mut().unwrap();
                    list1 = temp.next.take();
                } else {
                    break;
                }
            }
            temp.next = list2;
            if let None = temp.next {
                break;
            }
            temp = temp.next.as_mut().unwrap();
            list2 = temp.next.take();
        }

        dum.next
    }
}
