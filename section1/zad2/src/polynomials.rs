use std::vec;

#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial {
    coefficients: Vec<f64>,
}

impl std::ops::Add for Polynomial {
    type Output = Polynomial;

    fn add(self, other: Polynomial) -> Self::Output {
        let mut coefficients = Vec::new();
        let max_len = self.coefficients.len().max(other.coefficients.len());

        for i in 0..max_len {
            let a = self.coefficients.get(i).unwrap_or(&0.0);
            let b = other.coefficients.get(i).unwrap_or(&0.0);
            coefficients.push(a + b);
        }

        Self::Output {
            coefficients: Self::clean_vec(&coefficients),
        }
    }
}

impl std::ops::Sub for Polynomial {
    type Output = Polynomial;

    fn sub(self, other: Polynomial) -> Self::Output {
        let mut coefficients = Vec::new();
        let max_len = self.coefficients.len().max(other.coefficients.len());

        for i in 0..max_len {
            let a = self.coefficients.get(i).unwrap_or(&0.0);
            let b = other.coefficients.get(i).unwrap_or(&0.0);
            coefficients.push(a - b);
        }

        Self::Output {
            coefficients: Self::clean_vec(&coefficients),
        }
    }
}

impl std::ops::Mul for Polynomial {
    type Output = Polynomial;

    fn mul(self, other: Polynomial) -> Self::Output {
        let mut coefficients = vec![0.0; self.coefficients.len() + other.coefficients.len() - 1];

        for (i, a) in self.coefficients.iter().enumerate() {
            for (j, b) in other.coefficients.iter().enumerate() {
                coefficients[i + j] += a * b;
            }
        }

        Self::Output {
            coefficients: Self::clean_vec(&coefficients),
        }
    }
}

impl std::ops::Div<f64> for Polynomial {
    type Output = Polynomial;

    fn div(self, other: f64) -> Self::Output {
        Self::Output {
            coefficients: Polynomial::clean_vec(
                &self.coefficients.iter().map(|x| x / other).collect(),
            ),
        }
    }
}

impl std::ops::Div for Polynomial {
    type Output = Option<(Polynomial, Polynomial)>;

    fn div(self, other: Polynomial) -> Self::Output {
        if other.coefficients.is_empty()
            || (other.coefficients.len() == 1 && other.coefficients[0] == 0.0)
        {
            return None;
        }
        if other.coefficients.len() == 1 {
            return Some((
                self / other.coefficients[0],
                Polynomial {
                    coefficients: vec![0.0],
                },
            ));
        }

        let mut quotient = Polynomial::new(vec![0.0]);
        let mut reminder = self.clone();
        let temp = other.clone();
        let c = other.coefficients.last()?;

        while reminder.degree() >= temp.degree() {
            let mut s = vec![0.0; (reminder.degree() - temp.degree()) as usize];
            s.push(reminder.coefficients.last()? / c);
            let t = Polynomial::new(s);
            quotient = quotient + t.clone();
            reminder = reminder - (t * temp.clone());
        }
        Some((quotient, reminder))
    }
}

impl std::ops::Rem for Polynomial {
    type Output = Option<Polynomial>;

    fn rem(self, other: Polynomial) -> Self::Output {
        Some((self / other)?.1)
    }
}

impl std::fmt::Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut terms = Vec::new();
        for (i, coeff) in self.coefficients.iter().enumerate() {
if *coeff != 0.0 {
	let term = match i {
		0 => format!("{}", coeff.abs()),
		1 => format!("{} {}x", if coeff < &0.0 { "-" } else { "+" }, coeff.abs()),
		_ => format!("{} {}x^{}", if coeff < &0.0 { "-" } else { "+" }, coeff.abs(), i),
	};
	terms.push(term);
}
        }
        let polynomial = terms.join(" ");
        write!(f, "{}", polynomial)
    }
}

impl Polynomial {
    pub fn new(coefficients: Vec<f64>) -> Self {
        Polynomial {
            coefficients: Self::clean_vec(&coefficients),
        }
    }

    pub fn is_zero(&self) -> bool {
        self.coefficients.iter().all(|&x| x == 0.0) || self.coefficients.is_empty()
    }

    pub fn degree(&self) -> usize {
        if self.is_zero() {
            return 0;
        }
        self.coefficients.len() - 1
    }

    pub fn clean(&mut self) {
        while Some(&0.0_f64) == self.coefficients.last() {
            self.coefficients.pop();
        }
    }

    pub fn clean_vec(vec: &Vec<f64>) -> Vec<f64> {
        let mut result = vec.clone();
        while Some(&0.0_f64) == result.last() {
            result.pop();
        }
        result
    }

    pub fn gcd(&self, other: &Polynomial) -> Option<Polynomial> {
        let mut a = self.clone();
        let mut b = other.clone();
        if a.coefficients.len() < b.coefficients.len() {
            std::mem::swap(&mut a, &mut b);
        }
        while !b.is_zero() {
            let r = (a.clone() % b.clone())?;
            a = b;
            b = r;
        }
        Some(a)
    }

    pub fn lcm(&self, other: &Polynomial) -> Option<Polynomial> {
        let gcd = self.gcd(&other)?;
		let temp = self.clone() * other.clone();
        Some((temp / gcd)?.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let p1 = Polynomial::new(vec![1.0, 2.0, 2.0]);
        let p2 = Polynomial::new(vec![2.0, 1.0]);
        let p3 = p1 + p2;
        assert_eq!(p3.coefficients, vec![3.0, 3.0, 2.0]);
    }

    #[test]
    fn test_sub() {
        let p1 = Polynomial::new(vec![3.0, 1.0, 1.0]);
        let p2 = Polynomial::new(vec![2.0, 2.0, 1.0]);
        let p3 = p1 - p2;
        assert_eq!(p3.coefficients, vec![1.0, -1.0]);
    }

    #[test]
    fn test_mul() {
        let p1 = Polynomial::new(vec![1.0, 2.0, 1.0]);
        let p2 = Polynomial::new(vec![2.0, 1.0]);
        let p3 = p1 * p2;
        assert_eq!(p3.coefficients, vec![2.0, 5.0, 4.0, 1.0]);
    }

    #[test]
    fn test_div() {
        let p1 = Polynomial::new(vec![10., 13., -17., -11., 31., 4., -30.]);
        let p2 = Polynomial::new(vec![5., 4., -3., 2., 10.]);
        let (quotient, reminder) = (p1.clone() / p2.clone()).unwrap();
        assert_eq!(quotient.coefficients, vec![2.0, 1.0, -3.0]);
        assert_eq!(reminder.coefficients, vec![]);
        assert_eq!(p1, p2 * quotient + reminder);
    }

    #[test]
    fn test_gcd() {
        let p1 = Polynomial::new(vec![6., 7., 1.]);
        let p2 = Polynomial::new(vec![-6., -5., 1.]);
        let p3 = p1.gcd(&p2).unwrap();
		let expected = Polynomial::new(vec![1., 1.]);
        assert_eq!(p3.degree(), expected.degree());
        assert_eq!((p1 / p3.clone()).unwrap().1.coefficients, vec![]);
        assert_eq!((p2 / p3).unwrap().1.coefficients, vec![]);
    }

    #[test]
    fn test_lcm() {
        let p1 = Polynomial::new(vec![6., 7., 1.]);
        let p2 = Polynomial::new(vec![-6., -5., 1.]);
        let p3 = p1.lcm(&p2).unwrap();
		let expected = Polynomial::new(vec![-36., -36., 1., 1.]);
        assert_eq!(p3.degree(), expected.degree());
        assert_eq!((p3.clone() / p1).unwrap().1.coefficients, vec![]);
        assert_eq!((p3 / p2).unwrap().1.coefficients, vec![]);
    }
}
