use std::io;

fn main() {
    let mut entrada = String::new();

    io::stdin().read_line(&mut entrada).expect(
        "Error al llegir l'entrada",
    );

    //let adartne = entrada.chars().rev().collect::<String>();
    let adartne: String = entrada.chars().rev().collect();
    println!("{}", adartne);
}
