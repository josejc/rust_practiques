use std::io;
use std::io::Write; // <--- bring flush() into scope


// Heap's algorithm (https://en.wikipedia.org/wiki/Heap%27s_algorithm)
fn generate(k: usize, a: &mut Vec<usize>) {
    let is_even = |x| x % 2 == 0;

    if k == 1 {
        hprint(a);
    }
    else
    {
        // Generate permutations with kth unaltered
        // Initially k == length(A)
        generate(k-1, a);

        // Generate permutations for kth swapped with each k-1 initial
        for i in 0..k-1 {
            // Swap choice dependent on parity of k (even or odd)
            if is_even(k) {
                a.swap(i,k-1);          // zero-indexed, the kth is at k-1
            }
            else
            {
                a.swap(0,k-1);
            }
            generate(k-1, a);
        }
    }
}

fn initialize<T>(count: usize, f: fn(usize) -> T) -> Vec<T> {
    (0..count).map(f).collect()
}

/*
fn vprint(v: &Vec<usize>) {
    for (i, x) in v.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }
}
*/

fn hprint(v: &Vec<usize>) {
    println!("{:?}",v);
}

fn main() {
    println!("Generate permutations of n elements");

    let mut entrada = String::new();
    let mut n: usize  = 4;

    print!("N Permutation elements? [4]: ");
    io::stdout().flush().expect("Unable to flush stdout");
    io::stdin().read_line(&mut entrada).expect("Error al llegir l'entrada",);
    entrada.pop();
    if !entrada.is_empty() {
        n = entrada.trim().parse().expect("No és un nombre sencer [usize]",);
    }
    if n>16 {
        panic!("Error: n must be in (0 <= n < 17)");
    }

    println!("Permutacions de {} elements {{0,1,2,3..}}", n);
    // Initialize vector for generate the permutations
    let mut v = initialize(n, |i| i as usize);
    // Permutation of the elements
    generate(n, &mut v);
}