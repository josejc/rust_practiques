use std::io;
use io::Write;

fn main() {
    let mut entrada = String::new();

    print!("FizzBuzz fins a:");
    // Trigger a flush for print the string
    io::stdout().flush().expect("Flush failed!");
    io::stdin().read_line(&mut entrada).expect(
        "Error al llegir l'entrada",
    );

    let end: u32 = entrada.trim().parse().expect(
        // Surt amb panic
       "No és un nombre sencer",
    );

    for i in 1..end {
        match i%6 {
            0 => println!("FizzBuzz"),
            2 => println!("Fizz"),
            3 => println!("Buzz"),
            _ => println!("{}", i),
        }
    }
}
