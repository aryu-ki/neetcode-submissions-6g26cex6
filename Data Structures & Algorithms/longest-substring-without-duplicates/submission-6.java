 class Solution {
        public int lengthOfLongestSubstring(String s) {
            Set<Character> met = new HashSet<>(s.length());
            int left = 0, right = 0;
            boolean growing = true;
            char duplicate = (char) 0;
            int longest = 0;
            while (right < s.length()) {
                if (growing) {
                    if (met.add(s.charAt(right))) {
                        longest = Math.max(longest, right - left + 1);
                        right++;
                    } else {
                        growing = false;
                        duplicate = s.charAt(right);
                    }
                } else {
                    char removed = s.charAt(left);
                    met.remove(s.charAt(left));
                    left++;
                    if (removed == duplicate) {
                        growing = true;
                    }
                }
            }
            return longest;
        }
}