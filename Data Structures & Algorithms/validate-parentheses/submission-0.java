class Solution {
    private static Map<Character, Character> brackets = Map.of(
        '(', ')',
        '{', '}',
        '[', ']'
    );

    public boolean isValid(String s) {
        char[] stack = new char[s.length()];
        int ptr = -1;

        for (int i = 0; i < s.length(); i++) {
            if (brackets.containsKey(s.charAt(i))) {
                stack[++ptr] = s.charAt(i);
            } else if (ptr == -1 || !brackets.get(stack[ptr]).equals(s.charAt(i))) {
                return false;
            } else {
                stack[ptr--] = (char) 0;
            }
        }

        return ptr == -1;
    }
}
