extern "C" {
    fn doDangerousLowLevelWork();
}

fn do_a_bit_less_dangerous_work() {
    unsafe {
        doDangerousLowLevelWork();
    }
}

fn main() {
    println!("we're in Rust land and about to call C");
    do_a_bit_less_dangerous_work();
    println!("we're back in Rust land, bye!");
}
