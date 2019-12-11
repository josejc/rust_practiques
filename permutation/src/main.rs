use std::io;
use std::io::Write; // <--- bring flush() into scope

/* Heap's algorithm (https://en.wikipedia.org/wiki/Heap%27s_algorithm)
procedure generate(k : integer, A : array of any):
    if k = 1 then
        output(A)
    else
        // Generate permutations with kth unaltered
        // Initially k == length(A)
        generate(k - 1, A)

        // Generate permutations for kth swapped with each k-1 initial
        for i := 0; i < k-1; i += 1 do
            // Swap choice dependent on parity of k (even or odd)
            if k is even then
                swap(A[i], A[k-1]) // zero-indexed, the kth is at k-1
            else
                swap(A[0], A[k-1])
            end if
            generate(k - 1, A)

        end for
    end if
*/

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
    if (n<1) || (n>16) {
        panic!("Error: n must be in (0 < n < 17)");
    }


    println!("Permutacions de {} elements {{1,2,3..}}", n);

}