use std::io;
use std::io::Write; // <--- bring flush() into scope

pub struct LCG {
    x: u32,
    a: u32,
    c: u32,
    m: u32,
}

pub fn rand(r: &mut LCG) -> f32 {
    //x = ((a * x) + c ) % m; Panic multiply with overflow
    let mut aux: u64;       // Solution to overflow of u32 

    aux = r.a as u64 * r.x as u64;
    aux = aux + r.c as u64;
    aux = aux % r.m as u64;
    r.x = aux as u32;
    (r.x as f32 / r.m as f32)
}

fn main() {
    println!("Linear Congruential Generator: Xn+1 = (aXn + c) mod m");

    let mut entrada = String::new();
    let n: u32; 
    let mut r = LCG { x: 1_973_272_912, a: 630_360_016, c: 0, m: 2_147_483_647};

    print!("X0? [1.973.272.912] ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).expect("Error al llegir l'entrada",);
    entrada.pop();          // Remove CR last char ;)
    if !entrada.is_empty() {
        r.x = entrada.trim().parse().expect("No és un nombre sencer [u32]",);
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
        print!("{:.6}, ", rand(&mut r));
    }
    println!("...");
}
