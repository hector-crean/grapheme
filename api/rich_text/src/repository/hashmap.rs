use std::collections::HashMap;

use uuid::Uuid;

use super::RichTextRepositoryLike;

// Implement RichTextRepository for a HashMap
pub struct HashMapRepository(HashMap<Uuid, String>);

impl Default for HashMapRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl HashMapRepository {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl RichTextRepositoryLike for HashMapRepository {
    type RichText = String;
    fn insert(&mut self, id: &Uuid, rich_text: String) -> bool {
        if self.0.contains_key(id) {
            false
        } else {
            self.0.insert(*id, rich_text);
            true
        }
    }

    fn update(&mut self, id: &Uuid, rich_text: String) -> bool {
        if let Some(prev_rich_text) = self.0.get_mut(id) {
            *prev_rich_text = rich_text;
            true
        } else {
            false
        }
    }

    fn delete(&mut self, id: &Uuid) -> bool {
        self.0.remove(id).is_some()
    }

    fn get(&mut self, id: &Uuid) -> Option<String> {
        self.0.get(id).cloned()
    }

    fn list(&mut self) -> Vec<(Uuid, String)> {
        self.0
            .iter()
            .map(|entry| (*entry.0, entry.1.clone()))
            .collect()
    }
}
