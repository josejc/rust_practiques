//use std::io;
//use std::env;

fn main() {
    //let first = env::args().nth(1).expect("please supply an argument");
    //let n: i32 = first.parse().expect("not an integer!");

    let mut f: f64 = 1.0;
    let mut n: f64 = 1.0;
    loop {
        f = f*n;
        n = n+1.0;
        if f.is_infinite() {
            break;
        }
        println!("f({}): {}",n,f);
    }
    println!("Iteracions fins arribar a infinit (f64): {}", n);
    // TODO: Show time in seconds to arrive to "inf"
}
