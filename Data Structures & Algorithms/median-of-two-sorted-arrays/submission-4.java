class Solution {
    public double findMedianSortedArrays(int[] nums1, int[] nums2) {
        if (nums1.length > nums2.length) {
            return findMedianSortedArrays(nums2, nums1);
        }
        int n = nums1.length + nums2.length;

        int partitionSize = (nums1.length + nums2.length + 1) / 2;
        int left = 0, right = nums1.length;
        int i, j;
        while (left <= right) {
            int l0 = Integer.MIN_VALUE, l1 = Integer.MAX_VALUE, r0 = Integer.MIN_VALUE, r1 = Integer.MAX_VALUE;
            i = (left+right) / 2;
            j = partitionSize - i;
            if (i > 0) {
                l0 = nums1[i-1];
            }
            if (i < nums1.length) {
                l1 = nums1[i];
            }
            if (j > 0) {
                r0 = nums2[j-1];
            }
            if (j < nums2.length) {
                r1 = nums2[j];
            }

            if (l0 <= r1 && l1 >= r0) {
                if (n % 2 == 0) {
                    return ((double) (Math.max(l0, r0) + Math.min(l1, r1))) / 2d;
                }
                return Math.max(l0, r0);
            } else if (l0 > r1) {
                right = i - 1;
            } else {
                left = i + 1;
            }
        }

        return -1;
    }
}
