pub struct Queue<T> {
    queue: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { queue: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn add(&mut self, item: T) {
        self.queue.push(item)
    }

    pub fn out(&mut self) -> T {
        self.queue.remove(0)
    }
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn peek(&self) -> Option<&T> {
        self.queue.first()
    }
}
