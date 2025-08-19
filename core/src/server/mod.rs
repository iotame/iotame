use crate::StartArgs;
use axum_reverse_proxy::ReverseProxy;
use axum::Router;
use log::info;
use memory_serve::{load_assets, MemoryServe};

pub async fn start_server(args: &StartArgs) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();

    let app = if args.dev {
        let proxy = ReverseProxy::new("/", "http://localhost:5173");
        proxy.into()
    } else {
        let memory_router = MemoryServe::new(load_assets!("../frontend/dist"))
            .index_file(Some("/index.html"))
            .into_router();

        Router::new().merge(memory_router)
    };

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
