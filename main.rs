use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket, Message},
    extract::Path,
    routing::get,
    response::{IntoResponse, Response},
    http::*,
    body,
    body::*,
    Router,
};
use include_dir::{include_dir, Dir};
use rand::Rng;

use std::net::SocketAddr;
use std::env;

static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");

async fn handler(ws: WebSocketUpgrade) -> Response {
    println!("Rust: Socket Connected!");
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(mut msg) = msg {
            if msg.to_text().unwrap() == "rand" {
                msg = Message::Text(format!("rand {}", rand::thread_rng().gen_range(0..100)));
                msg
            }
            else {
                msg
            }
        } else {
            // client disconnected
            return;
        };

        if socket.send(msg).await.is_err() {
            // client disconnected
            return;
        }
    }
}

async fn static_path(Path(path): Path<String>) -> impl IntoResponse {
    let path = path.trim_start_matches('/');
    let mime_type = mime_guess::from_path(path).first_or_text_plain();

    match STATIC_DIR.get_file(path) {
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new()))
            .unwrap(),
        Some(file) => Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(mime_type.as_ref()).unwrap(),
            )
            .body(body::boxed(Full::from(file.contents())))
            .unwrap(),
    }
}

async fn index() -> impl IntoResponse {
    return static_path(Path("/index.html".to_string())).await;
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/static/*path", get(static_path))
        .route("/ws", get(handler));
    let addr = SocketAddr::from(([0, 0, 0, 0], env::var("PORT").unwrap_or("3000".to_string()).parse::<u16>().unwrap()));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
