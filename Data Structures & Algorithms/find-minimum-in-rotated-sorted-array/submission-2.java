class Solution {
    public int findMin(int[] nums) {
        int left = 0, right = nums.length - 1;

        int i;
        while (left <= right) {
            i = (left + right + 1) / 2;
            if (i == 0 || nums[i] < nums[i-1]) {
                return nums[i];
            }
            if (nums[left] > nums[i]) {
                right = i - 1;
            } else if (nums[i] > nums[right]) {
                left = i + 1;
            } else {
                right = i - 1;
            }
        }

        throw new IllegalStateException();
    }
}
