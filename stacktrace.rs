#![feature(backtrace)]
use std::backtrace::Backtrace;

fn main() {
    let bt = Backtrace::capture();

    // do_some_work();

    println!("{:?}", bt);
}
