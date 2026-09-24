# CLN-MCP Agent Guidelines

## MCP Server Architecture

The CLN-MCP server connects to a Core Lightning node via the clnrest HTTP plugin
(REST backend) or cln-grpc (gRPC backend). The REST backend is the default and
supports all RPC methods via `call_rpc_method`. The gRPC backend has a fixed set
of typed methods and rejects unknown methods.

## Tool Usage — Prefer Specialized Tools

The server exposes ~50 purpose-built tools (`get_info`, `list_funds`,
`list_forwards`, `list_peer_channels`, `list_nodes`, `list_channels`,
`list_invoices`, `close_channel`, `fund_channel`, `connect_peer`,
`disconnect_peer`, `channel_summary`, `node_health`, `fee_report`, ...).
**Use them first** — they aggregate multiple RPCs into a single view and are
the intended interface (e.g. `channel_summary` combines listpeerchannels +
listpeers + listforwards; `node_health` combines getinfo + feerates +
channel states).

`call_rpc_method` is the escape hatch only, for:
- RPCs with no tool wrapper (e.g. `sql`, `listconfigs` filters, `decode`)
- Parameter combinations the tool schema does not expose
- Raw fields a specialized tool omits

Do not dump unfiltered `listinvoices`/`listforwards`/`listpays` — they return
huge payloads. Filter via params (`in_channel`, `status`, `label`) or use
`sql` for targeted queries.

Note: backend/clnrest failures are returned as tool results with
`isError: true`, not protocol errors — a tool error does not mean the MCP
connection is dead.

## Forbidden RPC Methods via MCP

### `wait` — NEVER call via MCP

CLN's `wait` RPC is a long-polling command designed for persistent JSON-RPC
socket connections. When called through clnrest's HTTP API, the response returns
but the notification subscription is not cleaned up. This leaves a dangling
subscriber holding a connection slot in clnrest, causing intermittent
`Failed to connect to MCP server` errors.

**Do not call `wait` through `call_rpc_method` or any MCP tool.**

To get the latest forward index (the common reason for calling `wait`), use
one of these alternatives:

```
call_rpc_method(method: "listforwards", params: {"index": "created", "start": 999999, "limit": 1})
call_rpc_method(method: "sql", params: {"query": "SELECT MAX(created_index) FROM forwards"})
```

If `wait` was already called and the MCP server is intermittently failing,
restart the clnrest plugin on the node.

### Other long-polling / streaming methods to avoid

- `wait` — long-polling, leaves dangling subscribers
- `waitanyinvoice` — long-polling
- `waitinvoice` — long-polling
- `waitblockheight` — long-polling
- `sendcustommsg` — may block on peer response

## Sensitive Data

- Never print or commit credentials: runes, API tokens, node private keys,
  or any secret values from node configuration files.
- Rune IDs are not secret; full rune strings are.

## Local Operations

Node-specific operational details (hosts, paths, monitoring, channel policy)
live in `AGENTS.local.md`, which is gitignored and must never be committed or
pushed upstream.
