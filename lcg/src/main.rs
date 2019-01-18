use std::io;
use std::io::Write; // <--- bring flush() into scope

fn main() {
    println!("Linear Congruential Generator: Xn+1 = (aXn + c) mod m");

    let mut entrada = String::new();
    let mut x: u32;
    let mut u: f32;
    let mut aux: u64;   // Auxiliar per evitar overflow 
    let a: u32;
    let c: u32;
    let m: u32;
    let n: u32; 

    print!("X0? [1.973.272.912] ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).expect("Error al llegir l'entrada",);
    entrada.pop();          // Remove CR last char ;)
    if entrada.is_empty() {
        x = 1_973_272_912;
    } else {
        x = entrada.trim().parse().expect("No és un nombre sencer [u32]",);
    }
  
    entrada.clear();
    print!("a? [630.360.016] ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).expect("Error al llegir l'entrada",);
    entrada.pop();
    if entrada.is_empty() {
        a = 630_360_016;
    } else {
        a = entrada.trim().parse().expect("No és un nombre sencer [u32]",);
    }

    entrada.clear();
    print!("c? [0] ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).expect("Error al llegir l'entrada",);
    entrada.pop();
    if entrada.is_empty() {
        c = 0;
    } else {
        c = entrada.trim().parse().expect("No és un nombre sencer [u32]",);
    }

    entrada.clear();
    print!("m? [2.147.483.647] ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).expect("Error al llegir l'entrada",);
    entrada.pop();
    if entrada.is_empty() {
        m = 2_147_483_647;  // 2^31-1
    } else {
        m = entrada.trim().parse().expect("No és un nombre sencer [u32]",);
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
        //x = ((a * x) + c ) % m; Panic multiply with overflow
        aux = a as u64 * x as u64;
        aux = aux + c as u64;
        aux = aux % m as u64;
        x = aux as u32;
        u = x as f32 / m as f32;
        print!("{:.6}, ", u);
    }
    println!("...");
}
