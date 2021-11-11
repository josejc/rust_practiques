mod queue;

mod prelude {
    pub use crate::queue::*;
    pub use std::collections::HashMap;
}

use prelude::*;

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
    // would be `HashMap<String, Vec<&str>>` in this example).
    let mut relations = HashMap::new();

    // Insert friends in HashMap
    let data_person = build_data(vec!["alice".to_string(), "bob".to_string(), "claire".to_string()], false);
    relations.insert(
        "you".to_string(),
        data_person
    );

    let data_person = build_data(vec!["anuj".to_string(), "peggy".to_string()], false);
    relations.insert(
        "bob".to_string(),
        data_person
    );
    let data_person = build_data(vec!["thom".to_string(), "jonny".to_string()], false);
    relations.insert(
        "claire".to_string(),
        data_person
    );
    let data_person = build_data(vec!["peggy".to_string()], false);
    relations.insert(
        "alice".to_string(),
        data_person
    );
    let data_person = build_data(vec![], false);
    relations.insert(
        "anuj".to_string(),
        data_person
    );
    let data_person = build_data(vec![], false);
    relations.insert(
        "peggy".to_string(),
        data_person
    );
    let data_person = build_data(vec![], true);
    relations.insert(
        "thom".to_string(),
        data_person
    );
    let data_person = build_data(vec![], false);
    relations.insert(
        "jonny".to_string(),
        data_person
    );

    relations
}

fn main() {
    let mut queue: Queue<String> = Queue::new();
    queue.add("bob".to_string());
    let item = queue.out();
    assert_eq!(item, "bob".to_string());
    assert_eq!(queue.is_empty(), true);

    let mut relations = inserts_friends();
    
    // Iterate over everything.
    for (person, data) in &relations {
        println!("{}: \"{:?}\"", person, data);
    }
}
