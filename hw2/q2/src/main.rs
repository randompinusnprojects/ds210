fn main() { // (a)
    let mut f: [u128;180] = [0;180];
    for i in 0..180 { 
        f[i] = fib(i, f);
    }
    println!("{:?}", f);
}

fn fib(k: usize, f: [u128;180]) -> u128 {
    if k == 0 {
        return 0;
    }
    if k == 1{
        return 1;
    }
    else {
        return f[k-1] + f[k-2];
    }

}