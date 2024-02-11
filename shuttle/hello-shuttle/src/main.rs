use axum::{response::IntoResponse, routing::get, Router}; 

async fn hello_world() -> impl IntoResponse {
    "Hello world"
} 

#[shuttle_runtime::main]
async fn shuttle() -> shuttle_axum::ShuttleAxum {
    let app = Router::new().route("/", get(hello_world));
    Ok(app.into())
}
