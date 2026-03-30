class Solution {
    private record CharCount(char c, int count) {

    }

    public int characterReplacement(String s, int k) {
        if (s.length() < 2) {
            return s.length();
        }
        int left = 0, right = 0;

        int result = 1;
        while (right < s.length() - 1) {
            var candidate = charsInString(s.substring(left, right+2));
            int max = maxCount(candidate);
            if ((right+1 - left) + 1 - max <= k) {
                result = right+2 - left;
            } else {
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
