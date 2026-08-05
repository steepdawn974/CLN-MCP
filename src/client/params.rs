use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// === Read-only tool params (optional filters) ===

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetRouteParams {
    /// The pubkey of the destination node
    pub id: String,
    /// Amount to send in millisatoshi
    pub amount_msat: String,
    /// Risk factor (default 0)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub riskfactor: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DecodeParams {
    /// The string to decode (bolt11, bolt12, rune, etc.)
    pub string: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DecodePayParams {
    /// The bolt11 invoice string to decode
    pub bolt11: String,
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
pub struct ListTransactionsParams {
    /// Start index for pagination
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    /// Maximum number of entries to return
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
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
    pub amount: String,
    /// Fee rate for the funding transaction
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feerate: Option<String>,
    /// Whether to announce the channel (default true)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub announce: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CloseChannelParams {
    /// The pubkey or channel_id of the channel to close
    pub id: String,
    /// Timeout in seconds for unilateral close (0 = immediate)
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
    pub feebase: Option<String>,
    /// Parts per million fee
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feeppm: Option<String>,
    /// Minimum HTLC in millisatoshi
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub htlcmin: Option<String>,
    /// Maximum HTLC in millisatoshi
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub htlcmax: Option<String>,
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
    pub amount_msat: Option<String>,
    /// Optional label for tracking the payment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct KeysendParams {
    /// The pubkey of the destination node
    pub destination: String,
    /// Amount in millisatoshi
    pub amount_msat: String,
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
    /// Optional restrictions array (e.g. [["method=getinfo"]])
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Vec<String>>,
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

// === Generic tool params ===

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CallRpcMethodParams {
    /// The CLN RPC method name (e.g. "getinfo", "listpeers", "invoice")
    pub method: String,
    /// Optional parameters as a JSON object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}
