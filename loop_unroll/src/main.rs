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
}