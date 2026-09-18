// https://leetcode.cn/problems/print-foobar-alternately/

use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct FooBar {
    n: usize,
    /// false = foo 的回合，true = bar 的回合
    turn: Mutex<bool>,
    cv: Condvar,
}

impl FooBar {
    fn new(n: usize) -> Self {
        Self {
            n,
            turn: Mutex::new(false),
            cv: Condvar::new(),
        }
    }

    fn foo(&self) {
        for _ in 0..self.n {
            let mut turn = self.turn.lock().unwrap();
            while *turn {
                turn = self.cv.wait(turn).unwrap();
            }
            print!("foo");
            *turn = true;
            self.cv.notify_one();
        }
    }

    fn bar(&self) {
        for _ in 0..self.n {
            let mut turn = self.turn.lock().unwrap();
            while !*turn {
                turn = self.cv.wait(turn).unwrap();
            }
            print!("bar");
            *turn = false;
            self.cv.notify_one();
        }
    }
}

fn main() {
    let n = 2;
    let fb = Arc::new(FooBar::new(n));

    let a = {
        let fb = Arc::clone(&fb);
        thread::spawn(move || fb.foo())
    };
    let b = {
        let fb = Arc::clone(&fb);
        thread::spawn(move || fb.bar())
    };

    a.join().unwrap();
    b.join().unwrap();
    println!();
}
