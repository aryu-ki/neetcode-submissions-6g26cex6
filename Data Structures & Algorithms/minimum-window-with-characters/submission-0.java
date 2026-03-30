class Solution {
    public String minWindow(String s, String t) {
        if (t.length() > s.length()) {
            return "";
        }

        var tCharCount = calcCharCount(t);
        HashMap<Character, Integer> window = new HashMap<>();
        int l = 0;
        boolean expand = true;
        int[] result = {-1, -1};
        int resLen = Integer.MAX_VALUE;
        int have = 0, need = tCharCount.size();
        for (int r = 0; r < s.length(); r++) {
            var rc = s.charAt(r);
            if (tCharCount.containsKey(rc) && window.compute(rc, (k,v) -> v == null ? 1 : v + 1) == tCharCount.get(rc)) {
                have++;
            }
            while (have == need) {
                if (r - l + 1 < resLen) {
                    result[0] = l;
                    result[1] = r;
                    resLen = r - l + 1;
                }
                var lc = s.charAt(l);
                if (tCharCount.containsKey(lc) && window.computeIfPresent(lc, (k,v) -> v - 1) < tCharCount.get(lc)) {
                    have--;
                }
                l++;
            }
        }

        return resLen == Integer.MAX_VALUE ? "" : s.substring(result[0], result[1] + 1);
    }

    private HashMap<Character, Integer> calcCharCount(String s) {
        final HashMap<Character, Integer> result = new HashMap<>(s.length());
        for (int i = 0; i < s.length(); i++) {
            result.compute(s.charAt(i), (k,v) -> v == null ? 1 : v + 1);
        }
        return result;
    }
}
