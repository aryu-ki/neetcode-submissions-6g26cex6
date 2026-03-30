class Solution {
    public List<List<Integer>> subsets(int[] nums) {
        List<List<Integer>> result = new ArrayList<>();

        for (int i = 0; i < nums.length; i++) {
            result.addAll(combine(nums, List.of(nums[i]), i));
        }

        result.add(Collections.emptyList());

        return result;
    }

    private List<List<Integer>> combine(int[] nums, List<Integer> current, int start) {
        List<List<Integer>> result = new ArrayList<>();
        result.add(current);
        for (int i = start+1; i < nums.length; i++) {
            var newCombo = new ArrayList<>(current);
            newCombo.add(nums[i]);
            result.addAll(combine(nums, newCombo, i));
        }
        return result;
    }
}
