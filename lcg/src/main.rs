use std::io;
use std::io::Write; // <--- bring flush() into scope
mod rand;

fn main() {
    println!("Linear Congruential Generator: Xn+1 = (aXn + c) mod m");

    let mut entrada = String::new();
    let mut n: usize = 25; 
    let mut stream: usize = 1;
    //let mut r = LCG {x: LCG::seeds(), a: 630_360_016, c: 0, m: 2_147_483_647};
    //let mut r = rand::LCG::new(630_360_016, 0, 2_147_483_647);
    let mut r = rand::LCG::new();

    entrada.clear();
    print!("Stream? [1]");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).expect("Error al llegir l'entrada",);
    entrada.pop();
    if !entrada.is_empty() {
        stream = entrada.trim().parse().expect("No és un nombre sencer [usize]",);
    }
    entrada.clear();
    print!("n secuència a generar? [25]");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).expect("Error al llegir l'entrada",);
    entrada.pop();
    if !entrada.is_empty() {
        n = entrada.trim().parse().expect("No és un nombre sencer [usize]",);
    }

    for _i in 0..n {
        print!("{:.6}, ", r.rand(stream));
    }
    println!("...");
}
