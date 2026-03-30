impl Solution {
    pub fn reorder_list(start: &mut Option<Box<ListNode>>) {
        let mut head = &mut *start;
        
        let mut len = 0;
        while head.is_some() {
            len += 1;
            head = &mut head.as_mut().unwrap().next;
        }

        head = &mut *start;
        let mut i = 0;
        while i <= (len - 1) / 2 {
            head = &mut head.as_mut().unwrap().next;
            i += 1;
        }

        let mut head = head.take();
        let mut prev: Option<Box<ListNode>> = None;
        for _ in i..len {
            let next = head.as_mut().unwrap().next.take();
            head.as_mut().unwrap().next = prev;
            prev = Some(head.unwrap());
            head = next;
        }


        let mut head = &mut *start;
        let mut backward = prev;
        while head.is_some() && backward.is_some() {
            let nxt = head.as_mut().unwrap().next.take();
            let next_backward = backward.as_mut().unwrap().next.take();
            backward.as_mut().unwrap().next = nxt;
            head.as_mut().unwrap().next = backward;
            head = &mut head.as_mut().unwrap().next.as_mut().unwrap().next;
            backward = next_backward;
        }



    }
}