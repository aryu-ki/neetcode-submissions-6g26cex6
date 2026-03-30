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
    public ListNode reverseKGroup(ListNode head, int k) {
        if (k == 1) {
            return head;
        }
        int groups = 0;
        int len = 0;
        ListNode curr = head, prev = null;
        ListNode currGroupStart = head, lastReversedGroupEnd = null;
        for (; curr != null; len++, curr = curr.next) {}
        curr = head;
        prev = null;
        for (int i = 0; i < len - len % k; i++) {
            if (i % k == k - 1) {
                if (groups == 0) {
                    head = curr;
                    groups++;
                }
                if (lastReversedGroupEnd != null) {
                    lastReversedGroupEnd.next = curr;
                }
                lastReversedGroupEnd = currGroupStart;
                lastReversedGroupEnd.next = curr.next;
                var next = curr.next;
                curr.next = prev;
                prev = lastReversedGroupEnd;
                currGroupStart = next;
                curr = next;
            } else {
                var next = curr.next;
                curr.next = prev;
                prev = curr;
                curr = next;
            }
        }
        return head;
    }
}
