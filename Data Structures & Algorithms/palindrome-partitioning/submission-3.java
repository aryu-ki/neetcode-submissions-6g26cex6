class Solution {
    public List<List<String>> partition(String s) {
        List<List<String>> result = new ArrayList<>();
        
        palindromes(s, 0, 0, new ArrayList<>(), result);

        return result;
    }

    private void palindromes(String s, int i, int j, List<String> partitions, List<List<String>> result) {
        if (j >= s.length()) {
            if (i == j) {
                result.add(new ArrayList<>(partitions));
            }
            return;
        }
        
        if (isPalindrome(s.substring(i, j+1))) {
            partitions.add(s.substring(i, j+1));
            palindromes(s, j+1, j+1, partitions, result);
            partitions.remove(s.substring(i, j+1));
        }

        palindromes(s, i, j+1, partitions, result);
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
