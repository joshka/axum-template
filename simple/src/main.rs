use axum::{routing::get, Router};
use cli::Cli;
use color_eyre::Result;
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
    let addr = (args.ip, args.port).into();
    let config = tls::init(args.data_dir).await?;
    let server = axum_server::bind_rustls(addr, config).handle(handle.clone());
    let app = router().into_make_service();
    let server_task = tokio::spawn(server.serve(app));
    let Some(addr) = handle.listening().await else {
        error!("failed to start server");
        return Ok(());
    };

    let addr = format!("https://{addr}");
    info!("Listening on {addr}");
    if args.open {
        webbrowser::open(&addr).ok();
    }

    server_task.await??;
    Ok(())
}

fn router() -> Router {
    Router::new().route("/", get(root))
}

async fn root() -> &'static str {
    "Hello, World!"
}
