use std::sync::{Arc, Mutex};

struct Something {
    size: usize,
}

impl Something {
    fn increase(&mut self) {
        self.size = self.size + 1;
    }
}

fn main() {
    let something = Something { size: 1 };
    let arc = Arc::new(Mutex::new(something));
    arc.lock().unwrap().increase();
}
