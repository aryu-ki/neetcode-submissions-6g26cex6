class Solution {
    public boolean exist(char[][] board, String word) {
        // search first letter
        // start dfs/bfs search from each one
        for (int i = 0; i < board.length; i++) {
            for (int j = 0; j < board[0].length; j++) {
                if (board[i][j] == word.charAt(0)) {
                    if (beginSearch(board, word, i, j)) {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    private boolean beginSearch(char[][] board, String word, int i, int j) {
        return search(board, word, "", i, j, new HashSet<>());
    }

    private boolean search(char[][] board, String word, String current, int i, int j, Set<String> visited) {
        if (visited.contains(i + " " + j) || i >= board.length || i < 0 || j >= board[i].length || j < 0) {
            return false;
        }
        current += board[i][j];

        if (!word.startsWith(current)) {
            return false;
        }
        
        if (current.length() == word.length()) {
            return true;
        }

        visited.add(i + " " + j);

        boolean result = search(board, word, current, i+1, j, visited) || search(board, word, current, i, j+1, visited) || 
        search(board, word, current, i-1, j, visited) || search(board, word, current, i, j-1, visited);

        visited.remove(i + " " + j);
        return result;
    }
}
