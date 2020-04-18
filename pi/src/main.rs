use rand::distributions::{Distribution, Uniform};

fn main() {
    let range = Uniform::new(0.0, 1.0);
    let mut rng = rand::thread_rng();

    let total = 1_000_000;
    let mut in_circle = 0;

    for _ in 0..total {
        let x = range.sample(&mut rng);
        let y = range.sample(&mut rng);
	// funtion circle (x-1/2)²+(y-1/2)²=(1/2)² - center 1/2,1/2 amb radi 1/2
	// A_cercle = pi * r² -> pi * 1/4 => pi = A_cercle*4
	// Dins rectangle x [0,1] i y [0,1] amb àrea A_rect = 1
	// A_cercle/A_rect = pi/4 => pi = 4*A_cercle/A_rect
        //if ((x * x) - x + (y * y) -y) <= -1.0/4.0 {
        //    in_circle += 1;
        //}
	// Simplification x²+y²<=1 circle of center (0,0) amb r=1 -> A_circle = π 
	// Only calculate points in a quarter of circle -> A_cicle = π/4
	// A_rectangle = 1
	if (x*x)+(y*y) <= 1.0 {
		in_circle += 1;
	}
    }

    // prints something close to 3.14159...
    println!(
        "With N: {}, π is approximately {}", total,
        4. * (in_circle as f64) / (total as f64)
    );
}
