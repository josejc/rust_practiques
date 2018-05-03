//#include <stdio.h>
//main()
//{
//        int i,j;
//        printf("Numero:");
//        scanf("%d",&j);
//        for (i = 0; i <= j; i++)
//                printf("%d:Hola, mundo.\n",i);
//}
use std::io;

fn main () {
    let mut j = String::new();

    io::stdin().read_line(&mut j).expect(
        "Error al llegir l'entrada",
    );

    let j: u32 = j.trim().parse().expect(
        // Surt amb panic
       "No és un nombre sencer",
    );

    for i in 0..j {
        println!("{}: Hola, mundo.", i);
    }
}
