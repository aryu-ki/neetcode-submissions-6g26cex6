class Solution {
    public int trap(int[] height) {
        if (height.length < 3) {
            return 0;
        }

        int[] prefixMax, suffixMax;
        prefixMax = new int[height.length];
        suffixMax = new int[height.length];

        prefixMax[0] = 0;
        prefixMax[1] = height[0];
        for (int i = 2; i < height.length; i++) {
            prefixMax[i] = Math.max(height[i-1], prefixMax[i-1]);
        }

        suffixMax[height.length-1] = 0;
        suffixMax[height.length-2] = height[height.length-1];
        for (int i = height.length - 3; i >= 0; i--) {
            suffixMax[i] = Math.max(height[i+1], suffixMax[i+1]);
        }

        int result = 0;
        for (int i = 0; i < height.length; i++) {
            int trappedI = Math.min(prefixMax[i], suffixMax[i]) - height[i];
            if (trappedI > 0) {
                result += trappedI;
            }
        }

        return result;
    }
}
 