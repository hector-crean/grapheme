use serde::{Deserialize, Serialize};

pub trait RichTextLike: ToString + Serialize + for<'de> Deserialize<'de> {}

impl RichTextLike for String {}
