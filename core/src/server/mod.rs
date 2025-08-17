use crate::StartArgs;
use axum::body::Body;
use axum::extract::{Request, State};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use http::{StatusCode, Uri};
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::TokioExecutor;
use log::info;
use memory_serve::{load_assets, MemoryServe};

type Client = hyper_util::client::legacy::Client<HttpConnector, Body>;

pub async fn start_server(args: &StartArgs) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();

    let app = if args.dev {
        let client = hyper_util::client::legacy::Client::<(), ()>::builder(TokioExecutor::new())
            .build(HttpConnector::new());

        Router::new()
            .route("/", get(dev_server_handler))
            .with_state(client)
    } else {
        let memory_router = MemoryServe::new(load_assets!("../frontend/dist"))
            .index_file(Some("/index.html"))
            .into_router();

        Router::new().merge(memory_router)
    };

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn dev_server_handler(
    State(client): State<Client>,
    mut req: Request,
) -> Result<Response, StatusCode> {
    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);

    let uri = format!("http://127.0.0.1:5173{path_query}");

    println!("{path_query:?} -> {uri:?}");
    *req.uri_mut() = Uri::try_from(uri).unwrap();

    Ok(client
        .request(req)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)
        .into_response())
}
