#![warn(clippy::all, clippy::pedantic)]

//binary_search pag.9 - grokking algorithms
//Only work in a ordered array ;-)
fn binary_search (list: &Vec<i32>, item:i32) -> i32 {
    let mut low = 0;
    let mut high = list.len()-1;

    while low <= high {
        let mid = (low + high) / 2;
        let guess = list[mid];
        if guess == item {
            return mid.try_into().unwrap();
        } 
        if guess > item {
            if mid == 0 {
                break;
            }
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    -1
}

// binary_search_recursive, Exercise 4.3
fn bsr (list: &Vec<i32>, item: i32) -> i32 {
    let mid = list.len() / 2;
    //println!("{:?}", list);
    //println!("len:{}", list.len());
    //println!("mid:{}", mid);
    if list.len() == 0 {
        return -1;
    }
    if list[mid] == item {
        return mid as i32;
    }
    if list[mid] > item {
        return bsr(&list[0..mid].to_vec(), item);
    } else {
        // TODO lost the lengh of the original vector
        // The index solution is in the new vector...
        // Fix... 
        let n = list.len();
        if (mid + 1) <= n {
            return bsr(&list[mid+1..n].to_vec(), item);
        } 
    }
    -1
}

fn main() {
    let my_list = vec![1, 3, 5, 7, 9];

    println!("{:?}", my_list);
    println!("Binary search 1 at index: {}", binary_search(&my_list, 1));
    println!("Binary search 3 at index: {}", binary_search(&my_list, 3));
    println!("Binary search 5 at index: {}", binary_search(&my_list, 5));
    println!("Binary search 7 at index: {}", binary_search(&my_list, 7));
    println!("Binary search 9 at index: {}", binary_search(&my_list, 9));
    println!("Binary search 6 at index: {}", binary_search(&my_list, 6));
    println!("Binary search 26 at index: {}", binary_search(&my_list, 26));
    println!("Binary search -3 at index: {}", binary_search(&my_list, -3));
    println!("Binary search recursive 1 at index: {}", bsr(&my_list, 1));
    println!("Binary search recursive 3 at index: {}", bsr(&my_list, 3));
    println!("Binary search recursive 5 at index: {}", bsr(&my_list, 5));
    println!("Binary search recursive 7 at index: {}", bsr(&my_list, 7));
    println!("Binary search recursive 9 at index: {}", bsr(&my_list, 9));
    println!("Binary search recursive 6 at index: {}", bsr(&my_list, 6));
    println!("Binary search recursive 26 at index: {}", bsr(&my_list, 26));
    println!("Binary search recursive -3 at index: {}", bsr(&my_list, -3));
}
