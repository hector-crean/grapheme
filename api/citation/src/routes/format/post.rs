use axum::Json;

use crate::reference::reference_style::OutputFormat;

use super::*;

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn format_reference(Json(payload): Json<FormatRequest>) -> Json<FormatResponse> {
    let formatted = payload.reference.format(payload.style, OutputFormat::HTML);
    Json(FormatResponse {
        formatted_reference: formatted,
    })
}
