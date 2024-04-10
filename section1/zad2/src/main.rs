pub mod lib;

use lib::Polynomial;

fn main() {
    let a = Polynomial::new(vec![1., 1.]);
    let b = Polynomial::new(vec![1., 2., 1.]);
    let gcd = a.gcd(&b).unwrap();
    let lcm = a.lcm(&b).unwrap();

    println!("({}, {}) = (c) <==> c = {}", a, b, gcd);
    println!("({}) ∩ ({}) = (c) <==> d = {}", a, b, lcm);
}
