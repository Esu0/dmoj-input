#![allow(dead_code)]
use std::time::Instant;

fn dmojscan_usize() {
    for _ in 0..(1 << 20) {
        std::hint::black_box({
            let a = dmoj::scan!(usize);
            a
        });
    }
}

fn myscan_usize() {
    for _ in 0..(1 << 20) {
        std::hint::black_box({
            let a = myinput::scan!(usize);
            a
        });
    }
}

fn proconio_input_usize() {
    for _ in 0..(1 << 20) {
        std::hint::black_box({
            proconio::input! {
                a: usize,
            };
            a
        });
    }
}
fn main() {
    let start = Instant::now();
    // dmojscan_usize();
    // myscan_usize();
    // proconio_input_usize();
    println!("{:?}", start.elapsed());
}
