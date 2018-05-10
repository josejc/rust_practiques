//use std::io;
//use std::env;

fn main() {
    //let first = env::args().nth(1).expect("please supply an argument");
    //let n: i32 = first.parse().expect("not an integer!");

    let mut n = 3;
    let mut a: f64 = 1.0;
    let mut b: f64 = 1.0;
    let mut c: f64 = a + b;
    println!("{}",a);
    println!("{}",b);
    println!("{}",c);
    loop {
        n=n+1;
        a=b;
        b=c;
        c=a+b;
        if c.is_infinite() {
            break;
        }
        println!("{}",c);
    }
    println!("Iteracions fins arribar a infinit (f64): {}", n);
    // TODO: Show time in seconds to arrive to "inf"
}
