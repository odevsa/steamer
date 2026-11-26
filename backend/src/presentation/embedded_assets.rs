use axum::body::Body;
use axum::http::{header, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "../frontend/build"]
struct EmbeddedFrontend;

pub async fn serve_embedded_spa(uri: Uri) -> Response {
    let raw_path = uri.path().trim_start_matches('/');
    if raw_path == "api" || raw_path.starts_with("api/") {
        return StatusCode::NOT_FOUND.into_response();
    }

    if !raw_path.is_empty() {
        if let Some(asset) = EmbeddedFrontend::get(raw_path) {
            return asset_response(raw_path, asset.data.into_owned());
        }
    }

    match EmbeddedFrontend::get("index.html") {
        Some(index) => asset_response("index.html", index.data.into_owned()),
        None => (
            StatusCode::NOT_FOUND,
            "Frontend bundle not embedded. Run frontend build or use Vite dev server.",
        )
            .into_response(),
    }
}

fn asset_response(path: &str, bytes: Vec<u8>) -> Response {
    let mime = mime_guess::from_path(path).first_or_octet_stream();
    let cache_control = if path.starts_with("_app/immutable/") {
        "public, max-age=31536000, immutable"
    } else {
        "no-cache"
    };

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime.as_ref())
        .header(header::CACHE_CONTROL, cache_control)
        .body(Body::from(bytes))
        .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())
}
