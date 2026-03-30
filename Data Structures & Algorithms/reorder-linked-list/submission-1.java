/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode() {}
 *     ListNode(int val) { this.val = val; }
 *     ListNode(int val, ListNode next) { this.val = val; this.next = next; }
 * }
 */

class Solution {
    public void reorderList(ListNode head) {
        ListNode curr = head;
        ListNode prev = null;
        int len = 0;
        for (; curr != null; len++) {
            curr = curr.next;
        }
        if (len <= 2) {
            return;
        }
        curr = head.next;
        ListNode next = null;
        for (int i = 1; curr != null; i++) {
            if (i == (len - 1) / 2 + 1) {
                prev.next = null;
                prev = null;
            }
            if (i >= (len - 1) / 2 + 1) {
                next = curr.next;
                curr.next = prev;
                prev = curr;
                curr = next;
            } else {
                prev = curr;
                curr = curr.next;
            }
        }
        ListNode forward = head;
        ListNode backward = prev;
        for (int i = 0; i < len; i++) {
            if (i % 2 == 0) {
                ListNode fwNext = forward.next;
                forward.next = backward;
                forward = fwNext;
            } else {
                ListNode bkNext = backward.next;
                backward.next = forward;
                backward = bkNext;
            }
        }
    }
}
