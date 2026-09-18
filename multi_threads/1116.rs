use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct State {
    next: i32,       // 接下来要输出的 1..=n
    zero_turn: bool, // true 表示该打印 0
}

struct ZeroEvenOdd {
    n: i32,
    state: Mutex<State>,
    cv: Condvar,
}

impl ZeroEvenOdd {
    fn new(n: i32) -> Self {
        ZeroEvenOdd {
            n,
            state: Mutex::new(State {
                next: 1,
                zero_turn: true,
            }),
            cv: Condvar::new(),
        }
    }

    fn zero<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        for _ in 0..self.n {
            let mut st = self.state.lock().unwrap();
            while !st.zero_turn {
                st = self.cv.wait(st).unwrap();
            }
            print_number(0);
            st.zero_turn = false;
            self.cv.notify_all();
        }
    }

    fn even<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        for _ in 0..self.n / 2 {
            let mut st = self.state.lock().unwrap();
            while st.zero_turn || st.next % 2 == 1 {
                st = self.cv.wait(st).unwrap();
            }
            print_number(st.next);
            st.next += 1;
            st.zero_turn = true;
            self.cv.notify_all();
        }
    }

    fn odd<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        for _ in 0..(self.n + 1) / 2 {
            let mut st = self.state.lock().unwrap();
            while st.zero_turn || st.next % 2 == 0 {
                st = self.cv.wait(st).unwrap();
            }
            print_number(st.next);
            st.next += 1;
            st.zero_turn = true;
            self.cv.notify_all();
        }
    }
}

fn main() {
    let n = 5;
    let obj = Arc::new(ZeroEvenOdd::new(n));
    let print_number = |x: i32| print!("{}", x);

    let t_zero = {
        let obj = Arc::clone(&obj);
        thread::spawn(move || obj.zero(print_number))
    };
    let t_odd = {
        let obj = Arc::clone(&obj);
        thread::spawn(move || obj.odd(print_number))
    };
    let t_even = {
        let obj = Arc::clone(&obj);
        thread::spawn(move || obj.even(print_number))
    };

    t_zero.join().unwrap();
    t_odd.join().unwrap();
    t_even.join().unwrap();
    println!();
}
