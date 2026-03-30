class Solution {
    public int characterReplacement(String s, int k) {
        if (s.length() < 2) {
            return s.length();
        }
        int left = 0, right = 0;

        int result = 1;
        Map<Character, Integer> charsCount = new HashMap<>();
        charsCount.put(s.charAt(0), 1);
        int max = 1;
        while (right < s.length() - 1) {
            max = Math.max(max, charsCount.compute(s.charAt(right+1), (key,v) -> v == null ? 1 : v + 1));
            if ((right+1 - left) + 1 - max <= k) {
                result = right+2 - left;
            } else {
                charsCount.computeIfPresent(s.charAt(left), (key,v) -> v-1);
                max = maxCount(charsCount);
                left++;
            }
            right++;
        }
        return result;
    }

    private Map<Character, Integer> charsInString(String s) {
        Map<Character, Integer> result = new HashMap<>();
        for (int i = 0; i < s.length(); i++) {
            result.compute(s.charAt(i), (k, v) -> v == null ? 1 : v+1);
        }
        return result;
    }

    private int maxCount(Map<Character, Integer> chars) {
        return chars.values()
            .stream()
            .mapToInt(v -> v)
            .max()
            .orElse(0);
    }
}
