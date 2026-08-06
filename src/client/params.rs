use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// === Read-only tool params (optional filters) ===

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetRouteParams {
    /// The pubkey of the destination node
    pub id: String,
    /// Amount to send in millisatoshi
    pub amount_msat: u64,
    /// Risk factor (annual cost of funds being stuck, as a percentage)
    pub riskfactor: u64,
    /// CLTV blocks to spare (default 9)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cltv: Option<u32>,
    /// Node to start the route from (default is this node)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fromid: Option<String>,
    /// Array of short-channel-id/direction or node-id to exclude from routing
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
    /// Maximum number of channels/hops (default 20)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maxhops: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DecodeParams {
    /// The string to decode (bolt11, bolt12, rune, etc.)
    pub string: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CheckMessageParams {
    /// The message that was signed
    pub message: String,
    /// The signature (zbase32 encoded)
    pub signature: String,
    /// Optional pubkey of the signer
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pubkey: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FeeratesParams {
    /// Fee rate style: "perkb" (satoshis per 1000 virtual bytes) or "perkw" (satoshis per 1000 weight)
    pub style: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetLogParams {
    /// Log level filter: "broken", "unusual", "info", "debug", "trace", or "io" (default "info")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ShowRunesParams {
    /// Optional rune id to filter by
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rune: Option<String>,
}

// === Write/mutation tool params ===

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ConnectPeerParams {
    /// The node pubkey (optionally with @host:port)
    pub id: String,
    /// Optional host:port if not included in id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DisconnectPeerParams {
    /// The pubkey of the peer to disconnect
    pub id: String,
    /// Force disconnect even if channel is active
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FundChannelParams {
    /// The pubkey of the peer to open a channel with
    pub id: String,
    /// Amount in satoshis to fund the channel
    pub amount: u64,
    /// Fee rate for the funding transaction
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feerate: Option<u64>,
    /// Whether to announce the channel (default true)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub announce: Option<bool>,
    /// Amount of liquidity to lease from peer (requires compact_lease)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_amt: Option<u64>,
    /// Compact representation of peer's expected lease terms
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compact_lease: Option<String>,
    /// Amount in millisatoshi to push to peer at open
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push_msat: Option<u64>,
    /// Bitcoin address for cooperative close
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub close_to: Option<String>,
    /// UTXOs to use, as array of "txid:vout"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub utxos: Option<Vec<String>>,
    /// Number of confirmations required (default 1)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mindepth: Option<u32>,
    /// Reserve amount for peer (default 1% of funding)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserve: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CloseChannelParams {
    /// The pubkey or channel_id of the channel to close
    pub id: String,
    /// Timeout in seconds before unilateral close (0 = wait indefinitely)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unilateraltimeout: Option<u64>,
    /// Fee negotiation step
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee_negotiation_step: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SetChannelParams {
    /// The pubkey, short_channel_id, or "all"
    pub id: String,
    /// Base fee in millisatoshi
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feebase: Option<u64>,
    /// Parts per million fee
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feeppm: Option<u64>,
    /// Minimum HTLC in millisatoshi
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub htlcmin: Option<u64>,
    /// Maximum HTLC in millisatoshi
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub htlcmax: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateInvoiceParams {
    /// Amount in millisatoshi (or "any" for zero-amount invoice)
    pub amount_msat: String,
    /// Unique label for the invoice
    pub label: String,
    /// Description shown on the invoice
    pub description: String,
    /// Expiry in seconds (default 3600)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PayInvoiceParams {
    /// The bolt11 invoice string to pay
    pub bolt11: String,
    /// Optional amount in millisatoshi (for zero-amount invoices)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_msat: Option<u64>,
    /// Optional label for tracking the payment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct KeysendParams {
    /// The pubkey of the destination node
    pub destination: String,
    /// Amount in millisatoshi
    pub amount_msat: u64,
    /// Optional label for tracking
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WithdrawParams {
    /// The on-chain address to send funds to
    pub destination: String,
    /// Amount in satoshis, or "all" to withdraw everything
    pub satoshi: String,
    /// Optional fee rate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feerate: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NewAddressParams {
    /// Address type: "bech32" or "p2tr" (default "bech32")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresstype: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SignMessageParams {
    /// The message to sign
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateRuneParams {
    /// Optional restrictions as an array of arrays of condition strings,
    /// e.g. [["method=getinfo"], ["rate=60"]]. Each inner array is a set of
    /// alternative conditions for one restriction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Vec<Vec<String>>>,
    /// If true, create a read-only rune
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readonly: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateOfferParams {
    /// Description of the offer
    pub description: String,
    /// Amount in millisatoshi (optional for any-amount offers)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// Optional label for the offer
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DisableOfferParams {
    /// The offer id to disable
    pub offer_id: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FetchInvoiceParams {
    /// The bolt12 offer string
    pub offer: String,
    /// Optional amount in millisatoshi (for any-amount offers)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_msat: Option<String>,
    /// Optional quantity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
}

// === xpay / askrene tools ===

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct XpayParams {
    /// bolt11 or bolt12 invoice, a bolt12 offer, or a BIP353 name
    pub invstring: String,
    /// Optional amount in millisatoshi (for zero-amount invoices)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_msat: Option<u64>,
    /// Absolute fee limit in millisatoshi (default 5000msat or 1%, whichever is greater)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maxfee: Option<u64>,
    /// Askrene layers to apply (e.g. ["mylayer"]). Can alter topology or bias channels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<String>>,
    /// Seconds to keep retrying (default 60)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_for: Option<u32>,
    /// Pay only part of the invoice
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partial_msat: Option<u64>,
    /// Max blocks the payment may be delayed (default 2016)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maxdelay: Option<u32>,
    /// Optional label for tracking (added v26.06)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Message sent to payee within an invoice request (added v26.04)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payer_note: Option<String>,
    /// Link payment to a local invoice_request offer (added v26.06)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub localinvreqid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AskreneCreateLayerParams {
    /// Name of the layer to create
    pub layer: String,
    /// If true, save and restore layer across restarts (default false)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub persistent: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AskreneUpdateChannelParams {
    /// Name of the layer to apply changes to
    pub layer: String,
    /// Channel and direction, e.g. "564334x877x1/0"
    pub short_channel_id_dir: String,
    /// Whether the channel is usable at all
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Minimum HTLC value in millisatoshi
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub htlc_minimum_msat: Option<String>,
    /// Maximum HTLC value in millisatoshi
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub htlc_maximum_msat: Option<String>,
    /// Base fee in millisatoshi
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee_base_msat: Option<String>,
    /// Proportional fee in parts per million
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee_proportional_millionths: Option<u32>,
    /// CLTV expiry delta in blocks
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cltv_expiry_delta: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AskreneBiasChannelParams {
    /// Name of the layer to apply bias to
    pub layer: String,
    /// Channel and direction, e.g. "116x1x1/1"
    pub short_channel_id_dir: String,
    /// Bias value: positive favors, negative disfavors (-100 to +100)
    pub bias: i32,
    /// Optional description annotation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// If true, add to previous bias value (default false)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relative: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AskreneBiasNodeParams {
    /// Name of the layer to apply bias to
    pub layer: String,
    /// Node pubkey to bias
    pub node: String,
    /// "in" for incoming channels, "out" for outgoing channels
    pub direction: String,
    /// Bias value: positive favors, negative disfavors (-100 to +100)
    pub bias: i32,
    /// Optional description annotation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// If true, add to previous bias value (default false)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relative: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AskreneListLayersParams {
    /// Optional layer name to filter by
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AskreneRemoveLayerParams {
    /// Name of the layer to remove
    pub layer: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AskreneDisableNodeParams {
    /// Name of the layer to apply to
    pub layer: String,
    /// Node pubkey to disable
    pub node: String,
}

// === Splicing ===

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SpliceParams {
    /// Splice script to execute, e.g. "wallet -> 10K; 100% -> 8338aef0"
    pub script_or_json: String,
    /// Don't execute, just output what would be done
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dryrun: Option<bool>,
    /// Skip fee safety check if true
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force_feerate: Option<bool>,
    /// Add verbose calculation log to result
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debug_log: Option<bool>,
    /// Execute up until signatures would be sent, then abort (dev only)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dev_wetrun: Option<bool>,
}

// === Filter params for read-only tools ===

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListPeersParams {
    /// Optional pubkey to filter by
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Log level for peer logs: "io", "trace", "debug", "info", or "unusual"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListNodesParams {
    /// Optional pubkey to filter by
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListChannelsParams {
    /// Optional source node pubkey
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Optional destination node pubkey
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// Optional short channel ID (e.g. "899892x1091x0")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub short_channel_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListForwardsParams {
    /// Filter by status: "offered", "settled", "failed", or "local_failed"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Filter by incoming channel short ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_channel: Option<String>,
    /// Filter by outgoing channel short ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub out_channel: Option<String>,
    /// Index ordering: "created" or "updated" (default "created")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
    /// Start index for pagination (requires index)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    /// Maximum number of entries to return (requires index)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListInvoicesParams {
    /// Filter by invoice label
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Filter by payment hash
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_hash: Option<String>,
    /// Filter by offer ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListPaysParams {
    /// Filter by bolt11 invoice string
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bolt11: Option<String>,
    /// Filter by status: "pending", "complete", or "failed"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListSendPaysParams {
    /// Filter by bolt11 invoice string
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bolt11: Option<String>,
    /// Filter by status: "pending", "complete", or "failed"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListClosedChannelsParams {
    /// Optional peer pubkey to filter by
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListPeerChannelsParams {
    /// Optional peer pubkey to filter by
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Optional short channel ID (e.g. "899892x1091x0")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub short_channel_id: Option<String>,
    /// Optional channel ID (hash)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListOffersParams {
    /// Optional offer ID (hash) to filter by
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    /// If true, only return active offers
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_only: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListFundsParams {
    /// If true, include spent outputs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spent: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListDatastoreParams {
    /// Optional key to filter by (string or array of strings)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListHtlcsParams {
    /// Optional peer pubkey to filter by
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Index ordering: "created" or "updated"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
    /// Start index for pagination (requires index)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    /// Maximum number of entries to return (requires index)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListConfigsParams {
    /// Optional config option name to filter by
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListAddressesParams {
    /// Optional Bitcoin address to filter by
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Start index for pagination
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    /// Maximum number of entries to return
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

// === Feature filtering ===

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListPeersByFeatureParams {
    /// Feature name: "splice", "option_will_fund", "anchors", "static_remotekey",
    /// "zeroconf", "scid_alias", "route_blinding", or "simple_close"
    pub feature: String,
}

// === Generic tool params ===

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CallRpcMethodParams {
    /// The CLN RPC method name (e.g. "getinfo", "listpeers", "invoice")
    pub method: String,
    /// Optional parameters as a JSON object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}
