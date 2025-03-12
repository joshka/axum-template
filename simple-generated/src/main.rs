use std::{io, time::Duration};

use axum::{routing::get, Router};
use cli::Cli;
use color_eyre::Result;
use tokio::{signal::ctrl_c, task::JoinHandle};
use tracing::{error, info};

mod cli;
mod tls;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::args();
    color_eyre::install()?;
    tracing_subscriber::fmt::fmt()
        .with_max_level(args.verbosity)
        .init();
    run(args).await
}

async fn run(args: Cli) -> Result<()> {
    let handle = axum_server::Handle::new();
    let server_task = start_service(&args, handle.clone()).await?;

    let Some(addr) = handle.listening().await else {
        error!("failed to start server");
        return Ok(());
    };

    let addr = format!("https://{addr}");
    info!("Listening on {addr}");
    if args.open {
        webbrowser::open(&addr).ok();
    }

    shutdown_signal().await;
    handle.graceful_shutdown(Some(Duration::from_secs(10)));

    server_task.await??;
    Ok(())
}

async fn start_service(
    args: &Cli,
    handle: axum_server::Handle,
) -> Result<JoinHandle<io::Result<()>>> {
    let addr = (args.ip, args.port).into();
    let config = tls::init(&args.data_dir).await?;
    let server = axum_server::bind_rustls(addr, config).handle(handle);
    let app = router().into_make_service();
    let server_task = tokio::spawn(server.serve(app));
    Ok(server_task)
}

async fn shutdown_signal() {
    match ctrl_c().await {
        Ok(()) => info!("Received Ctrl-C"),
        Err(err) => error!("Failed to listen for Ctrl-C: {err}"),
    }
    info!("Shutting down");
}

fn router() -> Router {
    Router::new().route("/", get(root))
}

async fn root() -> &'static str {
    "Hello, World!"
}
