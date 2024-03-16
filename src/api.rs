use super::{sql, Args};
use axum::{
    extract::{Path, State},
    response::Json,
    routing::get,
    Router, debug_handler, http::StatusCode,
};
use axum::extract::Query;
use serde::Deserialize;

#[derive(Debug, Clone)]
struct AppState {
    database: sql::Database,
}

pub async fn start(args: &Args, database: sql::Database) -> anyhow::Result<()> {
    let state = AppState { database };
    let router = Router::new()
        .route("/", get(|| async { "API is alive and running!" }))
        .route("/api/get_near/", get(get_near))
        .route("/api/info/:id", get(info))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind((args.address.clone(), args.port)).await?;

    println!("Listening on http://{:?}", listener.local_addr()?);
    axum::serve(listener, router).await?;

    Ok(())
}

#[derive(Deserialize, Debug)]
struct Bounds {
    ne_lat: f64,
    ne_lng: f64,
    sw_lat: f64,
    sw_lng: f64,
}

async fn get_near(State(state): State<AppState>, Query(query): Query<Bounds>) -> Result<Json<Vec<sql::Establishment>>, StatusCode> {
    todo!("get_near")
}

#[debug_handler]
async fn info(State(state): State<AppState>, Path(path): Path<String>) -> Result<Json<sql::Establishment>, StatusCode> {
    match state.database.get_establishment(&path).await {
        Ok(Some(establishment)) => Ok(Json(establishment)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(err) => {
            eprintln!("Error in `/api/info/{}`: {}", path, err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}
