class Solution {
    public int maxArea(int[] heights) {
        if (heights.length < 2) {
            return 0;
        }
        int left = 0;
        int right = heights.length - 1;
        int result = 0;
        while (left < right) {
            result = Math.max(result, Math.min(heights[left], heights[right]) * (right - left));
            if (heights[left] < heights[right]) {
                left++;
            } else {
                right--;
            }
        }

        return result;
    }
}
