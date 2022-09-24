// This problem was asked by Google.
//
// The area of a circle is defined as πr^2. Estimate π to 
// 3 decimal places using a Monte Carlo method.
//
// Hint: The basic equation of a circle is x2 + y2 = r2.

use rand::distributions::{Distribution, Uniform};

fn main() {
    estimate_pi(1000);
}

fn estimate_pi(interval: i64) -> f64 {
    let mut circle_pts = 0;
    let mut square_pts = 0;

    let between = Uniform::from(-1.0..=1.0);
    let mut rng = rand::thread_rng();

    for _ in 0..interval*interval {
        // generate random x value
        let x = between.sample(&mut rng);

        // generate random y value
        let y = between.sample(&mut rng);

        // Calculate d = x*x + y*y
        let d = (x * x) + (y * y);

        // If d <= 1 increment circle_pts
        if d <= 1.0 {
            circle_pts += 1;
        }

        // Increment square_pts
        square_pts += 1
    }
    println!("circle_pts: {circle_pts}\nsquare_pts: {square_pts}");

    // Calculate π = 4 * ( circle_pts / square_pts )
    let pi_estimate = 4.0 * ( circle_pts as f64 / square_pts as f64 );
    println!("π estimate: {pi_estimate}");
    // Return π
    pi_estimate
}
