impl Solution {
    pub fn count_stable_subarrays(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let n = nums.len();
        let mut nxt: Vec<usize> = (0..n).collect();
        for i in (0..n - 1).rev() {
            if nums[i] <= nums[i + 1] {
                nxt[i] = nxt[i + 1];
            }
        }

        let mut prefix = vec![0i64; n + 1];
        let mut curr = 0i64;
        for i in 0..n {
            if i > 0 && nums[i - 1] > nums[i] {
                curr = 0;
            }
            curr += 1;
            prefix[i + 1] = prefix[i] + curr;
        }

        let mut ans = Vec::new();
        for q in queries {
            let l = q[0] as usize;
            let r = q[1] as usize;
            let m = nxt[l].min(r);
            let k = (m - l + 1) as i64;
            ans.push(k * (k + 1) / 2 + prefix[r + 1] - prefix[m + 1]);
        }
        ans
    }
}
