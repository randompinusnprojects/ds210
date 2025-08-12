use std::time::SystemTime;

fn main() {
    let mut k: u32;
    let mut f: u128;
    for k in 1..50 { // (b)
        let before = SystemTime::now();
        f = fib(k);
        let after = SystemTime::now();
        let difference = after.duration_since(before);
        let difference = difference.expect("Did the clock go back?");
        println!("k: {}, fib: {}, Time it took: {:?}", k, f, difference);
    }

}

fn fib(k: u32) -> u128 { // (a)
    if k == 0 {
        return 0;
    }
    if k == 1{
        return 1;
    }
    else {
        return fib(k - 1) + fib (k - 2);
    }

}