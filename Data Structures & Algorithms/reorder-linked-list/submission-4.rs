// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut vals = Vec::new();
        let mut cur = head.as_ref();
        while let Some(node) = cur {
            vals.push(node.val);
            cur = node.next.as_ref();
        }

        if vals.len() <= 1 {
            return;
        }

        let mut reordered = Vec::new();
        let mut i = 0;
        let mut j = vals.len() - 1;
        while i <= j {
            reordered.push(vals[i]);
            if i != j {
                reordered.push(vals[j]);
            }
            i += 1;
            if j == 0 { break; }
            j -= 1;
        }

        let mut cur = head.as_mut();
        for &v in &reordered {
            if let Some(node) = cur {
                node.val = v;
                cur = node.next.as_mut();
            }
        }
    }
}