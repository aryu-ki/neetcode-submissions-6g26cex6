class Solution {
    private Set<List<Integer>> result = new HashSet<>();

    public List<List<Integer>> subsetsWithDup(int[] nums) {
        result.add(new ArrayList<>());
        nums = Arrays.stream(nums).sorted().toArray();
        for (int i = 0; i < nums.length; i++) {
            combine(nums, i, new ArrayList<>());
        }
        return new ArrayList<>(result);
    }

    private void combine(int[] nums, int i, List<Integer> current) {
        if (i >= nums.length) {
            return;
        }
        current.add(nums[i]);
        result.add(new ArrayList<>(current));
        for (int s = i + 1; s < nums.length; s++) {
            combine(nums, s, current);
        }
        current.remove(current.size() - 1);
    }
}
