use clap::Parser;
use std::path::PathBuf;

use axum::{
    body::Body,
    extract::{Request, State},
    http::uri::Uri,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use memory_serve::{MemoryServe, load_assets};
use hyper::StatusCode;
use hyper_util::{client::legacy::connect::HttpConnector, rt::TokioExecutor};

#[derive(clap::Parser)]
#[command(version, about, name = "iotame")]
struct Cli {
    /// Sets a custom config file path
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Starts iotame
    Start {
        /// Use a separately running frontend dev server
        #[arg(short, long)]
        dev: bool,
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Start { dev } => {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
                .await
                .unwrap();
            let app: Router<_>;

            if *dev {
                let client = hyper_util::client::legacy::Client::<(), ()>::builder(TokioExecutor::new())
                    .build(HttpConnector::new());

                app = Router::new().route("/", get(dev_server_handler)).with_state(client);
            } else {
                let memory_router = MemoryServe::new(load_assets!("../frontend/dist"))
                    .index_file(Some("/index.html"))
                    .into_router();

                app = Router::new().merge(memory_router);
            }

            println!("listening on {}", listener.local_addr().unwrap());
            axum::serve(listener, app).await.unwrap();
        }
    }
}


type Client = hyper_util::client::legacy::Client<HttpConnector, Body>;

async fn dev_server_handler(State(client): State<Client>, mut req: Request) -> Result<Response, StatusCode> {
    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);

    let uri = format!("http://127.0.0.1:5173{path_query}");

    println!("{:?} -> {:?}", path_query, uri);
    *req.uri_mut() = Uri::try_from(uri).unwrap();

    Ok(client
        .request(req)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)
        .into_response()
    )
}
