//#include <stdio.h>
//                                /* La linea de comandos */
//main(argc, argv)
//int argc;
//char *argv[];
//{
//        int n=0;                /* Si no queremos ver el nombre del programa */
//        while(argc--){          /* inicializaremos n con 1 y --argc, de esta */
//                printf("%s ",argv[n]);
//                n++;            /* forma, resta antes y luego mira hasta que */
//        }                       /* el numero de argumentos llegue a 0 */
//        printf("\n");
//}       
// -- Traducció directa de C
fn main() {
    for arg in std::env::args() {
        print!("{} ", arg);
    }
    println!();
}

// Rust té llibreries per tractar directament els arguments
// use std::env;
//
//fn main() {
//    let first = env::args().nth(1).expect("please supply an argument");
//    let n: i32 = first.parse().expect("not an integer!");
//    // do your magic
//}
