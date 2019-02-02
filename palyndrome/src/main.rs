use std::io;

fn main() {
    let mut entrada = String::new();
    let mut cadena = String::new();

    io::stdin().read_line(&mut entrada).expect(
        "Error al llegir l'entrada",
    );

    for word in entrada.split_whitespace() {
        cadena.push_str(word);
    }
    cadena = cadena.to_ascii_lowercase();

    let mut palyndrome = true;

    if !cadena.is_ascii() {
        panic!("Not prepare for Unicode :-p");
    }
    
     while cadena.len() > 1 {
        //println!("{}", cadena);
        if Some(cadena.remove(0)) != cadena.pop() {
            //println!("{}", cadena);          
            palyndrome = false;
            break;
        }
    }

    if palyndrome {
        println!("Is a palyndrome");
    } else {
        println!("Is not a palyndrome");
    }
}
