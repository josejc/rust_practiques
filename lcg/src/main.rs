use std::io;
use std::io::Write; // <--- bring flush() into scope
mod rand;

fn main() {
    println!("Linear Congruential Generator: Xn+1 = (aXn + c) mod m");

    let mut entrada = String::new();
    let n: u32; 
    //let mut r = LCG {x: LCG::seeds(), a: 630_360_016, c: 0, m: 2_147_483_647};
    let mut r = rand::LCG::new(630_360_016, 0, 2_147_483_647);

    print!("X0? [1.973.272.912] ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).expect("Error al llegir l'entrada",);
    entrada.pop();          // Remove CR last char ;)
    if !entrada.is_empty() {
        r.x[1] = entrada.trim().parse().expect("No és un nombre sencer [u32]",);
    }
  
    entrada.clear();
    print!("a? [630.360.016] ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).expect("Error al llegir l'entrada",);
    entrada.pop();
    if !entrada.is_empty() {
        r.a = entrada.trim().parse().expect("No és un nombre sencer [u32]",);
    }

    entrada.clear();
    print!("c? [0] ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).expect("Error al llegir l'entrada",);
    entrada.pop();
    if !entrada.is_empty() {
        r.c = entrada.trim().parse().expect("No és un nombre sencer [u32]",);
    }

    entrada.clear();
    print!("m? [2.147.483.647] ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).expect("Error al llegir l'entrada",);
    entrada.pop();
    if !entrada.is_empty() {
        r.m = entrada.trim().parse().expect("No és un nombre sencer [u32]",);
    }

    entrada.clear();
    print!("n secuència a generar? [25]");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).expect("Error al llegir l'entrada",);
    entrada.pop();
    if entrada.is_empty() {
        n = 25;
    } else {
        n = entrada.trim().parse().expect("No és un nombre sencer [u32]",);
    }

    for _i in 0..n {
        print!("{:.6}, ", r.rand(1));
    }
    println!("...");
}
