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
    public ListNode removeNthFromEnd(ListNode head, int n) {
        ListNode prev = null;
        ListNode curr = head;
        int len = 0;
        for (; curr != null; len++, curr = curr.next) {}
        n = len - n;
        curr = head;
        prev = null;
        for (int i = 0; i < n; i++) {
            prev = curr;
            curr = curr.next;
        }
        if (prev == null) {
            return curr.next;
        }
        prev.next = curr.next;
        return head;
    }
}
