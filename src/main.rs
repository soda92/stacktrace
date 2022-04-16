use backtrace::Backtrace;

fn b() {
    let bt = Backtrace::new();

    // do_some_work();

    println!("{:?}", bt);
}

fn c() {
    b();
}

fn main(){
    c();
}
