use std::io;

fn main() {
    let mut entrada = String::new();

    io::stdin().read_line(&mut entrada).expect(
        "Error al llegir l'entrada",
    );
    
    let paraules: Vec<&str> = entrada.split_whitespace().collect();
    println!("{:?}", paraules);
    println!("Nombre paraules: {}", paraules.iter().count());
}
