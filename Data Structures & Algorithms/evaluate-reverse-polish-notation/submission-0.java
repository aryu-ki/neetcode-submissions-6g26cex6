class Solution {
    public int evalRPN(String[] tokens) {
        String[] stack = new String[tokens.length];
        int head = -1;

        for (int i = 0; i < tokens.length; i++) {
            if (!"+-*/".contains(tokens[i])) {
                stack[++head] = tokens[i];
            } else {
                int second = Integer.parseInt(stack[head]);
                stack[head--] = null;
                int first = Integer.parseInt(stack[head]);
                stack[head--] = null;

                switch (tokens[i]) {
                    case "+" -> stack[++head] = "" + (first + second);
                    case "-" -> stack[++head] = "" + (first - second);
                    case "*" -> stack[++head] = "" + (first * second);
                    case "/" -> stack[++head] = "" + (first / second);
                }
            }
        }

        return Integer.parseInt(stack[head]);
    }
}
