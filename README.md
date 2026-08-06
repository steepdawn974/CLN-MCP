# Core Lightning MCP Server

<div align="center">

[![Rust Version](https://img.shields.io/badge/rust-1.80%2B-blue.svg)](https://www.rust-lang.org) &nbsp;
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE) &nbsp;
![](https://badge.mcpx.dev?type=server 'MCP Server') &nbsp;
[![CI](https://github.com/adi2011/cln-mcp/actions/workflows/ci.yml/badge.svg)](https://github.com/adi2011/cln-mcp/actions/workflows/ci.yml) &nbsp;
[![gRPC Tests](https://github.com/adi2011/cln-mcp/actions/workflows/grpc.yml/badge.svg)](https://github.com/adi2011/cln-mcp/actions/workflows/grpc.yml) &nbsp;

</div>

A Rust-based MCP (Model Context Protocol) server that provides a standardized interface to Core Lightning nodes. Supports both **REST** (rune auth, no certs needed) and **gRPC** (mTLS) backends. Enables full node management — read-only queries, channel operations, payments, and more — via LLM tools.

![MCP](./assets/mcp-screenshot.png)

## Installation

### Option 1: From Release (Recommended)

1. Download the appropriate binary for your platform from the [latest release](https://github.com/adi2011/cln-mcp/releases/latest)
2. Extract the archive:
   ```bash
   # For Linux/macOS
   tar -xzf cln-mcp-<platform>.tar.gz
   
   # For Windows
   # Use your preferred zip extractor
   ```
3. Make the binary executable (Linux/macOS only):
   ```bash
   chmod +x cln-mcp
   ```

### Option 2: From Source
## Prerequisites

- Rust 1.80 or higher
- Protocol Buffers Compiler (protoc)
- Core Lightning (with gRPC enabled)
- MCP clients ([Claude](https://claude.ai/download), [Goose](https://github.com/block/goose), etc.)

#### Protocol Buffers Compiler (protoc)
**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install -y protobuf-compiler
```

**macOS:**
```bash
brew install protobuf
```

**Windows:**
```bash
choco install protoc
```

**Verify installation:**
```bash
protoc --version  # Should show version 3.0.0 or higher
```

1. Clone the repository:
```bash
git clone https://github.com/adi2011/cln-mcp.git
cd cln-mcp
```

2. Build the project:
```bash
cargo build --release
```

## Configuration

The server supports two backends: **REST** (default, recommended) and **gRPC** (legacy).

### REST Backend (Default)

The REST backend connects to CLN's built-in [clnrest](https://docs.corelightning.org/docs/rest) plugin using a **rune** for authentication. No TLS certificates need to be copied.

**Prerequisites:**
1. Enable clnrest in your CLN config: `clnrest-port=3010`
2. Create a rune: `lightning-cli createrune`

```bash
cln-mcp --rune <your-rune>
# or with custom URL:
cln-mcp --backend rest --rest-url https://localhost:3010 --rune <your-rune>
# or with CA cert for strict TLS:
cln-mcp --rune <your-rune> --ca-cert /path/to/ca.pem
```

### gRPC Backend (Legacy)

The gRPC backend uses mTLS certificates. Requires building with `--features grpc` and having protoc installed.

```bash
cargo build --release --features grpc
cln-mcp --backend grpc --certs-dir /path/to/certs
```

### Full CLI Options

```
--backend <rest|grpc>   Backend type (default: rest)
--rest-url <url>        REST endpoint (default: https://localhost:3010)
--rune <rune>           Rune token for REST auth (required for REST)
--ca-cert <path>        Optional CA cert for REST TLS verification
--certs-dir <path>      Path to gRPC certificates directory
--node-address <url>    gRPC node address (default: https://localhost:9736)
--help                  Shows help message
```

### TLS Certificate Setup (gRPC only)
Add the `--grpc-port`(default: 9736) option while running CLN, and it'll automatically generate the appropriate mTLS certificates. 

Copy the following PEM files from the Lightning directory to a separate directory:
- `ca.pem`: CA certificate
- `client.pem`: Client certificate
- `client-key.pem`: Client private key

### Claude Setup (REST — Recommended)
 - Install [Claude](https://claude.ai/download)
 - Go to settings -> Developer
 - Edit Config
 ```
    {
        "mcpServers" : {
            "cln-mcp" : {
                "command": "/path/to/cln-mcp",
                "args": [
                    "--rune",
                    "your-rune-token-here"
                ]
            }
        }
    }
 ```
 - Restart Claude

### Claude Setup (gRPC)
 ```
    {
        "mcpServers" : {
            "cln-mcp" : {
                "command": "/path/to/cln-mcp",
                "args": [
                    "--backend",
                    "grpc",
                    "--certs-dir",
                    "/path/to/certificates"
                ]
            }
        }
    }
 ```

## Tools

### Read-Only Tools
- `get_info`, `list_configs`, `list_addresses`, `list_channels`, `list_peer_channels`, `list_closed_channels`, `list_htlcs`
- `list_pays`, `list_send_pays`, `list_forwards`, `list_invoices`, `list_peers`, `list_nodes`, `list_funds`
- `list_offers`, `list_datastore`, `feerates`, `get_log`
- `bkpr_channels_apy`, `bkpr_list_balances`, `bkpr_list_income`, `bkpr_list_account_events`
- `get_route`, `decode`, `decode_pay`, `check_message`, `list_transactions`, `show_runes`

### Node Management Tools
- `connect_peer`, `disconnect_peer`, `fund_channel`, `close_channel`, `set_channel`
- `create_invoice`, `pay_invoice`, `keysend`, `withdraw`, `new_address`
- `sign_message`, `create_rune`, `create_offer`, `disable_offer`, `fetch_invoice`

### Generic Tool
- `call_rpc_method` — Call any CLN RPC method with arbitrary parameters

## Build Features

```bash
# REST only (default, no protoc needed)
cargo build --release

# gRPC + REST (requires protoc)
cargo build --release --features grpc
```

## Testing

Integration tests require a running CLN node with clnrest enabled. Tests skip gracefully if no credentials are provided.

### Setting Up

Export the following environment variables before running tests:

```bash
# Required: rune token for REST authentication
export CLN_RUNE="your-rune-token"

# Optional: REST endpoint URL (defaults to https://localhost:3010)
export CLN_REST_URL="https://your-node-address:port"
```

### Running Tests

```bash
# Run all integration tests
cargo test --release

# Run a specific test
cargo test --release test_get_info

# Run with verbose output
cargo test --release -- --nocapture
```

Tests run sequentially to avoid overwhelming the node. Each test calls a tool against the live node and asserts the response structure matches the CLN RPC specification.

# Future Goals
 [ ] Enable it to derive parameters for the RPC calls  
 [ ] Choose the most appropriate and useful RPCs for maximum utility  
 [ ] Extend support for LND  
 [ ] Host multiple servers to make it more efficient  

This is a work in progress. We welcome code reviews, pull requests, and issues based on your usage.
