class Solution {
    public int minEatingSpeed(int[] piles, int h) {
        int lo = 1, hi = max(piles);
        int k = hi;
        int result = -1;
        while (lo <= hi) {
            k = (hi + lo) / 2;
            int curr = 0;
            for (int pile : piles) {
                curr += pile / k;
                if (pile % k != 0) {
                    curr++;
                }
            }
            if (curr <= h) {
                hi = k - 1;
                result = k;
            } else {
                lo = k + 1;
            }
        }
        return result;
    }
    
    private int max(int... values) {
        if (values == null || values.length == 0) {
            throw new IllegalArgumentException();
        }
        int max = values[0];
        for (int i = 1; i < values.length; i++) {
            max = Math.max(values[i], max);
        }
        return max;
    }
}
