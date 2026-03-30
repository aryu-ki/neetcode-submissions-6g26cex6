class Solution {
    private List<List<String>> result = new ArrayList<>();

    private record Cell(int col, int row) {} 

    public List<List<String>> solveNQueens(int n) {
        solve(new ArrayList<>(), n);
        return result;
    }

    private void solve(List<Cell> occupied, int n) {
        int col = occupied.size();
        if (col == n) {
            result.add(serialize(occupied, n));
            return;
        }
        row: for (int row = 0; row < n; row++) {
            if (occupied.isEmpty()) {
                Cell newQueen = new Cell(col, row);
                occupied.add(newQueen);
                solve(occupied, n);
                occupied.remove(newQueen);
            } else {
                for (int i = 0; i < occupied.size(); i++) {
                    Cell queen = occupied.get(i);
                    if (Math.abs(queen.row - row) == Math.abs(queen.col - col) || queen.row == row) {
                        continue row;
                    }
                }
                Cell newQueen = new Cell(col, row);
                occupied.add(newQueen);
                solve(occupied, n);
                occupied.remove(newQueen);
            }
        }
    }

    private List<String> serialize(List<Cell> queens, int n) {
        return queens.stream()
            .sorted(Comparator.comparing(Cell::row))
            .map(queen -> {
                String res = "";
                for (int i = 0; i < n; i++) {  
                    res += i == queen.col ? "Q" : ".";
                }
                return res;
            })
            .toList();
    }
}
