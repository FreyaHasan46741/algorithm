fn insert_sort(nums: &mut Vec<usize>) {
    for i in 1..nums.len() {
        let (mut p, v) = (i, nums[i]);
        while p > 0 && nums[p-1] > v {
            nums[p] = nums[p-1];
            p -= 1;
        }
        nums[p] = v;
    }
}

