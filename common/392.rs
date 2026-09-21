impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let n = t.len();
        let mut nxt = vec![[n; 26]; n + 1];
        for (i, c) in t.bytes().enumerate().rev() {
            nxt[i] = nxt[i + 1];
            nxt[i][(c - b'a') as usize] = i;
        }
        s.bytes()
            .try_fold(0, |i, c| {
                let j = nxt[i][(c - b'a') as usize];
                (j != n).then_some(j + 1)
            })
            .is_some()
    }
}
