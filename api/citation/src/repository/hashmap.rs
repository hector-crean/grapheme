use std::collections::HashMap;

use uuid::Uuid;

use super::ReferenceRepositoryLike;

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

impl ReferenceRepositoryLike for HashMapRepository {
    type Reference = String;

    fn upsert(&mut self, id: &Uuid, rich_text: String) -> bool {
        self.0.insert(*id, rich_text).is_some()
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
