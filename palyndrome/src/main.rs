extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;
use std::io;
use std::io::Write; // <--- bring flush() into scope

fn main() {
    let mut entrada = String::new();
    let mut cadena = String::new();

    print!("Entrada ascii: ");
    io::stdout().flush().unwrap();    
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

    entrada.clear();
    cadena.clear();
    print!("Entrada Unicode: ");
    io::stdout().flush().unwrap();    
    io::stdin().read_line(&mut entrada).expect(
        "Error al llegir l'entrada",
    );

    for word in entrada.split_whitespace() {
        cadena.push_str(word);
    }
    cadena = cadena.to_lowercase();
    let anedac: String = cadena
        // Split the string into an Iterator of &strs, where each element is an
        // extended grapheme cluster.
        .graphemes(true)
        // Reverse the order of the grapheme iterator.
        .rev()
        // flat_map takes each element of an iterator, turns that element into
        // a new iterator, then outputs the elements of these sub-iterators as
        // one long chain.  In this case, we're turning each grapheme cluster
        // into an Iterator of code points, then yielding all those code points.
        // That is, this is now an Iterator of chars from the reversed grapheme
        // clusters.
        .flat_map(|g| g.chars())
        // Collect all the chars into a new owned String.
        .collect();

    println!("unicode: {}", cadena);
    println!("edocinu: {}", anedac);
 
}
