use std::collections::HashMap;

pub struct Ring {
    next: HashMap<u32, u32>,
    prev: HashMap<u32, u32>,
}

impl Ring {
    pub fn singleton(id: u32) -> Self {
        Self {
            next: [(id, id)].into_iter().collect(),
            prev: [(id, id)].into_iter().collect(),
        }
    }

    pub fn insert_after(&mut self, existing: u32, new: u32) {
        assert!(!self.next.contains_key(&new));

        let next = self.next[&existing];

        // existing -> new -> next
        self.next.insert(existing, new);
        self.next.insert(new, next);

        // existing <- new <- next
        self.prev.insert(next, new);
        self.prev.insert(new, existing);
    }

    pub fn remove(&mut self, id: u32) {
        assert!(self.next.contains_key(&id));

        let next = self.next[&id];
        let prev = self.prev[&id];

        // prev -> next
        self.next.insert(prev, next);

        // prev <- next
        self.prev.insert(next, prev);

        // Unlink id.
        self.next.remove(&id);
        self.prev.remove(&id);
    }

    pub fn after(&self, mut id: u32, count: usize) -> u32 {
        for _ in 0..count {
            id = self.next[&id];
        }
        id
    }

    pub fn before(&self, mut id: u32, count: usize) -> u32 {
        for _ in 0..count {
            id = self.prev[&id];
        }
        id
    }
}
