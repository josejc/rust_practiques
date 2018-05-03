//#include <stdio.h>
//#include <ctype.h>
//                                /* minusculas a MAYUSCULAS */
//main()
//{                               /* con GETCHAR & PUTCHAR... */ 
//        char c;
//        while ((c=getchar())!=EOF){     
//                c=toupper(c);   
//                putchar(c);
//        }
//}
use std::io;

fn main() {
    let mut entrada = String::new();

    io::stdin().read_line(&mut entrada).expect(
        "Error al llegir l'entrada",
    );

    let entrada_chars = entrada.chars();
    for i in entrada_chars {
        print!("{}",i.to_uppercase());
    }
}

