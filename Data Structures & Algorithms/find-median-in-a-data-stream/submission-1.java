class MedianFinder {
    private final PriorityQueue<Integer> lo = new PriorityQueue<>(Comparator.<Integer>naturalOrder().reversed());
    private final PriorityQueue<Integer> hi = new PriorityQueue<>();
    int total = 0;

    public MedianFinder() {
        
    }
    
    public void addNum(int num) {
        if (total < 2) {
            if (lo.isEmpty()) {
                lo.offer(num);
            } else if (lo.peek().compareTo(num) > 0) {
                hi.offer(lo.poll());
                lo.offer(num);
            } else {
                hi.offer(num);
            }
        } else {
            if (hi.peek().compareTo(num) > 0) {
                if (lo.size() > hi.size()) {
                    balanceLeft();
                }
                lo.offer(num);
            } else {
                if (hi.size() > lo.size()) {
                    balanceRight();
                }
                hi.offer(num);
            }
        }
        total++;
    }
    
    public double findMedian() {
        if (lo.size() == hi.size()) {
            return (double) (lo.peek() + hi.peek()) / 2.0;
        }
        return lo.size() > hi.size() ? lo.peek() : hi.peek();
    }

    private void balanceRight() {
        lo.offer(hi.poll());
    }

    private void balanceLeft() {
        hi.offer(lo.poll());
    }
}
