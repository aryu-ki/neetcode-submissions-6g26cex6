// [6,1,2,3,4,6], target 4

class Solution {
    public int search(int[] nums, int target) {
        int left = 0, right = nums.length - 1;

        int i;
        int cut = -1;
        while (left <= right) {
            i = (left + right + 1) / 2;
            if (i != 0 && nums[i] < nums[i-1]) {
                cut = i;
                break;
            }
            if (nums[left] > nums[i]) {
                right = i - 1;
            } else if (nums[i] > nums[right]) {
                left = i + 1;
            } else {
                right = i - 1;
            }
        }
        if (cut == -1) {
            return search(nums, 0, nums.length-1, target);
        }
        return Math.max(search(nums, 0, cut-1, target), search(nums, cut, nums.length-1, target));
    }

    private int search(int nums[], int left, int right, int target) {
        int i = (left + right) / 2;
        while (left <= right) {
            i = (left + right) / 2;
            if (nums[i] == target) {
                return i;
            }
        
            if (nums[i] > target) {
                right = i - 1;
            } else {
                left = i + 1;
            }
        }

        return -1;
    }
}
