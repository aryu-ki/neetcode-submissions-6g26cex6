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
    int res = Integer.MIN_VALUE;

    public int maxPathSum(TreeNode root) {
        maxPathSumInternal(root);
        return res;
    } 

    private int maxPathSumInternal(TreeNode root) {
       if (root == null) {
            return 0;
        }
        int left =  Math.max(0, maxPathSumInternal(root.left));
        int right =  Math.max(0, maxPathSumInternal(root.right));
        int throughRoot = root.val + left + right;
        res = Math.max(throughRoot, res);
        return  Math.max(left, right) + root.val;
    }
}
