#![warn(clippy::all, clippy::pedantic)]
use rand::Rng;

//quick_sort p.65 grokking algorithms
fn quick_sort(list: &mut Vec<i32>) -> Vec<i32> {
    let len = list.len();
    if len < 2 {
        return list.to_vec();
    } else {
        let mut rng = rand::thread_rng();
        let pivot =  rng.gen_range(0..len);
        let mut less = Vec::new();
        let mut greater = Vec::new();
        // vector less of elements that number is small than pivot
        // vector greater of elements that number is greater than pivot 
        for val in list.iter() {
            // Skip the pivot and all elements equal pivot ;-), remove duplicates
            if *val != list[pivot] {
                if *val < list[pivot] {
                    less.push(*val);
                } else {
                    greater.push(*val);
                }
            }
        }
        let mut less_sort = quick_sort(&mut less);
        let mid = &mut vec!(list[pivot]);
        let mut greater_sort = quick_sort(&mut greater);
        less_sort.append(mid);
        less_sort.append(&mut greater_sort);
        return less_sort;
    }
}


fn main() {
    let mut my_list = vec![10, 5, 10,2 ,3];

    println!("{:?}", my_list);
    println!("Quick sort: {:?}", quick_sort(&mut my_list));
}
