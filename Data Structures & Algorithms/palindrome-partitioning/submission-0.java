class Solution {
    public List<List<String>> partition(String s) {
        List<List<String>> result = new ArrayList<>();
        
        palindromes(s, 0, new ArrayList<>(), result);

        return result;
    }

    private void palindromes(String s, int start, List<String> partitions, List<List<String>> result) {
        if (start >= s.length()) {
            result.add(new ArrayList<>(partitions));
            return;
        }
        
        for (int i = start; i < s.length(); i++) {
            if (isPalindrome(s.substring(start, i+1))) {
                partitions.add(s.substring(start, i+1));
                palindromes(s, i+1, partitions, result);
                partitions.remove(s.substring(start, i+1));
            }
        }
    }

    private boolean isPalindrome(String s) {
        int l, r;
        l = (s.length() - 1) / 2;
        if (s.length() % 2 == 0) {
            r = l + 1;
        } else {
            r = l;
        }

        while (l >= 0) {
            if (s.charAt(l) != s.charAt(r)) {
                return false;
            }
            l--;
            r++;
        }

        return true;
    }
}
