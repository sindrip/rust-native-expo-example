use axum::{Router, extract::Path, routing::get};

pub fn router() -> Router {
    Router::new().route("/add/{a}/{b}", get(add))
}

async fn add(Path((a, b)): Path<(u64, u64)>) -> String {
    app_core::add(a, b).to_string()
}
