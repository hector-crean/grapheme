pub mod hashmap;
use uuid::Uuid;

use crate::rich_text::RichTextLike;

// Define the RichText struct

// Define the RichTextRepository trait
pub trait RichTextRepositoryLike: Send + Sync {
    type RichText: RichTextLike;
    fn get(&mut self, id: &Uuid) -> Option<Self::RichText>;
    fn insert(&mut self, id: &Uuid, rich_text: Self::RichText) -> bool;
    fn update(&mut self, id: &Uuid, rich_text: Self::RichText) -> bool;
    fn delete(&mut self, id: &Uuid) -> bool;
    fn list(&mut self) -> Vec<(Uuid, Self::RichText)>;
}
