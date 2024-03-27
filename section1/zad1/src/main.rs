mod gaussian_numbers;

use gaussian_numbers::GaussianNumber;

fn main() {
    let a = GaussianNumber::new(3, 4);
    let b = GaussianNumber::new(1, 3);
    let gcd = a.gcd(&b);
    let lcm = a.lcm(&b);

    println!("({}, {}) = (c) <==> c = {}", a, b, gcd);
    println!("({}) ∩ ({}) = (c) <==> d = {}", a, b, lcm);
}
