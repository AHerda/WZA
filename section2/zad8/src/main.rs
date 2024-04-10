use zad2::Polynomial;

fn main() {
    let f = Polynomial::new(vec![-1., -4., -3., 1., 1.]);
        let g = Polynomial::new(vec![-1., -1., 1., 1.]);
        let nwd = f.gcd(&g).unwrap();

        let (a, b) = diophantine(f.clone(), g.clone(),nwd.clone()).unwrap();
        println!("a: {}", a);
        println!("b: {}", b);
        println!("nwd: {}", nwd);
}

pub fn diophantine(mut a: Polynomial, mut b: Polynomial, c: Polynomial) -> Option<(Polynomial, Polynomial)> {
    let mut x1 = Polynomial::new(vec![1.]);
    let mut y1 = Polynomial::new(vec![0.]);
    let mut x2 = Polynomial::new(vec![0.]);
    let mut y2 = Polynomial::new(vec![1.]);

    while !b.is_zero() {
        let (q, r) = (a / b.clone())?;

        let temp = x1 - q.clone() * x2.clone();
        x1 = x2;
        x2 = temp;

        let temp = y1 - q * y2.clone();
        y1 = y2;
        y2 = temp;

        a = b;
        b = r;
    }

    if !(c.clone() % a.clone())?.is_zero() {
        None
    } else {
        let x = x1 * (c.clone() / a.clone())?.0;
        let y = y1 * (c / a)?.0;
        Some((x, y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diophantine() {
        let f = Polynomial::new(vec![-1., -4., -3., 1., 1.]);
        let g = Polynomial::new(vec![-1., -1., 1., 1.]);
        let nwd = f.gcd(&g).unwrap();

        let (a, b) = diophantine(f.clone(), g.clone(),nwd.clone()).unwrap();
        assert_eq!(nwd, a * f + b * g);
    }
}