class Solution {
    public boolean hasDuplicate(int[] nums) {
        var r = new HashSet<Integer>();
        for (int num : nums) {
            r.add(num);
        }
        return r.size() != nums.length;
    }
}