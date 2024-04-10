fn main() {
    let subset = Subset::new(
        2,
        |v1, v2| v1.iter().zip(v2).all(|(a, b)| a <= b),
        |value| value >= 11,
        |v| v.iter().product::<u32>() as i32,
    );

    let start_min = vec![1, 1];
    let max = vec![50, 50];
    let (number_of_minimal_points, minimal_points) = subset.find_minmal_points(&start_min, &max);
    println!("Number of minimal points: {}", number_of_minimal_points);
    for point in minimal_points {
        println!("{:?}", point);
    }

    let subset = Subset::new(
        2,
        |v1, v2| v1.iter().zip(v2).all(|(a, b)| a <= b),
        |value| value <= 25,
        |v| (v[0] as i32 - 10).pow(2) + (v[1] as i32 - 10).pow(2),
    );

    let start_min = vec![0, 0];
    let max = vec![50, 50];
    let (number_of_minimal_points, minimal_points) = subset.find_minmal_points(&start_min, &max);
    println!("Number of minimal points: {}", number_of_minimal_points);
    for point in minimal_points {
        println!("{:?}", point);
    }
}

struct Subset<F, G, H>
where
    F: Fn(i32) -> bool,
    G: Fn(&[u32]) -> i32,
    H: Fn(&[u32], &[u32]) -> bool,
{
    dimension: usize,
    order_function: H,
    border_function: F,
    value_function: G,
}

impl<F, G, H> Subset<F, G, H>
where
    F: Fn(i32) -> bool,
    G: Fn(&[u32]) -> i32,
    H: Fn(&[u32], &[u32]) -> bool,
{
    fn new(dimension: usize, order_function: H, border_function: F, value_function: G) -> Self {
        Self {
            dimension,
            order_function,
            border_function,
            value_function,
        }
    }

    fn order_function(&self, v1: &[u32], v2: &[u32]) -> bool {
        if v1.len() != self.dimension || v2.len() != self.dimension {
            panic!("Dimension mismatch");
        }
        (self.order_function)(v1, v2)
    }

    fn border_function(&self, value: i32) -> bool {
        (self.border_function)(value)
    }

    fn value_function(&self, v: &[u32]) -> i32 {
        if v.len() != self.dimension {
            panic!("Dimension mismatch");
        }
        (self.value_function)(v)
    }

    fn is_in_subset(&self, v: &[u32]) -> bool {
        if v.len() != self.dimension {
            panic!("Dimension mismatch");
        }
        self.border_function(self.value_function(v))
    }

    fn find_minmal_points(&self, start_min: &[u32], max: &[u32]) -> (u32, Vec<Vec<u32>>) {
        if start_min.len() != self.dimension || max.len() != self.dimension{
            panic!("Dimension mismatch");
        }

        let mut results: Vec<Vec<u32>> = Vec::new();
        let mut all_points: Vec<Vec<u32>> = Vec::new();

        let mut min_value = (*start_min).to_vec();
        for _ in 0..start_min.iter().zip(max.iter()).fold(1, |acc, (a, b)| acc * (b - a + 1)) {
            all_points.push(min_value.clone());
            for (i, p) in min_value.iter_mut().enumerate() {
                *p += 1;
                if *p < max[i] {
                    break;
                } else {
                    *p = start_min[i];
                }
            }
        }

        // for point in all_points.iter() {
        //     println!("{:?}", point);
        // }

        for point in all_points {
            if !self.contains_lower(&results, &point) && self.is_in_subset(&point) {
                if self.contains_higher(&results, &point) {
                    results.retain(|p| !self.order_function(p, &point));
                }
                results.push(point.clone());
            }
        }

        (results.len() as u32, results)
    }

    /* fn find_minmal_points(&self, start_min: &[u32], max: &[u32]) -> (u32, Vec<Vec<u32>>) {
        if start_min.len() != self.dimension || max.len() != self.dimension{
            panic!("Dimension mismatch");
        }

        let mut results: Vec<Vec<u32>> = Vec::new();
        let mut min_value = (*start_min).to_vec();
        let mut old_min_value = start_min;
        let mut i = 0_usize;
        loop {
            println!("{:?}", min_value);
            if min_value.iter().zip(max).any(|(a, b)| a >= b) {
                i += 1;
                if i == self.dimension {
                    break;
                }
                Self::reset(&mut min_value[..i], &start_min[..i]);
                min_value[i] += 1;
                i = 0;
            }

            if !self.contains_lower(&results, &min_value) && self.is_in_subset(&min_value) {
                println!("Znaleziono punkt: {:?}", min_value);
                println!("\ti = {}", i);
                results.push(min_value.to_vec());

                i += 1;
                if i == self.dimension {
                    break;
                }
                Self::reset(&mut min_value[..i], &start_min[..i]);
                min_value[i] += 1;
                i = 0;
            } else if !self.contains_lower(&results, &min_value) && !self.is_in_subset(&min_value) {
                min_value[i] += 1;
            } else if self.contains_lower(&results, &min_value) && self.is_in_subset(&min_value) {
                println!("\ti = {}", i);
                i += 1;
                if i == self.dimension {
                    break;
                }
                old_min_value = &min_value;
                Self::reset(&mut min_value[..i], &start_min[..i]);

                min_value[i] += 1;
                i = 0;
            } else {
                panic!("Nieoczekiwany przypadek");
            }
        }

        (results.len() as u32, results)
    } */

    fn reset(v: &mut [u32], start_min: &[u32]) {
        for (i, point) in v.iter_mut().enumerate() {
            *point = start_min[i];
        }
    }

    fn contains_lower(&self, set: &[Vec<u32>], v: &[u32]) -> bool {
        set.iter().any(|point| {
            self.order_function(point, v)
        })
    }

    fn contains_higher(&self, set: &[Vec<u32>], v: &[u32]) -> bool {
        set.iter().any(|point| {
            !self.order_function(point, v)
        })
    }
}
