//#include <stdio.h>
//                                /* ?Que numero tiene la linea? */
//main()
//{                               /* Declaraciones y inicializaciones */
//        int n=0;
//        char *p;
//        char array[80];         /* Max. caracteres por linea */
//        p=&array[0];
//
//        while (fgets(p,80,stdin)!=NULL){  /* Hasta que retorne NULL */  
//                n++;
//                printf("N: linea %d:",n); /* El n: de linea... */
//                fputs(p,stdout);          /* ahora viene la linea */
//        }       
//}
//-- Ara fer la materix practica amb read_to_end -> Vector ;-)
use std::io;
use std::io::Read;
//use std::str;

fn main() {
    let mut entrada = Vec::new();

    io::stdin().read_to_end(&mut entrada).expect(
        "Error al llegir l'entrada",
    );

    //let s = match str::from_utf8(&entrada) {
    //    Ok(v) => v,
    //    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    //};
    // s es ara &str
    
    let s = String::from_utf8_lossy(&entrada);
    // lossy -> including invalid characters
    // let s = String::from_utf8(entrada).expect("Found invalid UTF-8");
    // s es ara String    

    let linea = s.lines().enumerate();
    for i in linea {
        println!("Linea {}: {}", i.0, i.1);
    }
}
