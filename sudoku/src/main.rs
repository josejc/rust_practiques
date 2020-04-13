use std::collections::HashSet;
use std::io;
use std::io::Read;

const N: usize = 9;

fn pendent_fila(f: [u8; N]) -> HashSet<u8> {
    let p: HashSet<u8> =  [1, 2, 3, 4, 5, 6, 7, 8, 9].iter().cloned().collect();    // Conjunt de possibilitats d'una cel.la

    //println!("fila: {:?}",f);
    let f_set: HashSet<u8> = f.iter().cloned().collect();
    let diff: HashSet<u8> = p.difference(&f_set).cloned().collect();
    //println!("pendents: {:?}", diff);

    return diff;
}

fn pendent_columna(s: [[u8; N]; N], col: usize) -> HashSet<u8> {
    let p: HashSet<u8> =  [1, 2, 3, 4, 5, 6, 7, 8, 9].iter().cloned().collect();    // Conjunt de possibilitats d'una cel.la

    let mut c_set: HashSet<u8> = HashSet::new();
    for i in 0..N {
        c_set.insert(s[i][col]);
    }
    //println!("columna: {:?}",c_set  );
    let diff: HashSet<u8> = p.difference(&c_set).cloned().collect();
    //println!("pendents: {:?}", diff);

    return diff;    
}

fn pendent_sub(s: [[u8; N]; N], row: usize, col: usize) -> HashSet<u8> {
    let p: HashSet<u8> =  [1, 2, 3, 4, 5, 6, 7, 8, 9].iter().cloned().collect();    // Conjunt de possibilitats d'una cel.la

    let mut s_set: HashSet<u8> = HashSet::new();
    let mut i = 6..9;

    if row < 3 {
        i = 0..3;
    } else if row < 6 {
        i = 3..6;
    }

    for x in i {
        let mut j = 6..9;
        if col < 3 {
            j = 0..3;
        } else if col < 6 {
            j = 3..6;
        }        
        for y in j {      
            s_set.insert(s[x][y]);
        } 
    }
    //println!("sub: {:?}",s_set  );
    let diff: HashSet<u8> = p.difference(&s_set).cloned().collect();
    //println!("pendents: {:?}", diff);

    return diff; 
}

fn entrada(s: &mut [[u8; N]; N]) {
    let mut entrada = String::new();

    io::stdin().read_to_string(&mut entrada).expect(
        "Error al llegir l'entrada",
    );

    let linea = entrada.lines().enumerate();
    for i in linea {
        if i.0 > 8 {
            panic!("Error a l'entrada, no podem processar més de 9 linees.");
        }
        if (i.0 == 0) && (i.1.len() > 9) {   // Format.. Tot a una única linea amb punts per les cel.les buides
            if i.1.len() != 81 {
                panic!("Error a l'entrada, la linea ha de ser de 81 caràcters, només . i del 1 al 9.")
            }
            for j in i.1.chars().enumerate() {
                let x = j.0 / 9;
                let y = j.0 % 9;
                if j.1 == '.' {
                    s[x][y] = 0;
                } else {
                    let digit: u32 = j.1.to_digit(10).unwrap();
                    //println!("Linea {}, Caracter {}, u32 {}, u8 {}", i.0, j.0, digit, (digit & 0xff) as u8);
                    s[x][y] = (digit & 0xff) as u8;     // u32 as 4 bytes, u8 as 1 byte (transform only last byte)
                }    
            }
        } else {
            // Format 9 lineas, amb 9 caràcters i les cel.les buides son un 0
            if i.1.len() != 9 {
                panic!("Error a l'entrada, la linea ha de ser de 9 caràcters, només del 0 al 9.");
            }
            for j in i.1.chars().enumerate() {
                let digit: u32 = j.1.to_digit(10).unwrap();
                //println!("Linea {}, Caracter {}, u32 {}, u8 {}", i.0, j.0, digit, (digit & 0xff) as u8);
                s[i.0][j.0] = (digit & 0xff) as u8;     // u32 as 4 bytes, u8 as 1 byte (transform only last byte)
            }
        }    
    }
}

fn imprimir(s: [[u8; N]; N]) {
    for (i, row) in s.iter().enumerate() {
        if (i % 3) == 0 {
            println!("+-------+-------+-------+");
        }
        for (j, col) in row.iter().enumerate() {
            if (j % 3) == 0 {
                print!("| ");
            }
            if *col == 0 {
                print!(". ");
            } else { 
                print!("{} ", col);
            }
        }
        println!("|");
    }
    println!("+-------+-------+-------+");   
}


fn main() {
    
    let mut sudoku = [[0 as u8; N] ; N];
    entrada(&mut sudoku);

    println!("Inici:");
    imprimir(sudoku);

    let mut fi = false;
    let mut complet = true;
    while !fi {
        complet = true;
        let mut canvi = false;
        for i in 0..9 {
            for j in 0..9 {
                if sudoku[i][j] == 0 {
                    let f = pendent_fila(sudoku[i]);    //i
                    let c = pendent_columna(sudoku, j); //j
                    let s = pendent_sub(sudoku, i, j);  //i,j
                    let m: HashSet<u8>  = f.intersection(&c).cloned().collect();
                    //println!("Mig: {:?}", m);
                    let mut p: HashSet<u8> = m.intersection(&s).cloned().collect();
                    //println!("Possibilitats: {:?}", p);
                    if p.len() == 1 {
                        let number: u8 = p.drain().next().unwrap();
                        //println!("OK {}", number);
                        sudoku[i][j] = number;
                        canvi = true;
                    } else {
                        complet = false;    // Encara no complet pq no hem pogut assignar res a aquesta cel.la
                    }
                }
            }
        }
        if !canvi {     // Si no podem fer canvis aleshores a lo millor es que hem finalitzat?
            fi = true;
        }
    }

    println!("Final:");
    imprimir(sudoku);

    if !complet {       // Encara hi ha cel.les buides (amb 0)
        println!("Sudoku no finalitzat...");
        for i in 0..9 {
            for j in 0..9 {
                if sudoku[i][j] == 0 {
                    let f = pendent_fila(sudoku[i]);    //i
                    let c = pendent_columna(sudoku, j); //j
                    let s = pendent_sub(sudoku, i, j);  //i,j
                    let m: HashSet<u8> = f.intersection(&c).cloned().collect();
                    let p: HashSet<u8> = m.intersection(&s).cloned().collect();
                    println!("Possibilitats posició {}, {}: {:?}", i, j, p);
                }
            }
        }        
    }    
}
