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
    info!("Listening on: http://{}", listener.local_addr()?);
    axum::serve(listener, router()).await?;
    Ok(())
}

#[derive(Debug, Parser)]
struct Cli {
    #[command(flatten)]
    verbosity: Verbosity<InfoLevel>,

    #[arg(long, short, default_value_t = Ipv4Addr::LOCALHOST.into())]
    bind: IpAddr,

    #[arg(long, short, default_value = "3000")]
    port: u16,
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
