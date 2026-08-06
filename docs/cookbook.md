# CLN Node Runner Cookbook

Practical recipes for operating a Core Lightning node via the CLN-MCP tools.
Each recipe shows the MCP tool call, the equivalent `lightning-cli` command,
and an explanation of what's happening.

---

## Table of Contents

1. [Node Health & Monitoring](#1-node-health--monitoring)
2. [Channel Management](#2-channel-management)
3. [Fee Tuning](#3-fee-tuning)
4. [Sending Payments](#4-sending-payments)
5. [Receiving Payments](#5-receiving-payments)
6. [On-Chain Funds](#6-on-chain-funds)
7. [Askrene Layers: Routing Control](#7-askrene-layers-routing-control)
8. [Splicing: Rebalancing Channels](#8-splicing-rebalancing-channels)
9. [Security: Runes](#9-security-runes)
10. [Troubleshooting](#10-troubleshooting)

---

## 1. Node Health & Monitoring

### 1.1 Check Node Status

```
MCP tool: get_info
```
```bash
lightning-cli getinfo
```

Returns node id, alias, version, blockheight, peer count, active channels,
and total fees collected. Check this first to confirm the node is synced
and connected.

### 1.2 Full Health Dashboard

```
MCP tool: node_health
```

Composite tool that combines `getinfo` + `feerates` + `listpeerchannels`
into a single view: version, blockheight, network, peer/channel counts,
pending HTLCs, fees collected, and current fee rates.

### 1.3 Check Fee Rates

```
MCP tool: feerates(style: "perkb")
```
```bash
lightning-cli feerates perkb
```

Returns fee estimates in satoshis per 1000 virtual bytes. Use `perkw`
for weight-based units. Compare these with your channel fee settings
to ensure you're not setting fees too high or too low relative to
on-chain conditions.

### 1.4 View Node Logs

```
MCP tool: get_log(level: "unusual")
```
```bash
lightning-cli getlog unusual
```

Levels: `broken`, `unusual`, `info`, `debug`, `trace`, `io`. Use
`unusual` to see only warnings and errors. Use `broken` for critical
failures only.

### 1.5 Channel Summary Dashboard

```
MCP tool: channel_summary
```

Composite tool combining `listpeerchannels` + `listpeers` + `listforwards`
into a per-channel view showing balances, fee rates, forwarding counts,
fee revenue, and offline peers.

### 1.6 Fee Revenue Report

```
MCP tool: fee_report
```

Composite tool combining `listforwards` (revenue) + `listpeerchannels`
(fee config). Shows total fees earned, total forwards, and per-channel
breakdown of fee settings vs actual forwarding revenue.

---

## 2. Channel Management

### 2.1 Open a Channel

```
MCP tool: connect_peer(id: "pubkey@host:port")
MCP tool: fund_channel(id: "pubkey", amount: 500000)
```
```bash
lightning-cli connect pubkey@host:port
lightning-cli fundchannel pubkey 500000
```

First connect to the peer, then open the channel. The amount is in
satoshis. Optional parameters:
- `feerate`: custom fee rate for the funding transaction
- `announce: false`: open a private (unannounced) channel
- `mindepth`: require more than 1 confirmation
- `close_to`: specify a cooperative close address

### 2.2 Close a Channel (Cooperative)

```
MCP tool: close_channel(id: "pubkey_or_channel_id")
```
```bash
lightning-cli close pubkey
```

Initiates a cooperative close. The node will negotiate fees with the
peer and close the channel. This can take time if the peer is offline.

### 2.3 Force-Close a Channel (Unilateral)

```
MCP tool: close_channel(id: "pubkey", unilateraltimeout: 30)
```
```bash
lightning-cli close -k id=pubkey unilateraltimeout=30
```

After `unilateraltimeout` seconds with no peer response, the node will
unilaterally close the channel. Your funds will be time-locked for the
CSV delay (typically 144 blocks ~ 1 day) before returning to your wallet.

**Important:** `unilateraltimeout: 0` means wait indefinitely (not
immediate force-close). Omit the parameter entirely for a standard
cooperative close attempt.

### 2.4 List All Channels

```
MCP tool: list_peer_channels()
```
```bash
lightning-cli listpeerchannels
```

Shows channels with directly connected peers, including state, balances,
fee settings, and HTLCs. Filter by `id`, `short_channel_id`, or
`channel_id` for specific channels.

### 2.5 View Closed Channels

```
MCP tool: list_closed_channels()
```
```bash
lightning-cli listclosedchannels
```

Shows historically closed channels with close cause, final balances,
and commitment transaction details. Filter by peer `id` to see channels
with a specific peer.

### 2.6 Check Channel Balances

```
MCP tool: list_funds()
```
```bash
lightning-cli listfunds
```

Shows all on-chain UTXOs and channel balances. Use `spent: true` to
include already-spent outputs for accounting purposes.

---

## 3. Fee Tuning

### 3.1 Set Channel Fees

```
MCP tool: set_channel(id: "pubkey", feebase: 1000, feeppm: 50)
```
```bash
lightning-cli setchannel -k id=pubkey feebase=1000 feeppm=50
```

- `feebase`: base fee in millisatoshi (added to every HTLC)
- `feeppm`: proportional fee in parts per million (0.001% = 10 ppm)
- Use `id: "all"` to update all channels at once

### 3.2 Set HTLC Limits

```
MCP tool: set_channel(id: "pubkey", htlcmin: 1000, htlcmax: 500000000)
```
```bash
lightning-cli setchannel -k id=pubkey htlcmin=1000 htlcmax=500000000
```

`htlcmin` rejects tiny HTLCs that aren't worth the fee overhead.
`htlcmax` caps the maximum HTLC you'll forward, limiting exposure
on a single payment.

### 3.3 Analyze Forwarding Performance

```
MCP tool: list_forwards(status: "settled")
```
```bash
lightning-cli listforwards -k status=settled
```

See which channels are actually forwarding. Compare with `fee_report`
to identify channels that earn revenue vs channels that are dead weight.
Adjust fees up on high-traffic channels, down on idle ones.

Use `index: "updated"` with `start` and `limit` for pagination of
large forward histories.

---

## 4. Sending Payments

### 4.1 Pay a BOLT11 Invoice

```
MCP tool: pay_invoice(bolt11: "lnbc...")
```
```bash
lightning-cli pay lnbc...
```

Simple payment of a standard invoice. Optional `amount_msat` for
zero-amount invoices, and `label` for tracking.

### 4.2 Pay with xpay (Recommended)

```
MCP tool: xpay(invstring: "lnbc...")
```
```bash
lightning-cli xpay lnbc...
```

`xpay` is the modern payment command (v24.11+). It's smarter than
`pay` — it supports BOLT12 offers, BIP353 names, askrene layers for
routing control, and multi-part payments by default.

Key options:
- `maxfee`: absolute fee limit in msat (default: 5000 msat or 1%)
- `retry_for`: seconds to keep retrying (default: 60)
- `partial_msat`: pay only part of the invoice
- `layers`: apply askrene layers for routing control (see §7)

### 4.3 Pay with Routing Constraints

```
MCP tool: xpay(invstring: "lnbc...", maxfee: 2000, layers: ["avoid_expensive"])
```
```bash
lightning-cli xpay -k invstring=lnbc... maxfee=2000 layers='["avoid_expensive"]'
```

First create a layer that biases against expensive channels (see §7.4),
then reference it when paying. `maxfee` sets an absolute cap.

### 4.4 Keysend (Spontaneous Payment)

```
MCP tool: keysend(destination: "pubkey", amount_msat: 100000)
```
```bash
lightning-cli keysend -k destination=pubkey amount_msat=100000
```

Send funds without an invoice. The recipient must support keysend.
Optional `label` for tracking.

### 4.5 Check Payment Status

```
MCP tool: list_pays(status: "pending")
MCP tool: list_send_pays(status: "complete")
```
```bash
lightning-cli listpays -k status=pending
lightning-cli listsendpays -k status=complete
```

`listpays` gives a high-level view. `listsendpays` shows individual
payment parts (useful for debugging multi-part payments).

---

## 5. Receiving Payments

### 5.1 Create a BOLT11 Invoice

```
MCP tool: create_invoice(amount_msat: "100000", label: "order-123", description: "Coffee payment")
```
```bash
lightning-cli invoice 100000sat order-123 "Coffee payment"
```

`amount_msat` can be a number, or `"any"` for a zero-amount invoice
(let the payer decide). `label` must be unique. `expiry` defaults to
3600 seconds (1 hour).

### 5.2 Create a BOLT12 Offer

```
MCP tool: create_offer(description: "Donations", amount: "100000")
```
```bash
lightning-cli offer 100000sat "Donations"
```

Offers are reusable payment requests (like a static address). The
recipient fetches an invoice from the offer each time. Omit `amount`
for any-amount offers. Optional `label` for tracking.

### 5.3 Fetch an Invoice from an Offer

```
MCP tool: fetch_invoice(offer: "lno1...")
```
```bash
lightning-cli fetchinvoice lno1...
```

Fetches a specific invoice from a BOLT12 offer. For any-amount offers,
specify `amount_msat`. Use `quantity` for multi-quantity offers.

### 5.4 Disable an Offer

```
MCP tool: disable_offer(offer_id: "...")
```
```bash
lightning-cli disableoffer offer_id
```

Stops a offer from issuing new invoices. Existing invoices remain valid
until they expire.

### 5.5 List All Offers

```
MCP tool: list_offers(active_only: true)
```
```bash
lightning-cli listoffers -k active_only=true
```

Filter by `offer_id` for a specific offer, or `active_only: true` to
exclude disabled offers.

---

## 6. On-Chain Funds

### 6.1 Generate a Deposit Address

```
MCP tool: new_address(addresstype: "bech32")
```
```bash
lightning-cli newaddr bech32
```

Types: `bech32` (P2WPKH, segwit) or `p2tr` (taproot). Funds sent to
this address are managed by the CLN wallet and can be used for channel
opening or withdrawn.

### 6.2 Withdraw On-Chain

```
MCP tool: withdraw(destination: "bc1q...", satoshi: "all")
```
```bash
lightning-cli withdraw -k destination=bc1q... satoshi=all
```

Withdraws on-chain funds to a Bitcoin address. Use `"all"` to sweep
the entire wallet balance, or a specific amount in satoshis. Optional
`feerate` to control the transaction fee.

### 6.3 List On-Chain Transactions

```
MCP tool: list_transactions()
```
```bash
lightning-cli listtransactions
```

Shows all on-chain transactions including deposits, withdrawals, and
channel funding/close transactions. This RPC takes no parameters.

### 6.4 List All Addresses

```
MCP tool: list_addresses()
```
```bash
lightning-cli listaddresses
```

Shows all Bitcoin addresses issued by the node. Filter by `address`
for a specific one, or use `start`/`limit` for pagination.

---

## 7. Askrene Layers: Routing Control

### What Are Layers?

Askrene layers are **overlay modifications** to the Lightning network
gossip map. They let you influence how `xpay` and `getroutes` compute
payment paths without changing the actual network.

Think of layers as **routing preferences** stacked on top of reality:

- **Disable** a node or channel you know is unreliable
- **Bias** against expensive or slow channels (negative bias)
- **Bias** toward your preferred routes (positive bias)
- **Override** channel fees/capacity with your own values

Layers are applied per-payment via `xpay(layers: ["my_layer"])` or
`getroute(layers: ["my_layer"])`. They don't affect other payments
unless explicitly referenced.

### 7.1 Create a Layer

```
MCP tool: askrene_create_layer(layer: "avoid_bad_peers")
```
```bash
lightning-cli askrene-create-layer avoid_bad_peers
```

Creates an empty layer. Use `persistent: true` to survive node restarts
(default: false, layer is lost on restart).

### 7.2 Disable an Unreliable Node

```
MCP tool: askrene_disable_node(layer: "avoid_bad_peers", node: "038194b5f32...")
```
```bash
lightning-cli askrene-disable-node -k layer=avoid_bad_peers node=038194b5f32...
```

Completely blocks routing through this node when the layer is active.
Useful when you know a node is flaky or malicious. New channels opened
by that node are also automatically blocked.

### 7.3 Bias Against an Expensive Channel

```
MCP tool: askrene_bias_channel(layer: "avoid_bad_peers", short_channel_id_dir: "116x1x1/1", bias: -10)
```
```bash
lightning-cli askrene-bias-channel -k layer=avoid_bad_peers short_channel_id_dir=116x1x1/1 bias=-10
```

Negative bias makes the router less likely to use this channel. Values
range from -100 (strongly avoid) to +100 (strongly prefer). Useful
values are typically -1 to -10 for mild avoidance.

Use `relative: true` to add to an existing bias instead of replacing it.

### 7.4 Bias Against a Node's Outgoing Channels

```
MCP tool: askrene_bias_node(layer: "avoid_bad_peers", node: "038194b5f32...", direction: "out", bias: -5)
```
```bash
lightning-cli askrene-bias-node -k layer=avoid_bad_peers node=038194b5f32... direction=out bias=-5
```

Biases all outgoing channels of a node. Use `direction: "in"` to bias
incoming channels instead. This is more efficient than biasing each
channel individually.

### 7.5 Override Channel Properties

```
MCP tool: askrene_update_channel(layer: "avoid_bad_peers", short_channel_id_dir: "116x1x1/1", fee_base_msat: "10000", fee_proportional_millionths: 500)
```
```bash
lightning-cli askrene-update-channel -k layer=avoid_bad_peers short_channel_id_dir=116x1x1/1 fee_base_msat=10000 fee_proportional_millionths=500
```

Tells askrene to treat this channel as if it has different fees than
what the gossip map says. You can also override `enabled`, `htlc_minimum_msat`,
`htlc_maximum_msat`, and `cltv_expiry_delta`. Set `enabled: false` to
effectively disable a single channel.

### 7.6 Pay Using a Layer

```
MCP tool: xpay(invstring: "lnbc...", layers: ["avoid_bad_peers"])
```
```bash
lightning-cli xpay -k invstring=lnbc... layers='["avoid_bad_peers"]'
```

Routes the payment using your layer's modifications. You can stack
multiple layers: `layers: ["avoid_bad_peers", "prefer_cheap_routes"]`.

### 7.7 Inspect Layers

```
MCP tool: askrene_list_layers()
```
```bash
lightning-cli askrene-listlayers
```

Shows all layers with their disabled nodes, biases, channel overrides,
and constraints. Filter by `layer` name for a specific layer.

### 7.8 Remove a Layer

```
MCP tool: askrene_remove_layer(layer: "avoid_bad_peers")
```
```bash
lightning-cli askrene-remove-layer avoid_bad_peers
```

Deletes the layer and all its modifications. Non-persistent layers are
also automatically removed on node restart.

### Practical Layer Recipes

**Recipe: Avoid a known-bad routing node**

```
1. askrene_create_layer(layer: "avoid_038194")
2. askrene_disable_node(layer: "avoid_038194", node: "038194b5f32...")
3. xpay(invstring: "lnbc...", layers: ["avoid_038194"])
```

**Recipe: Prefer your own channels for rebalancing**

```
1. askrene_create_layer(layer: "prefer_own", persistent: true)
2. askrene_bias_channel(layer: "prefer_own", short_channel_id_dir: "951988x1966x0/0", bias: 10)
3. askrene_bias_channel(layer: "prefer_own", short_channel_id_dir: "951988x1966x0/1", bias: 10)
4. xpay(invstring: "lnbc...", layers: ["prefer_own"])
```

**Recipe: Avoid high-fee channels**

```
1. askrene_create_layer(layer: "cheap_routes", persistent: true)
2. askrene_update_channel(layer: "cheap_routes", short_channel_id_dir: "123x4x5/0", fee_base_msat: "999999")
3. askrene_update_channel(layer: "cheap_routes", short_channel_id_dir: "123x4x5/1", fee_base_msat: "999999")
4. xpay(invstring: "lnbc...", layers: ["cheap_routes"])
```

Setting an absurdly high fee makes the router avoid that channel
without fully disabling it (it can still be used if there's no
alternative).

---

## 8. Splicing: Rebalancing Channels

Splicing moves funds into or out of existing channels in a single
on-chain transaction, without closing them. This is far cheaper than
close-and-reopen.

### 8.1 Find Splice-Capable Peers

```
MCP tool: list_splice_peers()
```

Lists your connected peers that support option_splice (feature bit
62/63). These are the channels you can splice with.

### 8.2 Find Splice-Capable Nodes (Network-Wide)

```
MCP tool: list_splice_nodes()
```

Lists all gossip-known nodes that support splicing. Use this to find
new peers to open splice-capable channels with.

### 8.3 Splice In (Add Funds to a Channel)

```
MCP tool: splice(script_or_json: "wallet -> 50000sat; 100% -> 951988x1966")
```
```bash
lightning-cli splice "wallet -> 50000sat; 100% -> 951988x1966"
```

Takes 50,000 sats from the on-chain wallet and adds them to channel
`951988x1966`. The `100%` means "all remaining funds after fees" go
into the channel.

### 8.4 Splice Out (Withdraw from a Channel)

```
MCP tool: splice(script_or_json: "951988x1966 -> 100000sat")
```
```bash
lightning-cli splice "951988x1966 -> 100000sat"
```

Takes 100,000 sats out of the channel and sends them to the on-chain
wallet. The channel stays open with reduced capacity.

### 8.5 Move Funds Between Channels

```
MCP tool: splice(script_or_json: "951988x1966 -> 50%; * -> 866191x460")
```
```bash
lightning-cli splice "951988x1966 -> 50%; * -> 866191x460"
```

Takes 50% of available funds from one channel and moves them to another.
The `*` operator divides remaining funds among all destinations using it.

### 8.6 Dry Run (Preview Without Executing)

```
MCP tool: splice(script_or_json: "wallet -> 10K; 100% -> 951988x1966", dryrun: true)
```
```bash
lightning-cli splice -k script_or_json="wallet -> 10K; 100% -> 951988x1966" dryrun=true
```

Shows what the splice would do without actually executing it. Always
do a dry run first for complex scripts.

### 8.7 Splice to a Bitcoin Address

```
MCP tool: splice(script_or_json: "951988x1966 -> 0.5M+fee; 0.5M -> bc1q...")
```
```bash
lightning-cli splice "951988x1966 -> 0.5M+fee; 0.5M -> bc1q..."
```

Takes 500,000 sats from a channel and sends them to an on-chain address.
The `+fee` adds enough to cover the on-chain transaction fee.

---

## 9. Security: Runes

### 9.1 Create a Read-Only Rune

```
MCP tool: create_rune(readonly: true)
```
```bash
lightning-cli createrune -k readonly=true
```

Creates a rune that can only call read-only RPC methods. Safe to share
with monitoring tools or dashboards.

### 9.2 Create a Restricted Rune

```
MCP tool: create_rune(restrictions: [["method=getinfo"], ["method=listpeers"], ["method=listpeerchannels"]])
```
```bash
lightning-cli createrune -k restrictions='[[method=getinfo],[method=listpeers],[method=listpeerchannels]]'
```

Creates a rune that can only call specific methods. Each inner array
is a set of alternative conditions (OR). Multiple inner arrays are
AND'd together.

### 9.3 List All Runes

```
MCP tool: show_runes()
```
```bash
lightning-cli showrunes
```

Shows all runes with their IDs and restrictions. Filter by `rune` ID
for a specific rune.

### 9.4 Verify a Message Signature

```
MCP tool: check_message(message: "hello", signature: "signature_here")
```
```bash
lightning-cli checkmessage -k message=hello signature=signature_here
```

Verifies that a message was signed by a specific node. If `pubkey` is
omitted, CLN checks if it was signed by this node.

### 9.5 Sign a Message

```
MCP tool: sign_message(message: "hello")
```
```bash
lightning-cli signmessage hello
```

Signs a message with the node's private key. The signature can be
verified by anyone using `checkmessage`.

---

## 10. Troubleshooting

### 10.1 Check for Stuck HTLCs

```
MCP tool: list_htlcs()
```
```bash
lightning-cli listhtlcs
```

Shows all pending HTLCs. Stuck HTLCs can indicate channel problems.
Filter by peer `id` to focus on a specific channel. Use `index`/`start`/
`limit` for pagination on busy nodes.

### 10.2 Inspect Peer Connection

```
MCP tool: list_peers(id: "pubkey", level: "debug")
```
```bash
lightning-cli listpeers -k id=pubkey level=debug
```

Shows detailed peer connection info including features, channels, and
log entries. Use `level: "io"` for maximum detail (packet-level).

### 10.3 Check Forwarding History

```
MCP tool: list_forwards(status: "failed")
```
```bash
lightning-cli listforwards -k status=failed
```

See why forwards are failing. `local_failed` means your node couldn't
forward (insufficient balance, channel disabled, etc). `failed` means
a downstream node failed.

### 10.4 Decode an Invoice Before Paying

```
MCP tool: decode(string: "lnbc...")
```
```bash
lightning-cli decode lnbc...
```

Inspect a BOLT11 invoice or BOLT12 offer before paying. Shows amount,
expiry, payment hash, route hints, and features. Always decode
unfamiliar invoices before paying.

### 10.5 Check Node Configuration

```
MCP tool: list_configs()
```
```bash
lightning-cli listconfigs
```

Shows all configuration options. Filter by `config` name for a specific
setting. Useful for debugging fee policies, network settings, and
plugin configurations.

### 10.6 Call Any RPC Method

```
MCP tool: call_rpc_method(method: "getinfo")
MCP tool: call_rpc_method(method: "listpeers", params: {"id": "pubkey"})
```
```bash
lightning-cli getinfo
lightning-cli listpeers pubkey
```

Escape hatch for RPC methods not directly exposed as MCP tools. Pass
any method name and optional params as a JSON object.
