use serde::{Deserialize, Serialize};

use crate::reference::reference_style::{Reference, ReferenceStyle};

pub mod post;

#[derive(Deserialize)]
pub struct FormatRequest {
    reference: Reference,
    style: ReferenceStyle,
}

#[derive(Serialize)]
pub struct FormatResponse {
    formatted_reference: String,
}
