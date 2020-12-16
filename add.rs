fn add(a: i32, b: i32) -> i32 {
    let mut b = b;
    let mut res = a;
    let mut carry;

    loop {
        carry = (res & b) << 1;
        res ^= b;
        b = carry;

       if carry == 0 { break; }
    }

    res
}

fn main() {
    println!("{}", add(2, 3));
}
