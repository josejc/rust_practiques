use std::io;
use std::io::Write; // <--- bring flush() into scopeo

fn equals(n: usize, e: [usize; 17]) -> bool {
    for i in 1..n {             // REMEMBER for need plus one at end
        for j in i+1..n+1 {
            //println!("--e{}:{},e{}:{},",i,e[i],j,e[j]);        
            if e[i] == e[j] {
                return true;
            }
        }
    }
    // All elements are differents
    return false;
}

fn main() {
    println!("Generate permutations of n elements");

    let mut entrada = String::new();
    let mut n: usize  = 4;

    print!("N Permutation elements? [4]");
    io::stdout().flush().expect("Unable to flush stdout");
    io::stdin().read_line(&mut entrada).expect("Error al llegir l'entrada",);
    entrada.pop();
    if !entrada.is_empty() {
        n = entrada.trim().parse().expect("No és un nombre sencer [usize]",);
    }
    if (n<1) || (n>16) {
        panic!("Error: n must be in (0 < n < 17)");
    }


    println!("Permutacions de {} elements {{1,2,3..}}", n);
    let finish = n.pow(n as u32)+1;
    let mut p = 0;
    //println!("Finish: {}", finish);
    let mut e: [usize; 17] = [0; 17];
    for x in 1..finish {
        for i in 1..n+1 {
            e[i] = ((x/n.pow((i as u32)-1))%n)+1;
            //print!("{},",e[i]);
        } 
        //println!("");
        //println!("n: {:?}",n);
        //println!("e: {:?}",e);
        //println!("equals: {:?}", equals(n,e));
        if !equals(n, e) {
            p += 1;
            print!("----{}:",p);
            for i in 1..n+1 {
                print!("{},",e[i]);
            }
            println!("");
        }
    }   
}