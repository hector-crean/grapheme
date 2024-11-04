pub mod hashmap;
use uuid::Uuid;

use crate::reference::ReferenceLike;

// Define the RichText struct

// Define the RichTextRepository trait
pub trait ReferenceRepositoryLike: Send + Sync {
    type Reference: ReferenceLike;
    fn get(&mut self, id: &Uuid) -> Option<Self::Reference>;
    fn upsert(&mut self, id: &Uuid, rich_text: Self::Reference) -> bool;
    fn delete(&mut self, id: &Uuid) -> bool;
    fn list(&mut self) -> Vec<(Uuid, Self::Reference)>;
}
