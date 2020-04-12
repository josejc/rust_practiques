use std::collections::HashSet;

const N: usize = 9;

fn pendent_fila(f: [u8; N]) -> HashSet<u8> {
    let p: HashSet<u8> =  [1, 2, 3, 4, 5, 6, 7, 8, 9].iter().cloned().collect();    // Conjunt de possibilitats d'una cel.la

    //println!("fila: {:?}",f);
    let f_set: HashSet<u8> = f.iter().cloned().collect();
    let diff: HashSet<u8> = p.difference(&f_set).cloned().collect();
    //println!("pendents: {:?}", diff);

    return diff;
}

fn pendent_columna(s: [[u8; N]; N], col: u8) -> HashSet<u8> {
    let p: HashSet<u8> =  [1, 2, 3, 4, 5, 6, 7, 8, 9].iter().cloned().collect();    // Conjunt de possibilitats d'una cel.la

    let mut c_set: HashSet<u8> = HashSet::new();
    for i in 0..N {
        c_set.insert(s[i][0]);
    }
    //println!("columna: {:?}",c_set  );
    let diff: HashSet<u8> = p.difference(&c_set).cloned().collect();
    //println!("pendents: {:?}", diff);

    return diff;    
}

fn main() {
    
    let mut sudoku = [[0 as u8; N] ; N];
    // Static sudoku; TODO, Read from keyboard or file
    sudoku[0][2] = 3;
    sudoku[0][4] = 2;
    sudoku[0][7] = 6;

    sudoku[1][0] = 9;
    sudoku[1][3] = 3;
    sudoku[1][5] = 5;
    sudoku[1][8] = 1;

    sudoku[2][2] = 1;
    sudoku[2][3] = 8;
    sudoku[2][5] = 6;
    sudoku[2][6] = 4;

    sudoku[3][2] = 8;
    sudoku[3][3] = 1;   
    sudoku[3][5] = 2;
    sudoku[3][6] = 9;

    sudoku[4][0] = 7;
    sudoku[4][8] = 8;

    sudoku[5][2] = 6;
    sudoku[5][3] = 7;
    sudoku[5][5] = 8;
    sudoku[5][6] = 2;

    sudoku[6][2] = 2;
    sudoku[6][3] = 6;
    sudoku[6][5] = 9;
    sudoku[6][6] = 5;

    sudoku[7][0] = 8;
    sudoku[7][3] = 2;
    sudoku[7][5] = 3;
    sudoku[7][8] = 9;

    sudoku[8][2] = 5;
    sudoku[8][4] = 1;
    sudoku[8][6] = 3;

    //let mut possibilitats: [[HashSet<u8>;N];N];    // Per a cada cel.la del sudoku hem de calcular les seves possibilitats

    println!("Inici:");
    for (_i, row) in sudoku.iter().enumerate() {
        println!("{:?}", row);
    }

    //pendent_fila(sudoku[0]);
    //pendent_fila(sudoku[8]);

    //pendent_columna(sudoku, 0);

    //pendent_sub(sudoku, 0, 0); 
}
