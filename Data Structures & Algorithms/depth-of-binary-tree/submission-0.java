/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode() {}
 *     TreeNode(int val) { this.val = val; }
 *     TreeNode(int val, TreeNode left, TreeNode right) {
 *         this.val = val;
 *         this.left = left;
 *         this.right = right;
 *     }
 * }
 */

class Solution {
    public int maxDepth(TreeNode root) {
        return depth(root, 0);
    }

    private int depth(TreeNode root, int current) {
        if (root == null) {
            return current;
        }
        return Math.max(depth(root.left, current+1), depth(root.right, current+1));
    }
}
