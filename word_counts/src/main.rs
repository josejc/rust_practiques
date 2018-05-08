use std::io;
use std::io::Read;

fn main() {
    let mut entrada = String::new();

    io::stdin().read_to_string(&mut entrada).expect(
        "Error al llegir l'entrada",
    );

    let m_lineas = entrada.lines().enumerate();
    let paraules: Vec<&str> = entrada.split_whitespace().collect();
    let n_lineas: Vec<&str> = entrada.split_terminator("\n").collect();
    println!("Nombre n_lineas: {}", n_lineas.iter().count());
    println!("Nombre m_lineas: {}", m_lineas.count());
    println!("Nombre paraules: {}", paraules.iter().count());
    println!("Nombre bytes: {}", entrada.bytes().count());
}
