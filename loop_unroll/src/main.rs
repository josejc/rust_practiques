fn main() {

    println!("2 loops of range 4");
    for i in 1..5 {
        for j in 1..5 {
            println!("{},{}", i,j);
        } 
    }
    println!("1 loop of range 4*4");
    let mut x = 0;
    while x < 16 {
        let i = (x/4)+1;    
        let j = (x%4)+1;    // Remainder
        println!("{},{}", i,j);
        x += 1;
    }
    println!("4 loops of range 4 -> 256");    
    for a in 1..5 {
        for b in 1..5 {
            for c in 1..5 {
                for d in 1..5 {
                    println!("{},{},{},{}", a,b,c,d);
                }
            } 
       }
    }
    println!("1 loop of range 4*4*4*4 (256)");
    let mut x = 0;
    while x < 256 {
        let d = (x%4)+1;    // Remainder
        let c = ((x/4)%4)+1;
        let b = ((x/16)%4)+1;
        let a = ((x/64)%4)+1;   
        println!("{},{},{},{}", a,b,c,d);
        x += 1;
    }
    println!("1 loop of range 4*4*4*4 (256) amb array with formula");
    for x in 1..257 {
        let mut e: [i32; 5] = [0; 5];
        for i in 1..5 {
            e[i] = ((x/4_i32.pow((i as u32)-1))%4)+1;
        } 
        println!("{},{},{},{}", e[4],e[3],e[2],e[1]);
    }    
    // Check loops for generate index (function equality - permutation)
    println!("Index for equality in permutation");
    // i -> 1..n-1, j ->i+1..n
    for i in 1..4 {
        for j in i+1..5 {
            println!("{},{}",i,j);
        }
    }
}