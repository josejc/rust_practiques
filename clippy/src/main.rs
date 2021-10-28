fn main() {
    //let MYLIST = ["One", "Two", "Three"];
    let my_list = ["One", "Two", "Three"];
    //for i in 0..3 {
    for num in &my_list {
        println!("{}", num);
    }
}
