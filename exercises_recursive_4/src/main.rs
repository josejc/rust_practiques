#![warn(clippy::all, clippy::pedantic)]

// Sum all elements of Vector, recursive and empty the Vector 
fn sum4_1(list: &mut Vec<i32>) -> i32 {
    
    if let Some(x) = list.pop() {
        return x + sum4_1(list);    
    } else {
        return 0; 
    }
}

// Count function is the same len() but empty the Vector, Recursive
fn count4_2(list: &mut Vec<i32>) -> i32 {
    
    if let Some(_x) = list.pop() {
        return 1 + count4_2(list)
    } else {
        return 0;
    }
}

// Max function return the maximum number in a Vector and modified it
fn max4_3(list: &mut Vec<i32>) -> i32 {
    // Base cases
    if list.len() == 1 {
        return list[0];
    }
    if list.len() == 2 {
        if list[0] > list[1] {
            return list[0];
        } else {
            return list[1];
        }
    }
    if let Some(x) = list.pop () {
        let m1 = max4_3(list);
        if x > m1 {
            return x;
        } else {
            return m1;
        }
    }
    // If Vector is empty
    return -1
}    


fn main() {
    let my_list = vec![5, 3, 6, 2, 10];

    println!("{:?}", my_list);
    println!("4.1 Sum the elements in Vector: {:?}", sum4_1(&mut my_list.clone()));
    println!("4.2 Count the elements in Vector: {:?}", count4_2(&mut my_list.clone()));
    println!("4.3 Find the maximum number in Vector: {:?}", max4_3(&mut my_list.clone()));
}
