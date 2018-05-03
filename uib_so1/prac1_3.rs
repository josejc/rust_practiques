//#include <stdio.h>
//#include <stdlib.h>
//                        /* Cantidad de caracters leidos de la entrada */
//main()
//                        /* c es entera, guarda el codigo ascii de entrada */
//{
//        int i,c;        /* tambien la variable c podria ser tipo char */
//        i=0;
//        while ((c=getchar())!=EOF)
//                i++;
//        printf("\nNumero total de caracteres:%d\n",i);
//} 
use std::io;

fn main() {
    let mut entrada = String::new();

    io::stdin().read_line(&mut entrada).expect(
        "Error al llegir l'entrada",
    );
    
    println!("Nombre total de bytes (len): {}", entrada.len());
    println!("Nombre total de caracters: {}", entrada.chars().count());
}
