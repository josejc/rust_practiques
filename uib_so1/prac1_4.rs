//#include <stdio.h>
//#include <ctype.h>
//                                /* minusculas a MAYUSCULAS */
//main()
//{                               /* con READ & WRITE... */ 
//        char c;
//        while (read(0,&c,1)){   /* Bucle hasta que read retorne 0 */
//                c=toupper(c);   /* cuando no lea ningun caracter de */
//                write(1,&c,1);  /* la entrada, es decir el EOF */
//        }
//}
use std::io;

fn main() {
    let mut entrada = String::new();

    io::stdin().read_line(&mut entrada).expect(
        "Error al llegir l'entrada",
    );

    println!("Mayuscules: {}", entrada.to_uppercase());
}
