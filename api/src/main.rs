use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
  };
  use serde::{Deserialize, Serialize};
  use std::net::SocketAddr;

  #[tokio::main]
  async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/users", post(create_user));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
  }

  async fn create_user(
    Json(payload): Json<CreateUser>,
  ) -> impl IntoResponse {
    let user = User {
        id: payload.id,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
  }

  #[derive(Deserialize)]
  struct CreateUser {
    id: u64,
    username: String,
  }

  #[derive(Serialize)]
  struct User {
    id: u64,
    username: String,
  }