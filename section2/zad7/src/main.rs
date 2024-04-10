use zad2::Polynomial;


fn main() {
    let p1 = Polynomial::new(vec![1., 0., 1., 0., 1.]);
    let p2 = Polynomial::new(vec![-1., -2., -1., 0., 1.]);
    let p3 = Polynomial::new(vec![-1., 0., 0., 1.]);
    println!("GCD v1: {}", gcd_all(&vec![p1, p2, p3]).unwrap());
    let p4 = Polynomial::new(vec![-4., -4., 1., 1.]);
    let p5 = Polynomial::new(vec![4., -4., -1., 1.]);
    let p6 = Polynomial::new(vec![2., -1., -2., 1.]);
    println!("GCD v2: {}", gcd_all(&vec![p4, p5, p6]).unwrap());
}


fn gcd_all(v: &[Polynomial]) -> Option<Polynomial> {
    Some(v.iter().fold( Polynomial::new(vec![0.0]), |acc, x| acc.gcd(x).unwrap()))
}
