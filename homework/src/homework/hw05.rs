fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let a = 56;
    let b = 98;

    let result = gcd(a, b);

    println!("GCD of {} and {} is {}", a, b, result);
}
