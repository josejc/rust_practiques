#![warn(clippy::all, clippy::pedantic)]

//binary_search pag.9 - grokking algorithms
fn binary_search (list: [i32; 5], item:i32) -> i32 {
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

fn main() {
    let my_list = [1, 3, 5, 7, 9];

    println!("{}", binary_search(my_list, 1));
    println!("{}", binary_search(my_list, 3));
    println!("{}", binary_search(my_list, 5));
    println!("{}", binary_search(my_list, 7));
    println!("{}", binary_search(my_list, 9));
    println!("{}", binary_search(my_list, 6));
    println!("{}", binary_search(my_list, 26));
    println!("{}", binary_search(my_list, -3));
}
