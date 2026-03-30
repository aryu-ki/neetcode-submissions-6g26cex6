// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut nodes: Vec<i32> = Vec::new();
        let mut cur = &head;
        while let Some(node) = cur {
            nodes.push(node.val);
            cur = &node.next;
        }

        let remove_index = nodes.len() - n as usize;
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut cur = &mut dummy;
        for _ in 0..remove_index {
            cur = cur.next.as_mut().unwrap();
        }
        let next = cur.next.as_mut().unwrap().next.take();
        cur.next = next;
        dummy.next
    }
}