use rand::{thread_rng, Rng};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "pi-gen", about = "a numerical approximation of pi")]
struct Config {
    #[structopt(short = "p", long = "points", default_value = "100000")]
    points: i32
}

fn main() {
    let args = Config::from_args();
    let points = args.points;
    estimate_pi(points)
}

fn estimate_pi(points: i32) {
    // generate a random number in range [0;1)
    let rand = || thread_rng().gen();

    let mut points_circle = 0;
    
    for _i in 0..points {
        let x: f64 = rand();
        let y: f64 = rand();

        // check if the point is inside the circle and counts them (only works with coordinates in range [0;1))
        if x * x + y * y < 1.0 {
            points_circle += 1;
        }
    }

    // PI is 4 times the ratio of the points in the circle and all the points
    let pi = 4.0 * points_circle as f64 / points as f64;

    println!("PI is approximately {}", pi);
    println!("There have been {} out of {} Points inside a quarter of the unit circle", points_circle, points)
}
