mod client;
mod error;
mod utils;

use crate::client::backend::ClnBackend;
use crate::client::node::NodeService;
use crate::client::rest::RestBackend;
use anyhow::{anyhow, Result};
use rmcp::{transport::stdio, ServiceExt};
use std::env;
use std::sync::Arc;
use tracing::{error, info};
use tracing_subscriber::{self, EnvFilter};

fn print_usage() {
    println!("Usage: cln-mcp [OPTIONS]");
    println!();
    println!("Options:");
    println!("  --backend <rest|grpc>   Backend type (default: rest)");
    println!();
    println!("  REST backend options:");
    println!("    --rest-url <url>      REST endpoint (default: https://localhost:3010)");
    println!("    --rune <rune>         Rune token for REST auth (required for REST)");
    println!("    --ca-cert <path>      Optional CA cert for REST TLS verification");
    println!();
    println!("  gRPC backend options:");
    println!("    --certs-dir <path>    Path to gRPC certificates directory");
    println!("    --node-address <url>  gRPC node address (default: https://localhost:9736)");
    println!();
    println!("  --help                  Shows this help message");
}

#[allow(dead_code)]
struct Args {
    backend: String,
    rest_url: String,
    rune: Option<String>,
    ca_cert: Option<String>,
    certs_dir: Option<String>,
    node_address: String,
}

fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();
    let mut backend = "rest".to_string();
    let mut rest_url = "https://localhost:3010".to_string();
    let mut rune = None;
    let mut ca_cert = None;
    let mut certs_dir = None;
    let mut node_address = "https://localhost:9736".to_string();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--backend" => {
                if i + 1 < args.len() {
                    backend = args[i + 1].clone();
                    if backend != "rest" && backend != "grpc" {
                        error!("Error: --backend must be 'rest' or 'grpc'");
                        std::process::exit(1);
                    }
                    i += 2;
                } else {
                    error!("Error: --backend requires a value");
                    std::process::exit(1);
                }
            }
            "--rest-url" => {
                if i + 1 < args.len() {
                    rest_url = args[i + 1].clone();
                    i += 2;
                } else {
                    error!("Error: --rest-url requires a URL");
                    std::process::exit(1);
                }
            }
            "--rune" => {
                if i + 1 < args.len() {
                    rune = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    error!("Error: --rune requires a rune token");
                    std::process::exit(1);
                }
            }
            "--ca-cert" => {
                if i + 1 < args.len() {
                    ca_cert = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    error!("Error: --ca-cert requires a path");
                    std::process::exit(1);
                }
            }
            "--certs-dir" => {
                if i + 1 < args.len() {
                    certs_dir = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    error!("Error: --certs-dir requires a path");
                    std::process::exit(1);
                }
            }
            "--node-address" => {
                if i + 1 < args.len() {
                    node_address = args[i + 1].clone();
                    i += 2;
                } else {
                    error!("Error: --node-address requires a URL");
                    std::process::exit(1);
                }
            }
            "--help" => {
                print_usage();
                std::process::exit(0);
            }
            _ => {
                error!("Unknown argument: {}", args[i]);
                print_usage();
                std::process::exit(1);
            }
        }
    }

    Args {
        backend,
        rest_url,
        rune,
        ca_cert,
        certs_dir,
        node_address,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug")),
        )
        .init();

    info!("CLN MCP server initiated!");

    let args = parse_args();

    let backend: Arc<dyn ClnBackend> = match args.backend.as_str() {
        "rest" => {
            let rune = args.rune.ok_or_else(|| {
                anyhow!("--rune is required for REST backend. Create one with: lightning-cli createrune")
            })?;
            info!("Using REST backend at {}", args.rest_url);
            let rest = RestBackend::new(&args.rest_url, &rune, args.ca_cert.as_deref()).await?;
            Arc::new(rest)
        }
        "grpc" => {
            #[cfg(feature = "grpc")]
            {
                use crate::client::config::{create_channel, ClientConfig};
                use crate::client::grpc::GrpcBackend;
                use crate::utils::tls::load_tls_config;
                use std::time::Duration;

                info!("Using gRPC backend at {}", args.node_address);
                let tls_config = load_tls_config(args.certs_dir).await?;
                debug!("TLS certificate loaded!");

                let config = ClientConfig::new(
                    args.node_address,
                    Duration::from_secs(1),
                    Duration::from_secs(5),
                );

                let channel = create_channel(&config)?
                    .tls_config(tls_config)?
                    .connect_lazy();

                Arc::new(GrpcBackend::new(channel))
            }
            #[cfg(not(feature = "grpc"))]
            {
                return Err(anyhow!(
                    "gRPC backend requires building with --features grpc"
                ));
            }
        }
        _ => return Err(anyhow!("Unknown backend: {}", args.backend)),
    };

    info!("--------------------Server Started Running!--------------------------");

    let service = NodeService::new(backend)
        .serve(stdio())
        .await
        .inspect_err(|e| {
            println!("Error starting server: {e}");
        })?;

    service.waiting().await?;
    Ok(())
}

