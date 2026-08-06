use rmcp::{handler::server::wrapper::Parameters, model::*, tool, tool_handler, tool_router};
use serde::Serialize;
use serde_json::{json, Value};
use std::sync::Arc;
use tracing::debug;

use super::backend::ClnBackend;
use super::params::*;

#[derive(Clone)]
pub struct NodeService {
    backend: Arc<dyn ClnBackend>,
}

fn to_call_result(value: Value) -> Result<CallToolResult, ErrorData> {
    let content = ContentBlock::json(value)?;
    Ok(CallToolResult::success(vec![content]))
}

fn backend_error(e: anyhow::Error) -> ErrorData {
    ErrorData::internal_error(
        format!("Failed to communicate with lightning node: {e}"),
        None,
    )
}

fn has_feature(features_hex: &str, bits: &[usize]) -> bool {
    let bytes = match hex::decode(features_hex) {
        Ok(b) => b,
        Err(_) => return false,
    };
    // BOLT 9: Bit N is set if byte[N/8] has bit (N%8) set
    for &bit in bits {
        let byte_idx = bit / 8;
        if byte_idx < bytes.len() && (bytes[byte_idx] & (1 << (bit % 8))) != 0 {
            return true;
        }
    }
    false
}

fn feature_bits(name: &str) -> Option<Vec<usize>> {
    match name {
        "splice" | "option_splice" => Some(vec![62, 63]),
        "option_will_fund" => Some(vec![54, 55]),
        "anchors" | "option_anchors" => Some(vec![22, 23]),
        "static_remotekey" | "option_static_remotekey" => Some(vec![12, 13]),
        "zeroconf" | "option_zeroconf" => Some(vec![50, 51]),
        "scid_alias" | "option_scid_alias" => Some(vec![46, 47]),
        "route_blinding" | "option_route_blinding" => Some(vec![24, 25]),
        "simple_close" | "option_simple_close" => Some(vec![60, 61]),
        _ => None,
    }
}

fn serialize_params<T: Serialize>(params: &T) -> Value {
    serde_json::to_value(params).unwrap_or_else(|e| {
        tracing::warn!("Failed to serialize tool params, sending empty object: {e}");
        json!({})
    })
}

#[tool_router]
impl NodeService {
    pub fn new(backend: Arc<dyn ClnBackend>) -> Self {
        Self { backend }
    }

    async fn call_backend(&self, method: &str, params: Value) -> Result<CallToolResult, ErrorData> {
        debug!("Tool call: {} with params: {}", method, params);
        let result = self
            .backend
            .call(method, params)
            .await
            .map_err(backend_error)?;
        to_call_result(result)
    }

    // === Existing read-only tools ===

    #[tool(
        description = "getinfo - Get node information including id, alias, color, num_peers, num_active_channels, version, blockheight, network, fees_collected_msat, address, binding"
    )]
    pub async fn get_info(&self) -> Result<CallToolResult, ErrorData> {
        self.call_backend("getinfo", json!({})).await
    }

    #[tool(description = "listconfigs - List all configuration options of the Core Lightning node. Optionally filter by config name")]
    pub async fn list_configs(
        &self,
        Parameters(params): Parameters<ListConfigsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listconfigs", serialize_params(&params)).await
    }

    #[tool(
        description = "listaddresses - List all Bitcoin addresses issued by the node (bech32 and p2tr). Optionally filter by address, with start and limit for pagination"
    )]
    pub async fn list_addresses(
        &self,
        Parameters(params): Parameters<ListAddressesParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listaddresses", serialize_params(&params)).await
    }

    #[tool(
        description = "listchannels - Query active lightning channels in the entire network. Optionally filter by source, destination, or short_channel_id"
    )]
    pub async fn list_channels(
        &self,
        Parameters(params): Parameters<ListChannelsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listchannels", serialize_params(&params)).await
    }

    #[tool(
        description = "listpeerchannels - List channels with directly connected peers, showing their current state. Optionally filter by id, short_channel_id, or channel_id"
    )]
    pub async fn list_peer_channels(
        &self,
        Parameters(params): Parameters<ListPeerChannelsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listpeerchannels", serialize_params(&params)).await
    }

    #[tool(
        description = "listclosedchannels - List channels that have been closed, with details on close cause and fees. Optionally filter by peer id"
    )]
    pub async fn list_closed_channels(
        &self,
        Parameters(params): Parameters<ListClosedChannelsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listclosedchannels", serialize_params(&params)).await
    }

    #[tool(
        description = "listhtlcs - List all HTLCs (Hash Time Locked Contracts) with their current state. Optionally filter by id, with index/start/limit for pagination"
    )]
    pub async fn list_htlcs(
        &self,
        Parameters(params): Parameters<ListHtlcsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listhtlcs", serialize_params(&params)).await
    }

    #[tool(
        description = "listpays - List payment results. Optionally filter by bolt11 invoice string or status (pending/complete/failed)"
    )]
    pub async fn list_pays(
        &self,
        Parameters(params): Parameters<ListPaysParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listpays", serialize_params(&params)).await
    }

    #[tool(
        description = "listsendpays - List outgoing payments with more detail than listpays. Optionally filter by bolt11 invoice string or status (pending/complete/failed)"
    )]
    pub async fn list_send_pays(
        &self,
        Parameters(params): Parameters<ListSendPaysParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listsendpays", serialize_params(&params)).await
    }

    #[tool(
        description = "listforwards - List all HTLCs that have been forwarded by the node. Optionally filter by status (offered/settled/failed/local_failed), in_channel, or out_channel"
    )]
    pub async fn list_forwards(
        &self,
        Parameters(params): Parameters<ListForwardsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listforwards", serialize_params(&params)).await
    }

    #[tool(
        description = "listinvoices - Query invoice status. Optionally filter by label, payment_hash, or offer_id"
    )]
    pub async fn list_invoices(
        &self,
        Parameters(params): Parameters<ListInvoicesParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listinvoices", serialize_params(&params)).await
    }

    #[tool(
        description = "listpeers - List connected lightning nodes. Optionally filter by id, with optional log level for peer logs"
    )]
    pub async fn list_peers(
        &self,
        Parameters(params): Parameters<ListPeersParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listpeers", serialize_params(&params)).await
    }

    #[tool(
        description = "listnodes - Lookup node info from the network gossip map, optionally filtered by pubkey"
    )]
    pub async fn list_nodes(
        &self,
        Parameters(params): Parameters<ListNodesParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listnodes", serialize_params(&params)).await
    }

    #[tool(
        description = "listfunds - Show all funds currently managed by the node (UTXOs and channel funds). Optionally include spent outputs"
    )]
    pub async fn list_funds(
        &self,
        Parameters(params): Parameters<ListFundsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listfunds", serialize_params(&params)).await
    }

    #[tool(description = "listoffers - List all bolt12 offers on the node. Optionally filter by offer_id or active_only")]
    pub async fn list_offers(
        &self,
        Parameters(params): Parameters<ListOffersParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listoffers", serialize_params(&params)).await
    }

    #[tool(description = "listdatastore - List data stored in the node datastore. Optionally filter by key")]
    pub async fn list_datastore(
        &self,
        Parameters(params): Parameters<ListDatastoreParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listdatastore", serialize_params(&params)).await
    }

    #[tool(description = "feerates - Look up fee rates for various styles. Requires style: perkb (satoshis per 1000 virtual bytes) or perkw (satoshis per 1000 weight)")]
    pub async fn feerates(
        &self,
        Parameters(params): Parameters<FeeratesParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("feerates", serialize_params(&params)).await
    }

    #[tool(description = "getlog - Show log entries from the node. Optionally filter by level: broken, unusual, info, debug, trace, or io (default info)")]
    pub async fn get_log(
        &self,
        Parameters(params): Parameters<GetLogParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("getlog", serialize_params(&params)).await
    }

    // === Read-only tools with parameters ===

    #[tool(
        description = "getroute - Find the best route for a payment. Requires id (pubkey), amount_msat, and riskfactor. Optional cltv, fromid, exclude, maxhops. Deprecated in v26.06 in favor of getroutes"
    )]
    pub async fn get_route(
        &self,
        Parameters(params): Parameters<GetRouteParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("getroute", serialize_params(&params))
            .await
    }

    #[tool(description = "decode - Decode a lightning string (bolt11, bolt12, rune, etc.)")]
    pub async fn decode(
        &self,
        Parameters(params): Parameters<DecodeParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("decode", serialize_params(&params)).await
    }

    #[tool(
        description = "checkmessage - Verify a signature on a message. Requires message and signature, optional pubkey"
    )]
    pub async fn check_message(
        &self,
        Parameters(params): Parameters<CheckMessageParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("checkmessage", serialize_params(&params))
            .await
    }

    #[tool(
        description = "listtransactions - List on-chain transactions from the wallet (deposits, withdrawals, channel transactions)"
    )]
    pub async fn list_transactions(&self) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listtransactions", json!({})).await
    }

    #[tool(description = "showrunes - List runes on the node. Optionally filter by rune id")]
    pub async fn show_runes(
        &self,
        Parameters(params): Parameters<ShowRunesParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("showrunes", serialize_params(&params))
            .await
    }

    // === New write/mutation tools ===

    #[tool(
        description = "connect - Connect to a lightning node. Requires id (pubkey, optionally with @host:port), optional host"
    )]
    pub async fn connect_peer(
        &self,
        Parameters(params): Parameters<ConnectPeerParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("connect", serialize_params(&params))
            .await
    }

    #[tool(
        description = "disconnect - Disconnect from a peer. Requires id (pubkey), optional force flag"
    )]
    pub async fn disconnect_peer(
        &self,
        Parameters(params): Parameters<DisconnectPeerParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("disconnect", serialize_params(&params))
            .await
    }

    #[tool(
        description = "fundchannel - Open a channel with a connected peer. Requires id (pubkey) and amount (sat). Optional feerate, announce, request_amt (liquidity lease from peer), compact_lease, push_msat, close_to, utxos, mindepth, reserve"
    )]
    pub async fn fund_channel(
        &self,
        Parameters(params): Parameters<FundChannelParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("fundchannel", serialize_params(&params))
            .await
    }

    #[tool(
        description = "close - Close a channel. Requires id (pubkey or channel_id). Optional unilateraltimeout and fee_negotiation_step"
    )]
    pub async fn close_channel(
        &self,
        Parameters(params): Parameters<CloseChannelParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("close", serialize_params(&params)).await
    }

    #[tool(
        description = "setchannel - Update channel fee policy. Requires id (pubkey, short_channel_id, or all). Optional feebase, feeppm, htlcmin, htlcmax"
    )]
    pub async fn set_channel(
        &self,
        Parameters(params): Parameters<SetChannelParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("setchannel", serialize_params(&params))
            .await
    }

    #[tool(
        description = "invoice - Create a bolt11 invoice. Requires amount_msat, label, description. Optional expiry"
    )]
    pub async fn create_invoice(
        &self,
        Parameters(params): Parameters<CreateInvoiceParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("invoice", serialize_params(&params))
            .await
    }

    #[tool(
        description = "pay - Pay a bolt11 invoice. Requires bolt11. Optional amount_msat and label"
    )]
    pub async fn pay_invoice(
        &self,
        Parameters(params): Parameters<PayInvoiceParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("pay", serialize_params(&params)).await
    }

    #[tool(
        description = "keysend - Send funds to a node without an invoice. Requires destination (pubkey) and amount_msat. Optional label"
    )]
    pub async fn keysend(
        &self,
        Parameters(params): Parameters<KeysendParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("keysend", serialize_params(&params))
            .await
    }

    #[tool(
        description = "withdraw - Withdraw on-chain funds. Requires destination (address) and satoshi (amount or all). Optional feerate"
    )]
    pub async fn withdraw(
        &self,
        Parameters(params): Parameters<WithdrawParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("withdraw", serialize_params(&params))
            .await
    }

    #[tool(
        description = "newaddr - Generate a new on-chain address. Optional addresstype (bech32 or p2tr)"
    )]
    pub async fn new_address(
        &self,
        Parameters(params): Parameters<NewAddressParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("newaddr", serialize_params(&params))
            .await
    }

    #[tool(
        description = "signmessage - Sign a message with the node private key. Requires message"
    )]
    pub async fn sign_message(
        &self,
        Parameters(params): Parameters<SignMessageParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("signmessage", serialize_params(&params))
            .await
    }

    #[tool(
        description = "createrune - Create a new rune. Optional restrictions array and readonly flag"
    )]
    pub async fn create_rune(
        &self,
        Parameters(params): Parameters<CreateRuneParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("createrune", serialize_params(&params))
            .await
    }

    #[tool(
        description = "offer - Create a bolt12 offer. Requires description. Optional amount and label"
    )]
    pub async fn create_offer(
        &self,
        Parameters(params): Parameters<CreateOfferParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("offer", serialize_params(&params)).await
    }

    #[tool(description = "disableoffer - Disable an existing offer. Requires offer_id")]
    pub async fn disable_offer(
        &self,
        Parameters(params): Parameters<DisableOfferParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("disableoffer", serialize_params(&params))
            .await
    }

    #[tool(
        description = "fetchinvoice - Fetch an invoice for a bolt12 offer. Requires offer. Optional amount_msat and quantity"
    )]
    pub async fn fetch_invoice(
        &self,
        Parameters(params): Parameters<FetchInvoiceParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("fetchinvoice", serialize_params(&params))
            .await
    }

    // === xpay / askrene tools ===

    #[tool(
        description = "xpay - Send a payment using bolt11/bolt12 invoice or offer. Supports askrene layers for channel exclusion and biasing. Requires invstring. Optional amount_msat, maxfee, layers, retry_for, partial_msat, maxdelay, label"
    )]
    pub async fn xpay(
        &self,
        Parameters(params): Parameters<XpayParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("xpay", serialize_params(&params)).await
    }

    #[tool(
        description = "askrene_create_layer - Create a new askrene layer for routing customization. Requires layer name. Optional persistent flag to survive restarts"
    )]
    pub async fn askrene_create_layer(
        &self,
        Parameters(params): Parameters<AskreneCreateLayerParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("askrene-create-layer", serialize_params(&params))
            .await
    }

    #[tool(
        description = "askrene_update_channel - Override channel properties in a layer. Requires layer and short_channel_id_dir. Optional enabled, htlc_minimum_msat, htlc_maximum_msat, fee_base_msat, fee_proportional_millionths, cltv_expiry_delta"
    )]
    pub async fn askrene_update_channel(
        &self,
        Parameters(params): Parameters<AskreneUpdateChannelParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("askrene-update-channel", serialize_params(&params))
            .await
    }

    #[tool(
        description = "askrene_bias_channel - Apply a manual bias to a channel in a layer. Positive bias favors, negative disfavors (-100 to +100). Requires layer, short_channel_id_dir, bias. Optional description, relative"
    )]
    pub async fn askrene_bias_channel(
        &self,
        Parameters(params): Parameters<AskreneBiasChannelParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("askrene-bias-channel", serialize_params(&params))
            .await
    }

    #[tool(
        description = "askrene_bias_node - Apply a manual bias to all channels of a node in a layer. Requires layer, node pubkey, direction (in/out), bias. Optional description, relative"
    )]
    pub async fn askrene_bias_node(
        &self,
        Parameters(params): Parameters<AskreneBiasNodeParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("askrene-bias-node", serialize_params(&params))
            .await
    }

    #[tool(
        description = "askrene_list_layers - List all askrene layers and their contents. Optional layer name to filter"
    )]
    pub async fn askrene_list_layers(
        &self,
        Parameters(params): Parameters<AskreneListLayersParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("askrene-listlayers", serialize_params(&params))
            .await
    }

    #[tool(
        description = "askrene_remove_layer - Remove an askrene layer. Requires layer name"
    )]
    pub async fn askrene_remove_layer(
        &self,
        Parameters(params): Parameters<AskreneRemoveLayerParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("askrene-remove-layer", serialize_params(&params))
            .await
    }

    #[tool(
        description = "askrene_disable_node - Disable a node in a layer, preventing routing through it. Requires layer and node pubkey"
    )]
    pub async fn askrene_disable_node(
        &self,
        Parameters(params): Parameters<AskreneDisableNodeParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("askrene-disable-node", serialize_params(&params))
            .await
    }

    // === Splicing ===

    #[tool(
        description = "splice - High-level splice script command (dev-splice). Move funds into/out of channels in a single onchain transaction. Supports wallet, channels, bitcoin addresses, and lease requests. Requires script_or_json. Optional dryrun, force_feerate, debug_log. Examples: 'wallet -> 10K; 100% -> 8338aef0' (splice in), '8338aef0 -> 50%' (splice out), '8338aef0 -> 50%; * -> 07bfddea' (move between channels)"
    )]
    pub async fn splice(
        &self,
        Parameters(params): Parameters<SpliceParams>,
    ) -> Result<CallToolResult, ErrorData> {
        self.call_backend("dev-splice", serialize_params(&params)).await
    }

    // === Splice peer discovery ===

    #[tool(
        description = "list_splice_peers - List your connected peers that support option_splice (feature bit 62/63). Filters your actual channel partners, not the entire gossip map. Returns peers with their pubkey, connection status, and features. Use this to find which channels you can splice with."
    )]
    pub async fn list_splice_peers(&self) -> Result<CallToolResult, ErrorData> {
        let result = self
            .backend
            .call("listpeers", json!({}))
            .await
            .map_err(backend_error)?;

        let peers = result
            .get("peers")
            .and_then(|p| p.as_array())
            .cloned()
            .unwrap_or_default();

        let splice_bits = [62usize, 63];
        let splice_peers: Vec<Value> = peers
            .into_iter()
            .filter(|peer| {
                peer.get("features")
                    .and_then(|f| f.as_str())
                    .map(|features| has_feature(features, &splice_bits))
                    .unwrap_or(false)
            })
            .map(|peer| {
                json!({
                    "id": peer.get("id").cloned().unwrap_or(Value::Null),
                    "connected": peer.get("connected").cloned().unwrap_or(Value::Null),
                    "features": peer.get("features").cloned().unwrap_or(Value::Null),
                    "num_channels": peer.get("num_channels").cloned().unwrap_or(Value::Null),
                })
            })
            .collect();

        let summary = json!({
            "splice_capable_peers": splice_peers.len(),
            "peers": splice_peers,
        });
        to_call_result(summary)
    }

    #[tool(
        description = "list_splice_nodes - List all gossip-known nodes that support option_splice (feature bit 62/63). Use this to find new peers to connect and open splice-capable channels with. Returns nodes with their pubkey, alias, and addresses."
    )]
    pub async fn list_splice_nodes(&self) -> Result<CallToolResult, ErrorData> {
        let result = self
            .backend
            .call("listnodes", json!({}))
            .await
            .map_err(backend_error)?;

        let nodes = result
            .get("nodes")
            .and_then(|n| n.as_array())
            .cloned()
            .unwrap_or_default();

        let splice_bits = [62usize, 63];
        let splice_nodes: Vec<Value> = nodes
            .into_iter()
            .filter(|node| {
                node.get("features")
                    .and_then(|f| f.as_str())
                    .map(|features| has_feature(features, &splice_bits))
                    .unwrap_or(false)
            })
            .map(|node| {
                json!({
                    "nodeid": node.get("nodeid").cloned().unwrap_or(Value::Null),
                    "alias": node.get("alias").cloned().unwrap_or(Value::Null),
                    "addresses": node.get("addresses").cloned().unwrap_or(Value::Null),
                    "features": node.get("features").cloned().unwrap_or(Value::Null),
                })
            })
            .collect();

        let summary = json!({
            "splice_capable_nodes": splice_nodes.len(),
            "nodes": splice_nodes,
        });
        to_call_result(summary)
    }

    // === Composite operator tools ===

    #[tool(
        description = "channel_summary - Aggregated channel dashboard. Combines listpeerchannels + listpeers + listforwards into a single view showing per-channel balances, fee rates, forwarding counts, fee revenue, and offline peers."
    )]
    pub async fn channel_summary(&self) -> Result<CallToolResult, ErrorData> {
        let channels_res = self.backend.call("listpeerchannels", json!({})).await;
        let peers_res = self.backend.call("listpeers", json!({})).await;
        let forwards_res = self.backend.call("listforwards", json!({})).await;

        let channels = channels_res
            .map_err(backend_error)?
            .get("channels")
            .and_then(|c| c.as_array())
            .cloned()
            .unwrap_or_default();

        let peers = peers_res
            .map_err(backend_error)?
            .get("peers")
            .and_then(|p| p.as_array())
            .cloned()
            .unwrap_or_default();

        let forwards = forwards_res
            .map_err(backend_error)?
            .get("forwards")
            .and_then(|f| f.as_array())
            .cloned()
            .unwrap_or_default();

        // Build peer lookup: pubkey -> {connected, alias}
        let peer_map: std::collections::HashMap<String, Value> = peers
            .iter()
            .filter_map(|p| {
                let id = p.get("id").and_then(|v| v.as_str())?.to_string();
                Some((id, p.clone()))
            })
            .collect();

        // Aggregate forwards by out_channel
        let mut fwd_by_channel: std::collections::HashMap<String, (u64, u64)> = std::collections::HashMap::new();
        for fwd in &forwards {
            if fwd.get("status").and_then(|s| s.as_str()) == Some("settled") {
                if let Some(out_ch) = fwd.get("out_channel").and_then(|c| c.as_str()) {
                    let fee = fwd.get("fee_msat").and_then(|f| f.as_u64()).unwrap_or(0);
                    let entry = fwd_by_channel.entry(out_ch.to_string()).or_insert((0, 0));
                    entry.0 += 1;
                    entry.1 += fee;
                }
            }
        }

        let mut total_capacity: u64 = 0;
        let mut active_count = 0u64;
        let mut inactive_count = 0u64;
        let mut offline_peers: Vec<Value> = Vec::new();

        let channel_list: Vec<Value> = channels
            .iter()
            .map(|ch| {
                let peer_id = ch.get("peer_id").and_then(|v| v.as_str()).unwrap_or("");
                let scid = ch.get("short_channel_id").and_then(|v| v.as_str()).unwrap_or("");
                let state = ch.get("state").and_then(|v| v.as_str()).unwrap_or("");
                let to_us = ch.get("to_us_msat").and_then(|v| v.as_u64()).unwrap_or(0);
                let total = ch.get("total_msat").and_then(|v| v.as_u64()).unwrap_or(0);
                let connected = ch.get("peer_connected").and_then(|v| v.as_bool()).unwrap_or(false);
                let fee_base = ch.get("fee_base_msat").and_then(|v| v.as_u64()).unwrap_or(0);
                let fee_ppm = ch.get("fee_proportional_millionths").and_then(|v| v.as_u64()).unwrap_or(0);

                total_capacity += total;
                if state == "CHANNELD_NORMAL" {
                    active_count += 1;
                } else {
                    inactive_count += 1;
                }

                if !connected {
                    offline_peers.push(json!(peer_id));
                }

                let (fwd_count, fwd_fees) = fwd_by_channel.get(scid).copied().unwrap_or((0, 0));

                let peer_alias = peer_map
                    .get(peer_id)
                    .and_then(|p| p.get("alias"))
                    .cloned()
                    .unwrap_or(Value::Null);

                json!({
                    "peer_id": peer_id,
                    "peer_alias": peer_alias,
                    "channel_id": ch.get("channel_id").cloned().unwrap_or(Value::Null),
                    "short_channel_id": scid,
                    "state": state,
                    "peer_connected": connected,
                    "to_us_msat": to_us,
                    "total_msat": total,
                    "fee_base_msat": fee_base,
                    "fee_proportional_millionths": fee_ppm,
                    "forwards_settled": fwd_count,
                    "fees_earned_msat": fwd_fees,
                })
            })
            .collect();

        let summary = json!({
            "total_capacity_msat": total_capacity,
            "active_channels": active_count,
            "inactive_channels": inactive_count,
            "channels": channel_list,
            "offline_peers": offline_peers,
        });
        to_call_result(summary)
    }

    #[tool(
        description = "node_health - Health dashboard combining getinfo + feerates + listpeerchannels. Shows version, blockheight, network, peer/channel counts, pending HTLCs, fees collected, and current fee rates."
    )]
    pub async fn node_health(&self) -> Result<CallToolResult, ErrorData> {
        let info_res = self.backend.call("getinfo", json!({})).await;
        let feerates_res = self.backend.call("feerates", json!({})).await;
        let channels_res = self.backend.call("listpeerchannels", json!({})).await;

        let info = info_res.map_err(backend_error)?;
        let feerates = feerates_res.map_err(backend_error)?;
        let channels = channels_res
            .map_err(backend_error)?
            .get("channels")
            .and_then(|c| c.as_array())
            .cloned()
            .unwrap_or_default();

        let mut active = 0u64;
        let mut inactive = 0u64;
        let mut pending_htlcs = 0u64;

        for ch in &channels {
            if ch.get("state").and_then(|s| s.as_str()) == Some("CHANNELD_NORMAL") {
                active += 1;
            } else {
                inactive += 1;
            }
            if let Some(htlcs) = ch.get("htlcs").and_then(|h| h.as_array()) {
                pending_htlcs += htlcs.len() as u64;
            }
        }

        let summary = json!({
            "version": info.get("version").cloned().unwrap_or(Value::Null),
            "blockheight": info.get("blockheight").cloned().unwrap_or(Value::Null),
            "network": info.get("network").cloned().unwrap_or(Value::Null),
            "num_peers": info.get("num_peers").cloned().unwrap_or(Value::Null),
            "num_active_channels": active,
            "num_inactive_channels": inactive,
            "pending_htlcs": pending_htlcs,
            "fees_collected_msat": info.get("fees_collected_msat").cloned().unwrap_or(Value::Null),
            "feerates": feerates,
            "addresses": info.get("address").cloned().unwrap_or(Value::Null),
        });
        to_call_result(summary)
    }

    #[tool(
        description = "fee_report - Per-channel fee performance report. Combines listforwards (revenue) + listpeerchannels (fee config). Shows total fees earned, total forwards, and per-channel breakdown of fee settings vs actual forwarding revenue."
    )]
    pub async fn fee_report(&self) -> Result<CallToolResult, ErrorData> {
        let forwards_res = self.backend.call("listforwards", json!({})).await;
        let channels_res = self.backend.call("listpeerchannels", json!({})).await;

        let forwards = forwards_res
            .map_err(backend_error)?
            .get("forwards")
            .and_then(|f| f.as_array())
            .cloned()
            .unwrap_or_default();

        let channels = channels_res
            .map_err(backend_error)?
            .get("channels")
            .and_then(|c| c.as_array())
            .cloned()
            .unwrap_or_default();

        // Aggregate settled forwards by out_channel
        let mut fwd_by_channel: std::collections::HashMap<String, (u64, u64)> = std::collections::HashMap::new();
        let mut total_fees: u64 = 0;
        let mut total_forwards: u64 = 0;

        for fwd in &forwards {
            if fwd.get("status").and_then(|s| s.as_str()) == Some("settled") {
                let fee = fwd.get("fee_msat").and_then(|f| f.as_u64()).unwrap_or(0);
                total_fees += fee;
                total_forwards += 1;
                if let Some(out_ch) = fwd.get("out_channel").and_then(|c| c.as_str()) {
                    let entry = fwd_by_channel.entry(out_ch.to_string()).or_insert((0, 0));
                    entry.0 += 1;
                    entry.1 += fee;
                }
            }
        }

        let per_channel: Vec<Value> = channels
            .iter()
            .map(|ch| {
                let scid = ch.get("short_channel_id").and_then(|v| v.as_str()).unwrap_or("");
                let (fwd_count, fwd_fees) = fwd_by_channel.get(scid).copied().unwrap_or((0, 0));

                json!({
                    "channel_id": ch.get("channel_id").cloned().unwrap_or(Value::Null),
                    "short_channel_id": scid,
                    "peer_id": ch.get("peer_id").cloned().unwrap_or(Value::Null),
                    "fee_base_msat": ch.get("fee_base_msat").cloned().unwrap_or(Value::Null),
                    "fee_proportional_millionths": ch.get("fee_proportional_millionths").cloned().unwrap_or(Value::Null),
                    "htlc_min_msat": ch.get("minimum_htlc_out_msat").cloned().unwrap_or(Value::Null),
                    "htlc_max_msat": ch.get("maximum_htlc_out_msat").cloned().unwrap_or(Value::Null),
                    "forwards_settled": fwd_count,
                    "fees_earned_msat": fwd_fees,
                })
            })
            .collect();

        let report = json!({
            "total_fees_earned_msat": total_fees,
            "total_forwards": total_forwards,
            "per_channel": per_channel,
        });
        to_call_result(report)
    }

    #[tool(
        description = "list_peers_by_feature - Filter your connected peers by BOLT 9 feature bit. Requires feature name: splice, option_will_fund, anchors, static_remotekey, zeroconf, scid_alias, route_blinding, or simple_close. Returns matching peers with pubkey, connection status, and features."
    )]
    pub async fn list_peers_by_feature(
        &self,
        Parameters(params): Parameters<ListPeersByFeatureParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let bits = match feature_bits(&params.feature) {
            Some(b) => b,
            None => {
                return to_call_result(json!({
                    "error": format!("Unknown feature: {}. Supported: splice, option_will_fund, anchors, static_remotekey, zeroconf, scid_alias, route_blinding, simple_close", params.feature),
                    "matching_peers": 0,
                    "peers": [],
                }));
            }
        };

        let result = self
            .backend
            .call("listpeers", json!({}))
            .await
            .map_err(backend_error)?;

        let peers = result
            .get("peers")
            .and_then(|p| p.as_array())
            .cloned()
            .unwrap_or_default();

        let matching: Vec<Value> = peers
            .into_iter()
            .filter(|peer| {
                peer.get("features")
                    .and_then(|f| f.as_str())
                    .map(|features| has_feature(features, &bits))
                    .unwrap_or(false)
            })
            .map(|peer| {
                json!({
                    "id": peer.get("id").cloned().unwrap_or(Value::Null),
                    "connected": peer.get("connected").cloned().unwrap_or(Value::Null),
                    "features": peer.get("features").cloned().unwrap_or(Value::Null),
                    "num_channels": peer.get("num_channels").cloned().unwrap_or(Value::Null),
                })
            })
            .collect();

        let summary = json!({
            "feature": params.feature,
            "matching_peers": matching.len(),
            "peers": matching,
        });
        to_call_result(summary)
    }

    // === Generic tool ===

    #[tool(
        description = "call_rpc_method - Call any CLN RPC method directly. Requires method name. Optional params object"
    )]
    pub async fn call_rpc_method(
        &self,
        Parameters(params): Parameters<CallRpcMethodParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let CallRpcMethodParams { method, params } = params;
        let rpc_params = params.unwrap_or(json!({}));
        self.call_backend(&method, rpc_params).await
    }
}

#[tool_handler]
impl rmcp::ServerHandler for NodeService {}
