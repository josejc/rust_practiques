#![warn(clippy::all, clippy::pedantic)]

// Sum all elements of Vector, recursive
fn sum4_1(list: &mut Vec<i32>) -> i32 {
    
    if list.len() == 1 {
        return list.pop().unwrap();
    }
    
    return list.pop().unwrap() + sum4_1(list)
}

// Count function is the same len() in Vector, now Recursive
fn count4_2(list: &mut Vec<i32>) -> i32 {
    
    if list.len() == 1 {
        return 1;
    }
    list.pop();
    return 1 + count4_2(list)
}

fn main() {
    let my_list = vec![5, 3, 6, 2, 10];

    println!("{:?}", my_list);
    println!("4.1 Sum the elements in Vector: {:?}", sum4_1(&mut my_list.clone()));
    println!("4.2 Count the elements in Vector: {:?}", count4_2(&mut my_list.clone()));
}
