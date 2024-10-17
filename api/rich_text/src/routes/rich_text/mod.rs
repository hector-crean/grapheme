pub mod post {

    use crate::Server;

    use axum::{extract::State, Json};
    use log::info;
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Deserialize, Serialize, Debug)]
    pub struct RichTextRequest {
        pub id: Uuid,
        pub rich_text: String,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct RichTextResponse {
        success: bool,
    }

    pub async fn handle_post_rich_text(
        State(state): State<Server>,
        Json(payload): Json<RichTextRequest>,
    ) -> Json<RichTextResponse> {
        info!(
            "Handling POST request for rich text with id: {}",
            payload.id
        );

        let mut db = state.db.lock().unwrap();
        let success = db.upsert(&payload.id, payload.rich_text);

        Json(RichTextResponse { success })
    }
}

pub mod get {

    use crate::Server;

    use axum::{
        extract::{Path, Query, State},
        Json,
    };
    use log::{info, warn};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Deserialize, Serialize, Debug)]
    pub struct RichTextQuery {
        pub id: Uuid,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct RichTextResponse {
        rich_text: String,
    }

    #[axum::debug_handler]
    pub async fn handle_get_rich_text(
        State(state): State<Server>,
        Path(query): Path<RichTextQuery>,
    ) -> Json<RichTextResponse> {
        info!("Handling GET request for rich text with id: {}", query.id);
        let rich_text = state.db.lock().unwrap().get(&query.id);

        match rich_text {
            Some(rich_text) => {
                info!("Rich text found for id: {}", query.id);
                Json(RichTextResponse { rich_text })
            }
            None => {
                warn!("Rich text not found for id: {}", query.id);
                Json(RichTextResponse {
                    rich_text: "".to_string(),
                })
            }
        }
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct ListRichTextResponse {
        rich_texts: Vec<(Uuid, String)>,
    }
    #[axum::debug_handler]
    pub async fn handle_list_rich_text(State(state): State<Server>) -> Json<ListRichTextResponse> {
        info!("Handling GET request for list of rich text");
        let rich_texts = state.db.lock().unwrap().list();
        Json(ListRichTextResponse { rich_texts })
    }
}
