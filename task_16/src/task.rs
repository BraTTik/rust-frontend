
use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub priority: u8,
    pub status: String,
}

impl Hash for Task {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Task {}

impl Borrow<u64> for Task {
    fn borrow(&self) -> &u64 {
        &self.id
    }
}

impl BorrowMut<u64> for Task {
    fn borrow_mut(&mut self) -> &mut u64 {
        &mut self.id
    }
}

fn upsert_task<T: Hash + Eq + Clone>(map: &mut HashMap<T, T>, task: T) {
    map.insert(task.clone(), task.clone());
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use super::*;

    fn sample_task(id: u64, title: &str, status: &str) -> Task {
        Task { id, title: title.to_owned(), description: "Sample description".to_owned(), priority: 1, status: status.to_owned() }
    }

    #[test]
    fn tasks_with_same_id_are_equal() {
        let a = sample_task(42, "Task A", "todo");
        let b = sample_task(42, "Task B", "done");
        assert_eq!(a, b);
    }

    #[test]
    fn tasks_with_different_id_are_not_equal() {
        let a = sample_task(42, "A", "todo");
        let b = sample_task(43, "A", "todo");
        assert_ne!(a, b);
    }

    #[test]
    fn upsert_replaces_existing_task() {
        let mut map: HashMap<Task, Task> = HashMap::new();

        let original = sample_task(777, "Old Task", "todo");
        upsert_task(&mut map, original.clone());

        let updated = sample_task(777, "New Task", "done");
        upsert_task(&mut map, updated.clone());

        let stored = map.get(&updated).unwrap();
        assert_eq!(stored.title, "New Task");
        assert_eq!(stored.status, "done");
    }

    #[test]
    fn borrow_allows_lookup_by_id() {
        let mut map: HashSet<Task> = HashSet::new();

        let task = sample_task(1234, "Test", "in_progress");
        map.insert(task.clone());

        assert!(map.contains(&1234u64));
        assert!(map.get(&1234u64).is_some());
        assert!(map.remove(&1234u64));
    }
}