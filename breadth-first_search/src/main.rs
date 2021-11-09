mod queue;

mod prelude {
    pub use crate::queue::*;
}

use prelude::*;

fn main() {
    let mut queue: Queue<isize> = Queue::new();
    queue.enqueue(1);
    let item = queue.dequeue();
    assert_eq!(item, 1);
    assert_eq!(queue.is_empty(), true);
}
