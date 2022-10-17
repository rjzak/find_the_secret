use anyhow::anyhow;
use axum::extract::{Extension, Path};
use axum::routing::get;
use axum::Router;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone, Debug)]
struct State {
    secret: String,
}

impl State {
    fn new() -> Self {
        Self {
            secret: Uuid::new_v4().as_hyphenated().to_string(),
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let state = State::new();

    #[cfg(debug_assertions)]
    eprintln!("Secret: {}", state.secret);

    #[cfg(not(target_os = "wasi"))]
    {
        use std::net::SocketAddr;
        let addr: SocketAddr = "0.0.0.0:8080"
            .parse()
            .expect("Unable to parse socket address");
        axum::Server::bind(&addr)
            .serve(app(state).into_make_service())
            .await
            .map_err(|e| anyhow!(e))?;
    }
    #[cfg(target_os = "wasi")]
    {
        use std::os::wasi::io::FromRawFd;
        let std_listener = unsafe { std::net::TcpListener::from_raw_fd(3) };
        std_listener.set_nonblocking(true).map_err(|e| anyhow!(e))?;
        axum::Server::from_tcp(std_listener)
            .map_err(|e| anyhow!(e))?
            .serve(app(state).into_make_service())
            .await
            .map_err(|e| anyhow!(e))?;
    }
    Ok(())
}

fn app(state: State) -> Router {
    Router::new()
        .route("/", get(greeting))
        .route("/:secret", get(check_secret))
        .layer(Extension(Arc::new(state)))
}

async fn greeting() -> Vec<u8> {
    "Hello!".to_string().as_bytes().to_vec()
}

async fn check_secret(
    Path(secret): Path<String>,
    Extension(state): Extension<Arc<State>>,
) -> Vec<u8> {
    if state.secret.to_string() == secret {
        "Secret validated!".to_string().as_bytes().to_vec()
    } else {
        "The secret is still safe!".to_string().as_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use crate::{app, State};
    use axum::http::Request;
    use hyper::Body;
    use tower::ServiceExt; // for `app.oneshot()`

    #[tokio::test]
    async fn test_successful_secret_validation() {
        let state = State::new();
        let request = Request::builder()
            .method("GET")
            .uri(format!("/{}", state.secret))
            .body(Body::from(vec![]))
            .unwrap();

        let response = app(state.clone()).oneshot(request).await.unwrap();
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(
            body.to_vec(),
            "Secret validated!".to_string().as_bytes().to_vec()
        );
    }

    #[tokio::test]
    async fn test_unsuccessful_secret_validation() {
        let state = State::new();
        let request = Request::builder()
            .method("GET")
            .uri(format!("/a{}b", state.secret))
            .body(Body::from(vec![]))
            .unwrap();

        let response = app(state.clone()).oneshot(request).await.unwrap();
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(
            body.to_vec(),
            "The secret is still safe!".to_string().as_bytes().to_vec()
        );
    }
}
