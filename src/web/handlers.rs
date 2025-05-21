//! u8bf7u6c42u5904u7406u7a0b

use axum::response::{IntoResponse, Response, Html};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use std::collections::HashMap;
use std::sync::Arc;

use crate::docs::DocRegistry;
use crate::core::config::Config;

/// u52a8u6001u72b6u6001
pub struct AppState {
    pub config: Config,
    pub doc_registry: Arc<DocRegistry>,
}

/// u9996u9875
pub async fn index() -> Html<&'static str> {
    Html(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>xwdoc</title>
    <style>
        body { font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; line-height: 1.6; margin: 0; padding: 20px; color: #333; }
        h1 { margin-top: 0; }
        a { color: #3490dc; text-decoration: none; }
        a:hover { text-decoration: underline; }
    </style>
</head>
<body>
    <h1>xwdoc</h1>
    <p>Welcome to the Rust implementation of xwdoc API Documentation Browser.</p>
    <p>This is a work in progress. Features:</p>
    <ul>
        <li><a href="/docs.json">Documentation list</a></li>
        <li><a href="/search?q=html">Search</a></li>
    </ul>
</body>
</html>
"#)
}

/// u5fc3u8df3u68c0u6d4b
pub async fn ping() -> &'static str {
    "pong"
}

/// u641cu7d22
pub async fn search(
    State(_state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>
) -> Response {
    // u6240u6709u6587u6863u641cu7d22
    let query = params.get("q").cloned().unwrap_or_default();
    if query.is_empty() {
        return (StatusCode::BAD_REQUEST, "Missing query parameter 'q'").into_response();
    }

    // u5728u771fu6b63u5b9eu73b0u4e2d uff0cu8fd9u91ccu4f1au641cu7d22u6587u6863u7d22u5f15
    let results: Vec<serde_json::Value> = vec![]; // u5360u4f4du7b26

    Json(results).into_response()
}

/// u83b7u53d6u6240u6709u6587u6863u5217u8868
pub async fn docs_list(State(_state): State<Arc<AppState>>) -> Response {
    // u8fd4u56deu6240u6709u53efu7528u6587u6863u7684u5217u8868
    let docs = _state.doc_registry.all();

    // u5c06u6587u6863u8f6cu6362u4e3au53efu5e8fu5217u5316u7684u683cu5f0f
    let result: Vec<serde_json::Value> = docs.iter().map(|doc| {
        serde_json::json!({
            "name": doc.name,
            "slug": doc.slug,
            "version": doc.version,
            "release": doc.release,
            "mtime": doc.mtime,
            "db_size": doc.db_size,
            "index_size": doc.index_size
        })
    }).collect();

    Json(result).into_response()
}

/// u83b7u53d6u7279u5b9au6587u6863u7684u7d22u5f15
pub async fn doc_index(
    State(state): State<Arc<AppState>>,
    Path(doc_slug): Path<String>
) -> Response {
    // u68c0u67e5u6587u6863u662fu5426u5b58u5728
    match state.doc_registry.find(&doc_slug) {
        Some(_doc) => {
            // u5728u771fu6b63u5b9eu73b0u4e2du FF0Cu4f1au8fd4u56deu6587u6863u7d22u5f15
            let index = serde_json::json!({
                "entries": [],
                "types": []
            });

            Json(index).into_response()
        },
        None => {
            (StatusCode::NOT_FOUND, format!("Documentation '{}' not found", doc_slug)).into_response()
        }
    }
}

/// u83b7u53d6u7279u5b9au6587u6863u7684u9875u9762
pub async fn doc_page(
    State(state): State<Arc<AppState>>,
    Path((doc_slug, page_path)): Path<(String, String)>
) -> Response {
    // u68c0u67e5u6587u6863u662fu5426u5b58u5728
    match state.doc_registry.find(&doc_slug) {
        Some(doc) => {
            // u5728u771fu6b63u5b9eu73b0u4e2du FF0Cu4f1au8bbfu95eeu5e76u8fd4u56deu7279u5b9au9875u9762
            let content = format!("<h1>Page: {}</h1><p>From documentation: {}</p>", page_path, doc.name);

            Html(content).into_response()
        },
        None => {
            (StatusCode::NOT_FOUND, format!("Documentation '{}' not found", doc_slug)).into_response()
        }
    }
}