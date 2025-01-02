use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{routing::get, Router};
use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let args = Cli::parse();
    color_eyre::install()?;
    tracing_subscriber::fmt::fmt()
        .with_max_level(args.verbosity)
        .init();

    let listener = TcpListener::bind(args.addr()).await?;
    let addr = format!("http://{}", listener.local_addr()?);
    info!("Listening on: {addr}");
    if args.open {
        webbrowser::open(&addr)?;
    }
    axum::serve(listener, router()).await?;
    Ok(())
}

#[derive(Debug, Parser)]
struct Cli {
    /// Verbosity flags (--verbose / --quiet) for logging
    #[command(flatten)]
    verbosity: Verbosity<InfoLevel>,

    /// The IP address to bind to
    #[arg(long, short, default_value_t = Ipv4Addr::LOCALHOST.into())]
    bind: IpAddr,

    /// The port to bind to
    #[arg(long, short, default_value = "3000")]
    port: u16,

    /// Open the browser after starting the server
    #[arg(long)]
    open: bool,
}

impl Cli {
    pub fn addr(&self) -> SocketAddr {
        (self.bind, self.port).into()
    }
}

fn router() -> Router {
    Router::new().route("/", get(root))
}

async fn root() -> &'static str {
    "Hello, World!"
}
