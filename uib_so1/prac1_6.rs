//#include <stdio.h>
//                                /* ?Que numero tiene la linea? */
//main()
//{                               /* Declaraciones y inicializaciones */
//        int n=0;
//        char *p;
//        char array[80];         /* Max. caracteres por linea */
//        p=&array[0];
//
//        while (gets(p)!=NULL){  /* Mientras gets no retorne NULL */
//                n++;
//                printf("N: linea %d:%s\n",n,p);
//        }       
//}
use std::io;
use std::io::Read;

fn main() {
    let mut entrada = String::new();

    io::stdin().read_to_string(&mut entrada).expect(
        "Error al llegir l'entrada",
    );

    let linea = entrada.lines().enumerate();
    for i in linea {
        //println!("Linea {:?}", i); 
        println!("Linea {}: {}", i.0, i.1);
    }
}
