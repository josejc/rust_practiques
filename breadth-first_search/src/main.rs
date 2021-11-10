mod queue;

mod prelude {
    pub use crate::queue::*;
    pub use std::collections::HashMap;
}

use prelude::*;

fn main() {
    let mut queue: Queue<String> = Queue::new();
    queue.add("bob".to_string());
    let item = queue.out();
    assert_eq!(item, "bob".to_string());
    assert_eq!(queue.is_empty(), true);

    // Type inference lets us omit an explicit type signature (which
    // would be `HashMap<String, Vec<String>>` in this example).
    let mut relations = HashMap::new();

    // Insert friends in HashMap
    relations.insert(
        "you".to_string(),
        vec!["alice", "bob", "claire"],
    );
    relations.insert(
        "bob".to_string(),
        vec!["anuj", "peggy"],
    );
    relations.insert(
        "claire".to_string(),
        vec!["thom", "jonny"],
    );
    relations.insert(
        "alice".to_string(),
        vec!["peggy"],
    );
    relations.insert(
        "anuj".to_string(),
        vec![],
    );
    relations.insert(
        "peggy".to_string(),
        vec![],
    );
    relations.insert(
        "thom".to_string(),
        vec![],
    );
    relations.insert(
        "jonny".to_string(),
        vec![],
    );
    
    // Iterate over everything.
    for (person, friends) in &relations {
        println!("{}: \"{:?}\"", person, friends);
    }
}
