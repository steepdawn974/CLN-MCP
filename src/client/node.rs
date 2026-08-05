use rmcp::{
    handler::server::wrapper::Parameters,
    model::*,
    tool, tool_handler, tool_router,
};
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
    let content = ContentBlock::json(value).unwrap();
    Ok(CallToolResult::success(vec![content]))
}

fn backend_error(e: anyhow::Error) -> ErrorData {
    ErrorData::internal_error(
        format!("Failed to communicate with lightning node: {e}"),
        None,
    )
}

fn serialize_params<T: Serialize>(params: &T) -> Value {
    serde_json::to_value(params).unwrap_or_else(|_| json!({}))
}

#[tool_router]
impl NodeService {
    pub fn new(backend: Arc<dyn ClnBackend>) -> Self {
        Self { backend }
    }

    async fn call_backend(&self, method: &str, params: Value) -> Result<CallToolResult, ErrorData> {
        debug!("Tool call: {} with params: {}", method, params);
        let result = self.backend.call(method, params).await.map_err(backend_error)?;
        to_call_result(result)
    }

    // === Existing read-only tools ===

    #[tool(description = "getinfo - Get node information including id, alias, color, num_peers, num_active_channels, version, blockheight, network, fees_collected_msat, address, binding")]
    pub async fn get_info(&self) -> Result<CallToolResult, ErrorData> {
        self.call_backend("getinfo", json!({})).await
    }

    #[tool(description = "listconfigs - List all configuration options of the Core Lightning node")]
    pub async fn list_configs(&self) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listconfigs", json!({})).await
    }

    #[tool(description = "listaddresses - List all addresses of the node (both announced and binding addresses)")]
    pub async fn list_addresses(&self) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listaddresses", json!({})).await
    }

    #[tool(description = "listchannels - Query active lightning channels in the entire network. Optionally filter by short_channel_id, source, or destination")]
    pub async fn list_channels(&self) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listchannels", json!({})).await
    }

    #[tool(description = "listpeerchannels - List channels with directly connected peers, showing their current state")]
    pub async fn list_peer_channels(&self) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listpeerchannels", json!({})).await
    }

    #[tool(description = "listclosedchannels - List channels that have been closed, with details on close cause and fees")]
    pub async fn list_closed_channels(&self) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listclosedchannels", json!({})).await
    }

    #[tool(description = "listhtlcs - List all HTLCs (Hash Time Locked Contracts) with their current state")]
    pub async fn list_htlcs(&self) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listhtlcs", json!({})).await
    }

    #[tool(description = "listpays - List payment results, optionally filtered by bolt11 invoice string")]
    pub async fn list_pays(&self) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listpays", json!({})).await
    }

    #[tool(description = "listsendpays - List outgoing payments with more detail than listpays, optionally filtered by bolt11")]
    pub async fn list_send_pays(&self) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listsendpays", json!({})).await
    }

    #[tool(description = "listforwards - List all HTLCs that have been forwarded by the node, optionally filtered by status, in_channel, or out_channel")]
    pub async fn list_forwards(&self) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listforwards", json!({})).await
    }

    #[tool(description = "listinvoices - Query invoice status. Optionally filter by label, invstring, payment_hash, or offer_id")]
    pub async fn list_invoices(&self) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listinvoices", json!({})).await
    }

    #[tool(description = "listpeers - List connected lightning nodes. Optionally filter by id or show log level")]
    pub async fn list_peers(&self) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listpeers", json!({})).await
    }

    #[tool(description = "listnodes - Lookup node info from the network gossip map, optionally filtered by pubkey")]
    pub async fn list_nodes(&self) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listnodes", json!({})).await
    }

    #[tool(description = "listfunds - Show all funds currently managed by the node (UTXOs and channel funds). Optionally include spent outputs")]
    pub async fn list_funds(&self) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listfunds", json!({})).await
    }

    #[tool(description = "listoffers - List all bolt12 offers on the node")]
    pub async fn list_offers(&self) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listoffers", json!({})).await
    }

    #[tool(description = "listdatastore - List data stored in the node datastore")]
    pub async fn list_datastore(&self) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listdatastore", json!({})).await
    }

    #[tool(description = "feerates - Look up fee rates for various styles (perkb or perkw)")]
    pub async fn feerates(&self) -> Result<CallToolResult, ErrorData> {
        self.call_backend("feerates", json!({})).await
    }

    #[tool(description = "getlog - Show log entries from the node, optionally filtered by level")]
    pub async fn get_log(&self) -> Result<CallToolResult, ErrorData> {
        self.call_backend("getlog", json!({})).await
    }

    #[tool(description = "bkpr-channelsapy - List APY stats for channels, optionally filtered by account")]
    pub async fn bkpr_channels_apy(&self) -> Result<CallToolResult, ErrorData> {
        self.call_backend("bkpr-channelsapy", json!({})).await
    }

    #[tool(description = "bkpr-listbalances - List current and historical account balances")]
    pub async fn bkpr_list_balances(&self) -> Result<CallToolResult, ErrorData> {
        self.call_backend("bkpr-listbalances", json!({})).await
    }

    #[tool(description = "bkpr-listincome - List income events (routing fees, on-chain transactions, etc.)")]
    pub async fn bkpr_list_income(&self) -> Result<CallToolResult, ErrorData> {
        self.call_backend("bkpr-listincome", json!({})).await
    }

    #[tool(description = "bkpr-listaccountevents - List detailed accounting events (channel opens, closes, HTLCs, etc.)")]
    pub async fn bkpr_list_account_events(&self) -> Result<CallToolResult, ErrorData> {
        self.call_backend("bkpr-listaccountevents", json!({})).await
    }

    // === New read-only tools with parameters ===

    #[tool(description = "getroute - Find the best route for a payment. Requires id (pubkey) and amount_msat. Optional riskfactor")]
    pub async fn get_route(&self, Parameters(params): Parameters<GetRouteParams>) -> Result<CallToolResult, ErrorData> {
        self.call_backend("getroute", serialize_params(&params)).await
    }

    #[tool(description = "decode - Decode a lightning string (bolt11, bolt12, rune, etc.)")]
    pub async fn decode(&self, Parameters(params): Parameters<DecodeParams>) -> Result<CallToolResult, ErrorData> {
        self.call_backend("decode", serialize_params(&params)).await
    }

    #[tool(description = "decode - Decode a lightning string (bolt11, bolt12, rune, etc.)"PAY)]
    pub async fn decode_pay(&self, Parameters(params): Parameters<DecodePayParams>) -> Result<CallToolResult, ErrorData> {
        self.call_backend("decodepay", serialize_params(&params)).await
    }

    #[tool(description = "checkmessage - Verify a signature on a message. Requires message and signature, optional pubkey")]
    pub async fn check_message(&self, Parameters(params): Parameters<CheckMessageParams>) -> Result<CallToolResult, ErrorData> {
        self.call_backend("checkmessage", serialize_params(&params)).await
    }

    #[tool(description = "listtransactions - List on-chain transactions from the wallet. Optional start and limit for pagination")]
    pub async fn list_transactions(&self, Parameters(params): Parameters<ListTransactionsParams>) -> Result<CallToolResult, ErrorData> {
        self.call_backend("listtransactions", serialize_params(&params)).await
    }

    #[tool(description = "showrunes - List runes on the node. Optionally filter by rune id")]
    pub async fn show_runes(&self, Parameters(params): Parameters<ShowRunesParams>) -> Result<CallToolResult, ErrorData> {
        self.call_backend("showrunes", serialize_params(&params)).await
    }

    // === New write/mutation tools ===

    #[tool(description = "connect - Connect to a lightning node. Requires id (pubkey, optionally with @host:port), optional host")]
    pub async fn connect_peer(&self, Parameters(params): Parameters<ConnectPeerParams>) -> Result<CallToolResult, ErrorData> {
        self.call_backend("connect", serialize_params(&params)).await
    }

    #[tool(description = "disconnect - Disconnect from a peer. Requires id (pubkey), optional force flag")]
    pub async fn disconnect_peer(&self, Parameters(params): Parameters<DisconnectPeerParams>) -> Result<CallToolResult, ErrorData> {
        self.call_backend("disconnect", serialize_params(&params)).await
    }

    #[tool(description = "fundchannel - Open a channel with a connected peer. Requires id (pubkey) and amount (sat). Optional feerate and announce")]
    pub async fn fund_channel(&self, Parameters(params): Parameters<FundChannelParams>) -> Result<CallToolResult, ErrorData> {
        self.call_backend("fundchannel", serialize_params(&params)).await
    }

    #[tool(description = "close - Close a channel. Requires id (pubkey or channel_id). Optional unilateraltimeout and fee_negotiation_step")]
    pub async fn close_channel(&self, Parameters(params): Parameters<CloseChannelParams>) -> Result<CallToolResult, ErrorData> {
        self.call_backend("close", serialize_params(&params)).await
    }

    #[tool(description = "setchannel - Update channel fee policy. Requires id (pubkey, short_channel_id, or all). Optional feebase, feeppm, htlcmin, htlcmax")]
    pub async fn set_channel(&self, Parameters(params): Parameters<SetChannelParams>) -> Result<CallToolResult, ErrorData> {
        self.call_backend("setchannel", serialize_params(&params)).await
    }

    #[tool(description = "invoice - Create a bolt11 invoice. Requires amount_msat, label, description. Optional expiry")]
    pub async fn create_invoice(&self, Parameters(params): Parameters<CreateInvoiceParams>) -> Result<CallToolResult, ErrorData> {
        self.call_backend("invoice", serialize_params(&params)).await
    }

    #[tool(description = "pay - Pay a bolt11 invoice. Requires bolt11. Optional amount_msat and label")]
    pub async fn pay_invoice(&self, Parameters(params): Parameters<PayInvoiceParams>) -> Result<CallToolResult, ErrorData> {
        self.call_backend("pay", serialize_params(&params)).await
    }

    #[tool(description = "keysend - Send funds to a node without an invoice. Requires destination (pubkey) and amount_msat. Optional label")]
    pub async fn keysend(&self, Parameters(params): Parameters<KeysendParams>) -> Result<CallToolResult, ErrorData> {
        self.call_backend("keysend", serialize_params(&params)).await
    }

    #[tool(description = "withdraw - Withdraw on-chain funds. Requires destination (address) and satoshi (amount or all). Optional feerate")]
    pub async fn withdraw(&self, Parameters(params): Parameters<WithdrawParams>) -> Result<CallToolResult, ErrorData> {
        self.call_backend("withdraw", serialize_params(&params)).await
    }

    #[tool(description = "newaddr - Generate a new on-chain address. Optional addresstype (bech32 or p2tr)")]
    pub async fn new_address(&self, Parameters(params): Parameters<NewAddressParams>) -> Result<CallToolResult, ErrorData> {
        self.call_backend("newaddr", serialize_params(&params)).await
    }

    #[tool(description = "signmessage - Sign a message with the node private key. Requires message")]
    pub async fn sign_message(&self, Parameters(params): Parameters<SignMessageParams>) -> Result<CallToolResult, ErrorData> {
        self.call_backend("signmessage", serialize_params(&params)).await
    }

    #[tool(description = "createrune - Create a new rune. Optional restrictions array and readonly flag")]
    pub async fn create_rune(&self, Parameters(params): Parameters<CreateRuneParams>) -> Result<CallToolResult, ErrorData> {
        self.call_backend("createrune", serialize_params(&params)).await
    }

    #[tool(description = "offer - Create a bolt12 offer. Requires description. Optional amount and label")]
    pub async fn create_offer(&self, Parameters(params): Parameters<CreateOfferParams>) -> Result<CallToolResult, ErrorData> {
        self.call_backend("offer", serialize_params(&params)).await
    }

    #[tool(description = "disableoffer - Disable an existing offer. Requires offer_id")]
    pub async fn disable_offer(&self, Parameters(params): Parameters<DisableOfferParams>) -> Result<CallToolResult, ErrorData> {
        self.call_backend("disableoffer", serialize_params(&params)).await
    }

    #[tool(description = "fetchinvoice - Fetch an invoice for a bolt12 offer. Requires offer. Optional amount_msat and quantity")]
    pub async fn fetch_invoice(&self, Parameters(params): Parameters<FetchInvoiceParams>) -> Result<CallToolResult, ErrorData> {
        self.call_backend("fetchinvoice", serialize_params(&params)).await
    }

    // === Generic tool ===

    #[tool(description = "call_rpc_method - Call any CLN RPC method directly. Requires method name. Optional params object")]
    pub async fn call_rpc_method(&self, Parameters(params): Parameters<CallRpcMethodParams>) -> Result<CallToolResult, ErrorData> {
        let method = params.method.clone();
        let rpc_params = params.params.clone().unwrap_or(json!({}));
        self.call_backend(&method, rpc_params).await
    }
}

#[tool_handler]
impl rmcp::ServerHandler for NodeService {}
