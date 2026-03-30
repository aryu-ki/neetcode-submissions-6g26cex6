class Solution {
    private record El(int val, int i) implements Comparable<El>{
        public int compareTo(El other) {
            return val - other.val;
        }
    }
    public int[] maxSlidingWindow(int[] nums, int k) {
        PriorityQueue<El> window = new PriorityQueue<>(Comparator.<El>naturalOrder().reversed());
        int[] result = new int[nums.length - k + 1];
        for (int i = 0; i < k; i++) {
            window.offer(new El(nums[i], i));
        }
        for (int r = k - 1, l = 0; r < nums.length; l++, r++) {
            window.offer(new El(nums[r], r));
            El max;
            while (window.peek().i < l) {
                window.poll();
            }
            max = window.peek();
            result[l] = max.val;
        }

        return result;
    }
}
