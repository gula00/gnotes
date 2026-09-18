// https://leetcode.cn/problems/print-in-order/

use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct Foo {
    // 0: 初始
    // 1: first 已完成
    // 2: second 已完成
    state: Mutex<i32>,
    cv: Condvar,
}

impl Foo {
    fn new() -> Self {
        Self {
            state: Mutex::new(0),
            cv: Condvar::new(),
        }
    }

    fn first(&self, print_first: impl FnOnce()) {
        print_first();
        let mut state = self.state.lock().unwrap();
        *state = 1;
        self.cv.notify_all();
    }

    fn second(&self, print_second: impl FnOnce()) {
        let mut state = self.state.lock().unwrap();
        while *state < 1 {
            state = self.cv.wait(state).unwrap();
        }
        print_second();
        *state = 2;
        self.cv.notify_all();
    }

    fn third(&self, print_third: impl FnOnce()) {
        let mut state = self.state.lock().unwrap();
        while *state < 2 {
            state = self.cv.wait(state).unwrap();
        }
        print_third();
    }
}

fn run(nums: [i32; 3]) -> String {
    let foo = Arc::new(Foo::new());
    let output = Arc::new(Mutex::new(String::new()));
    let mut handles = Vec::new();

    // nums 只决定哪个线程调用哪个方法，不保证调度顺序
    for n in nums {
        let foo = Arc::clone(&foo);
        let output = Arc::clone(&output);
        handles.push(thread::spawn(move || match n {
            1 => foo.first(|| output.lock().unwrap().push_str("first")),
            2 => foo.second(|| output.lock().unwrap().push_str("second")),
            3 => foo.third(|| output.lock().unwrap().push_str("third")),
            _ => unreachable!(),
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    output.lock().unwrap().clone()
}

fn main() {
    for nums in [
        [1, 2, 3],
        [1, 3, 2],
        [2, 1, 3],
        [2, 3, 1],
        [3, 1, 2],
        [3, 2, 1],
    ] {
        let out = run(nums);
        println!("{nums:?} -> {out}");
        assert_eq!(out, "firstsecondthird");
    }
}
