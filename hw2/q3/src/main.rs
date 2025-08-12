use std::io;

fn main() {
    let mut sum:u32 = 0;
    let mut cubed:u32 = 1; // for inefficient programming

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    let k:u8 = input.parse().expect("Not a good number!"); // ~255
    

    for i in 1..=k {
        for _c in 0..3 { // as stupid as possible using a nested for loop
            cubed = cubed * (i as u32)
        }
        sum = sum + cubed; // compute sum
        cubed = 1; // reinitialize cubed for next time
    }

    println!("{}", sum);
}
