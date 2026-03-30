class Solution {
    public boolean checkInclusion(String s1, String s2) {
        if (s1.length() > s2.length()) {
            return false;
        }

        int l = 0, r = s1.length()-1;
        HashMap<Character, Integer> charCount = calcCharCount(s1);
        for (int i = 0; i <= r; i++) {
            char addedChar = s2.charAt(i);
            int added = charCount.compute(addedChar, (k,v) -> v == null ? -1 : v - 1);
            if (added == 0) {
                charCount.remove(addedChar);
            }
        }
        while (r < s2.length() - 1) {
            if (charCount.isEmpty()) {
                return true;
            }
            r++;
            char addedChar = s2.charAt(r);
            char removedChar = s2.charAt(l);
            if (charCount.compute(addedChar, (k,v) -> v == null ? -1 : v - 1) == 0) {
                charCount.remove(addedChar);  
            }
            if (charCount.compute(removedChar, (k,v) -> v == null ? 1 : v + 1) == 0) {
                charCount.remove(removedChar);
            }
            l++; 
        } 
        return charCount.isEmpty();
    }

    private HashMap<Character, Integer> calcCharCount(String s) {
        final HashMap<Character, Integer> result = new HashMap<>(s.length());
        for (int i = 0; i < s.length(); i++) {
            result.compute(s.charAt(i), (k,v) -> v == null ? 1 : v + 1);
        }
        return result;
    }
}
