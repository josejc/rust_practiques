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
            }        }    
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

fn pos(sudo: [[u8; N]; N], row: usize, col: usize) -> HashSet<u8> {

    let mut p: HashSet<u8> = HashSet::new();
    if sudo[row][col] == 0 {
        let f = pendent_fila(sudo[row]);
        let c = pendent_columna(sudo, col);
        let s = pendent_sub(sudo, row, col);
        let m: HashSet<u8>  = f.intersection(&c).cloned().collect();
        //println!("Mig: {:?}", m);
        p = m.intersection(&s).cloned().collect();
    }

    return p;
} 

fn restricc_fila(s: &mut [[u8; N]; N], row: usize) -> bool {     // Retorna 'true' si modifica el sudoku
    let mut modified = false;

    let mut p_f: [HashSet<u8>; N] = [HashSet::new(), HashSet::new(), HashSet::new(),
        HashSet::new(), HashSet::new(), HashSet::new(),HashSet::new(), HashSet::new(), HashSet::new()];
    
    // Calcular les possibilitats de cada cel.la d'aquesta fila
    for y in 0..N {
        p_f[y] = pos(*s, row, y);
    } 

    // Check de cada cel.la la resta per veure si hi ha un nombre exclusiu 
    for y in 0..9 {
        let mut p: HashSet<u8> = p_f[y].iter().cloned().collect();
        for col in 0..9 {
            if (y != col) && !p.is_empty() {
                p = p.difference(&p_f[col]).cloned().collect();
            }    
        }
        if p.len() == 1 {
            let number: u8 = p.drain().next().unwrap();
            s[row][y] = number;
            modified = true;            
        }

    } 

    return modified;
}

fn restricc_col(s: &mut [[u8; N]; N], col: usize) -> bool {     // Retorna 'true' si modifica el sudoku
    let mut modified = false;

    let mut p_c: [HashSet<u8>; N] = [HashSet::new(), HashSet::new(), HashSet::new(),
        HashSet::new(), HashSet::new(), HashSet::new(),HashSet::new(), HashSet::new(), HashSet::new()];
    
    // Calcular les possibilitats de cada cel.la d'aquesta fila
    for x in 0..N {
        p_c[x] = pos(*s, x, col);
    } 

    // Check de cada cel.la la resta per veure si hi ha un nombre exclusiu 
    for x in 0..9 {
        let mut p: HashSet<u8> = p_c[x].iter().cloned().collect();
        for row in 0..9 {
            if (x != row) && !p.is_empty() {
                p = p.difference(&p_c[row]).cloned().collect();
            }    
        }
        if p.len() == 1 {
            let number: u8 = p.drain().next().unwrap();
            s[x][col] = number;
            modified = true;            
        }

    } 

    return modified;
}

fn main() {
    
    let mut sudoku = [[0 as u8; N] ; N];
    entrada(&mut sudoku);

    println!("Inici:");
    imprimir(sudoku);

/*
    let mut fi = false;
    let mut complet = true;
    while !fi {
        complet = true;
        let mut canvi = false;
        for i in 0..9 {
            for j in 0..9 {
                if sudoku[i][j] == 0 {
                    let mut p = pos(sudoku, i , j);
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
*/
    for i in 0..9 {
        if restricc_fila(&mut sudoku, i) {
            println!("S'HA MODIFICAT, LINEA {}", i);
        }    
    }
    for i in 0..9 {
        if restricc_col(&mut sudoku, i) {
            println!("S'HA MODIFICAT, COLUMNA {}", i);
        }    
    }
    println!("Final:");
    imprimir(sudoku);
/*
    if !complet {       // Encara hi ha cel.les buides (amb 0)
        println!("Sudoku no finalitzat...");
        for i in 0..9 {
            for j in 0..9 {
                if sudoku[i][j] == 0 {
                    let p = pos(sudoku, i, j);
                    println!("Possibilitats posició {}, {}: {:?}", i, j, p);
                }
            }
        }        
    }
*/     
}
