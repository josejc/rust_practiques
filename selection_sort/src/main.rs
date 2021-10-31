#![warn(clippy::all, clippy::pedantic)]

fn smallest(list: &mut Vec<i32>) -> i32 {
    let min_value = list.iter().min();
    
    match min_value {
        Some(min) => return *min,
        None      => panic!("Vector is empty"),
    }
}

//selection_sort pag.35 - grokking algorithms
fn selection_sort(list: &mut Vec<i32>) -> Vec<i32> {
    let mut ordered_list: Vec<i32> = vec![];

    while list.len() != 0 {
        let m = smallest(list);
        let index = list.iter().position(|x| *x == m).unwrap();
        list.remove(index);
        ordered_list.push(m);
    }
    ordered_list
}

fn main() {
    let mut my_list = vec![5, 3, 6, 2, 10];

    println!("{:#?}", my_list);
    println!("Selection Sort {:#?}", selection_sort(&mut my_list));
}
