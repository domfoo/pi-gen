use rand::{thread_rng, Rng};

fn main() {
    // generate a random number in range [0;1)
    let rand = || thread_rng().gen();

    let mut n1 = 0;
    let n2 = 10000000;
    
    for _i in 0..n2 {
        let x: f64 = rand();
        let y: f64 = rand();

        // check if the point is inside the circle (only works with coordinates in range [0;1))
        if x * x + y * y < 1.0 {
            n1 += 1;
        }
    }

    // PI is 4 times the ratio of the points in the circle and all the points
    let pi = 4.0 * n1 as f64 / n2 as f64;

    println!("PI is approximately {}", pi);
}
