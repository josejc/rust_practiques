mod queue;

mod prelude {
    pub use crate::queue::*;
    pub use std::collections::HashMap;
}

use prelude::*;
use std::process;

#[derive(Debug)]
struct Data {
    friends: Vec<String>,
    mango_seller: bool,
}

fn build_data(friends: Vec<String>, mango_seller: bool) -> Data {
    Data {
        friends,
        mango_seller,
    }
}

fn inserts_friends() -> std::collections::HashMap<String, Data> {
    // Type inference lets us omit an explicit type signature (which
    // would be `HashMap<String, Vec<String>>` in this example).
    let mut relations = HashMap::new();

    // Insert friends in HashMap
    let data_person = build_data(
        vec!["alice".to_string(), "bob".to_string(), "claire".to_string()],
        false,
    );
    relations.insert("you".to_string(), data_person);

    let data_person = build_data(vec!["anuj".to_string(), "peggy".to_string()], false);
    relations.insert("bob".to_string(), data_person);
    let data_person = build_data(vec!["thom".to_string(), "jonny".to_string()], false);
    relations.insert("claire".to_string(), data_person);
    let data_person = build_data(vec!["peggy".to_string()], false);
    relations.insert("alice".to_string(), data_person);
    let data_person = build_data(vec![], false);
    relations.insert("anuj".to_string(), data_person);
    let data_person = build_data(vec![], false);
    relations.insert("peggy".to_string(), data_person);
    let data_person = build_data(vec![], true);
    relations.insert("thom".to_string(), data_person);
    let data_person = build_data(vec![], false);
    relations.insert("jonny".to_string(), data_person);

    relations
}

fn exists(mut q: Queue<String>, name: &String) -> bool {
    while !q.empty() {
        if q.out() == name.to_string() {
            return true;
        }
    }
    return false;
}

fn main() {
    let mut breadth: Queue<String> = Queue::new();
    let mut searched: Queue<String> = Queue::new();
    let relations = inserts_friends();

    let value = relations.get(&"you".to_string());
    if let Some(v) = value {
        for val in &v.friends {
            breadth.add(val.to_string());
        }
        searched.add("you".to_string());
    }

    while !breadth.empty() {
        let person = breadth.out();

        if !exists(searched.clone(), &person) {
            let data = relations.get(&person);
            println!("Friend: {}", person);
            if let Some(d) = data {
                if d.mango_seller {
                    println!("Great! I find a mango seller");
                    process::exit(1);
                } else {
                    searched.add(person);
                    for val in &d.friends {
                        breadth.add(val.to_string());
                    }
                }
            }
        }
    }
    println!("Sorry! I don't find a mango seller");
}
