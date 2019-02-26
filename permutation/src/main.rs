fn main() {

    println!("Permutacions de 4 elements {{1,2,3,4}}");

    let mut i = 1;

    for a in 1..5 {
        for b in 1..5 {
            for c in 1..5 {
                for d in 1..5 {
                    //println!("{}{}{}{}", a,b,c,d);
                    if (a!=b) && (a!=c) && (a!=d) {
                        if (b!=c) && (b!=d) {
                            if c!=d {
                                println!("----{}: {}{}{}{}", i,a,b,c,d);
                                i += 1;
                            }
                        }
                    }
                }
            } 
       }
    }
}