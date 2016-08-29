extern crate rand;

#[allow(dead_code)]
fn example_simple() {
    use rand::Rng;

    let mut rng = rand::thread_rng();
    if rng.gen() {
        // random bool
        println!("i32: {}, u32: {}", rng.gen::<i32>(), rng.gen::<u32>())
    }

    let tuple = rand::random::<(f64, char)>();
    println!("{:?}", tuple)
}

#[allow(dead_code)]
fn example_monte_carlo() {
    use rand::distributions::{IndependentSample, Range};

    let between = Range::new(-1f64, 1.);
    let mut rng = rand::thread_rng();

    let total = 1_000_000;
    let mut in_circle = 0;

    for _ in 0..total {
        let a = between.ind_sample(&mut rng);
        let b = between.ind_sample(&mut rng);
        if a * a + b * b <= 1. {
            in_circle += 1;
        }
    }

    // prints something close to 3.14159...
    println!("{}", 4. * (in_circle as f64) / (total as f64));
}

use rand::Rng;
use rand::distributions::{IndependentSample, Range};

fn dice<R: Rng>(faces: &Range<u32>, rng: &mut R) -> u32 {
    faces.ind_sample(rng)
}


fn main() {
    let mut rng = rand::thread_rng();
    let faces = Range::new(1, 6);
    for i in 1..10 {
        println!("{} = {:?}", i, dice(&faces, &mut rng));
    }
    // example_simple();
    // example_monte_carlo();
}
