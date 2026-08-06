use cln_mcp::client::params::*;
use cln_mcp::{NodeService, RestBackend};
use rmcp::handler::server::wrapper::Parameters;
use serde_json::{json, Value};
use std::sync::Arc;

/// Setup a REST-based test environment.
/// Returns None when CLN_RUNE is not set, so tests skip gracefully without a live node.
/// Requires a running CLN node with clnrest enabled (default https://localhost:3010).
/// Set the CLN_RUNE environment variable to the rune token, CLN_REST_URL to override the endpoint.
async fn setup_rest_env() -> Option<NodeService> {
    let rune = std::env::var("CLN_RUNE").ok()?;
    let url =
        std::env::var("CLN_REST_URL").unwrap_or_else(|_| "https://localhost:3010".to_string());

    let backend = RestBackend::new(&url, &rune, None)
        .await
        .expect("Failed to create REST backend");
    Some(NodeService::new(Arc::new(backend)))
}

/// Helper to extract the JSON response body from a CallToolResult.
fn extract_json(response: &rmcp::model::CallToolResult) -> Value {
    let content = response.content.first().expect("Empty response!");
    serde_json::from_str(&content.as_text().unwrap().text).unwrap()
}

/// Helper to extract the JSON object from a CallToolResult.
fn extract_obj(response: &rmcp::model::CallToolResult) -> serde_json::Map<String, Value> {
    extract_json(response).as_object().unwrap().clone()
}

#[tokio::test]
async fn test_server_initialization() {
    let Some(_service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };
}

// === Read-only tool tests ===

#[tokio::test]
async fn test_get_info() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service.get_info().await;
    assert!(result.is_ok(), "get_info failed: {:?}", result.err());

    if let Ok(response) = result {
        let obj = extract_obj(&response);

        // CLN docs: getinfo returns id, alias, version, blockheight, network, num_peers,
        // num_active_channels, fees_collected_msat, address, binding
        assert!(obj.contains_key("id"));
        assert!(obj.contains_key("alias"));
        assert!(obj.contains_key("version"));
        assert!(obj.contains_key("blockheight"));
        assert!(obj.contains_key("network"));
        assert!(obj.contains_key("num_peers"));
        assert!(obj.contains_key("num_active_channels"));
        assert!(obj.contains_key("fees_collected_msat"));
        assert!(obj.contains_key("address"));
        assert!(obj.contains_key("binding"));
    }
}

#[tokio::test]
async fn test_list_configs() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service
        .list_configs(Parameters(ListConfigsParams { config: None }))
        .await;
    assert!(result.is_ok(), "list_configs failed: {:?}", result.err());

    if let Ok(response) = result {
        let obj = extract_obj(&response);
        assert!(obj.contains_key("configs"));
    }
}

#[tokio::test]
async fn test_list_configs_filtered() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service
        .list_configs(Parameters(ListConfigsParams {
            config: Some("network".to_string()),
        }))
        .await;
    assert!(
        result.is_ok(),
        "list_configs filtered failed: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_list_addresses() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service
        .list_addresses(Parameters(ListAddressesParams {
            address: None,
            start: None,
            limit: None,
        }))
        .await;
    assert!(result.is_ok(), "list_addresses failed: {:?}", result.err());

    if let Ok(response) = result {
        let obj = extract_obj(&response);
        assert!(obj.contains_key("addresses"));
    }
}

#[tokio::test]
async fn test_list_channels() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service
        .list_channels(Parameters(ListChannelsParams {
            source: None,
            destination: None,
            short_channel_id: None,
        }))
        .await;
    assert!(result.is_ok(), "list_channels failed: {:?}", result.err());

    if let Ok(response) = result {
        let obj = extract_obj(&response);
        assert!(obj.contains_key("channels"));
        // CLN docs: each channel has source, destination, short_channel_id, public, amount_msat
        if let Some(channels) = obj.get("channels").and_then(|c| c.as_array()) {
            if !channels.is_empty() {
                let ch = channels[0].as_object().unwrap();
                assert!(ch.contains_key("source"));
                assert!(ch.contains_key("destination"));
                assert!(ch.contains_key("short_channel_id"));
                assert!(ch.contains_key("amount_msat"));
            }
        }
    }
}

#[tokio::test]
async fn test_list_peer_channels() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service
        .list_peer_channels(Parameters(ListPeerChannelsParams {
            id: None,
            short_channel_id: None,
            channel_id: None,
        }))
        .await;
    assert!(
        result.is_ok(),
        "list_peer_channels failed: {:?}",
        result.err()
    );

    if let Ok(response) = result {
        let obj = extract_obj(&response);
        assert!(obj.contains_key("channels"));
        // CLN docs: each peer channel has state, peer_id, channel_id, to_us_msat, total_msat
        if let Some(channels) = obj.get("channels").and_then(|c| c.as_array()) {
            if !channels.is_empty() {
                let ch = channels[0].as_object().unwrap();
                assert!(ch.contains_key("state"));
                assert!(ch.contains_key("peer_id"));
                assert!(ch.contains_key("channel_id"));
            }
        }
    }
}

#[tokio::test]
async fn test_list_closed_channels() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service
        .list_closed_channels(Parameters(ListClosedChannelsParams { id: None }))
        .await;
    assert!(
        result.is_ok(),
        "list_closed_channels failed: {:?}",
        result.err()
    );

    if let Ok(response) = result {
        let obj = extract_obj(&response);
        assert!(obj.contains_key("closedchannels"));
    }
}

#[tokio::test]
async fn test_list_htlcs() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service
        .list_htlcs(Parameters(ListHtlcsParams {
            id: None,
            index: None,
            start: None,
            limit: None,
        }))
        .await;
    assert!(result.is_ok(), "list_htlcs failed: {:?}", result.err());

    if let Ok(response) = result {
        let obj = extract_obj(&response);
        assert!(obj.contains_key("htlcs"));
    }
}

#[tokio::test]
async fn test_list_pays() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service
        .list_pays(Parameters(ListPaysParams {
            bolt11: None,
            status: None,
        }))
        .await;
    assert!(result.is_ok(), "list_pays failed: {:?}", result.err());

    if let Ok(response) = result {
        let obj = extract_obj(&response);
        assert!(obj.contains_key("pays"));
    }
}

#[tokio::test]
async fn test_list_send_pays() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service
        .list_send_pays(Parameters(ListSendPaysParams {
            bolt11: None,
            status: None,
        }))
        .await;
    assert!(result.is_ok(), "list_send_pays failed: {:?}", result.err());

    if let Ok(response) = result {
        let obj = extract_obj(&response);
        assert!(obj.contains_key("payments"));
    }
}

#[tokio::test]
async fn test_list_forwards() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service
        .list_forwards(Parameters(ListForwardsParams {
            status: None,
            in_channel: None,
            out_channel: None,
            index: None,
            start: None,
            limit: None,
        }))
        .await;
    assert!(result.is_ok(), "list_forwards failed: {:?}", result.err());

    if let Ok(response) = result {
        let obj = extract_obj(&response);
        assert!(obj.contains_key("forwards"));
    }
}

#[tokio::test]
async fn test_list_forwards_settled() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service
        .list_forwards(Parameters(ListForwardsParams {
            status: Some("settled".to_string()),
            in_channel: None,
            out_channel: None,
            index: None,
            start: None,
            limit: None,
        }))
        .await;
    assert!(
        result.is_ok(),
        "list_forwards settled failed: {:?}",
        result.err()
    );

    if let Ok(response) = result {
        let obj = extract_obj(&response);
        // CLN docs: listforwards returns forwards array, each with status, in_channel, out_channel, fee_msat
        if let Some(forwards) = obj.get("forwards").and_then(|f| f.as_array()) {
            for fwd in forwards {
                let f = fwd.as_object().unwrap();
                assert_eq!(
                    f.get("status").and_then(|s| s.as_str()),
                    Some("settled"),
                    "Filtered forwards should all be settled"
                );
            }
        }
    }
}

#[tokio::test]
async fn test_list_invoices() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service
        .list_invoices(Parameters(ListInvoicesParams {
            label: None,
            payment_hash: None,
            offer_id: None,
        }))
        .await;
    assert!(result.is_ok(), "list_invoices failed: {:?}", result.err());

    if let Ok(response) = result {
        let obj = extract_obj(&response);
        assert!(obj.contains_key("invoices"));
    }
}

#[tokio::test]
async fn test_list_peers() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service
        .list_peers(Parameters(ListPeersParams {
            id: None,
            level: None,
        }))
        .await;
    assert!(result.is_ok(), "list_peers failed: {:?}", result.err());

    if let Ok(response) = result {
        let obj = extract_obj(&response);
        assert!(obj.contains_key("peers"));
        // CLN docs: each peer has id, connected, features, channels
        if let Some(peers) = obj.get("peers").and_then(|p| p.as_array()) {
            if !peers.is_empty() {
                let peer = peers[0].as_object().unwrap();
                assert!(peer.contains_key("id"));
                assert!(peer.contains_key("connected"));
            }
        }
    }
}

#[tokio::test]
async fn test_list_nodes() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service
        .list_nodes(Parameters(ListNodesParams { id: None }))
        .await;
    assert!(result.is_ok(), "list_nodes failed: {:?}", result.err());

    if let Ok(response) = result {
        let obj = extract_obj(&response);
        assert!(obj.contains_key("nodes"));
        // CLN docs: each node has nodeid, alias, color, features, addresses
        if let Some(nodes) = obj.get("nodes").and_then(|n| n.as_array()) {
            if !nodes.is_empty() {
                let node = nodes[0].as_object().unwrap();
                assert!(node.contains_key("nodeid"));
                assert!(node.contains_key("alias"));
            }
        }
    }
}

#[tokio::test]
async fn test_list_funds() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service
        .list_funds(Parameters(ListFundsParams { spent: None }))
        .await;
    assert!(result.is_ok(), "list_funds failed: {:?}", result.err());

    if let Ok(response) = result {
        let obj = extract_obj(&response);
        // CLN docs: listfunds returns outputs (on-chain) and channels (LN funds)
        assert!(obj.contains_key("outputs"));
        assert!(obj.contains_key("channels"));
    }
}

#[tokio::test]
async fn test_list_offers() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service
        .list_offers(Parameters(ListOffersParams {
            offer_id: None,
            active_only: None,
        }))
        .await;
    assert!(result.is_ok(), "list_offers failed: {:?}", result.err());

    if let Ok(response) = result {
        let obj = extract_obj(&response);
        assert!(obj.contains_key("offers"));
    }
}

#[tokio::test]
async fn test_list_datastore() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service
        .list_datastore(Parameters(ListDatastoreParams { key: None }))
        .await;
    assert!(result.is_ok(), "list_datastore failed: {:?}", result.err());

    if let Ok(response) = result {
        let obj = extract_obj(&response);
        assert!(obj.contains_key("datastore"));
    }
}

#[tokio::test]
async fn test_feerates_perkb() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service
        .feerates(Parameters(FeeratesParams {
            style: "perkb".to_string(),
        }))
        .await;
    assert!(result.is_ok(), "feerates perkb failed: {:?}", result.err());

    if let Ok(response) = result {
        let obj = extract_obj(&response);
        // CLN docs: feerates with perkb returns perkb, onchain_fee_estimates
        assert!(obj.contains_key("perkb"));
        assert!(obj.contains_key("onchain_fee_estimates"));
        // perkb should have estimates array with blockcount and feerate
        let perkb = obj.get("perkb").unwrap().as_object().unwrap();
        assert!(perkb.contains_key("estimates"));
        assert!(perkb.contains_key("min_acceptable"));
        assert!(perkb.contains_key("max_acceptable"));
    }
}

#[tokio::test]
async fn test_feerates_perkw() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service
        .feerates(Parameters(FeeratesParams {
            style: "perkw".to_string(),
        }))
        .await;
    assert!(result.is_ok(), "feerates perkw failed: {:?}", result.err());

    if let Ok(response) = result {
        let obj = extract_obj(&response);
        // CLN docs: feerates with perkw returns perkw, onchain_fee_estimates
        assert!(obj.contains_key("perkw"));
        assert!(obj.contains_key("onchain_fee_estimates"));
    }
}

#[tokio::test]
async fn test_get_log() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service
        .get_log(Parameters(GetLogParams { level: None }))
        .await;
    assert!(result.is_ok(), "get_log failed: {:?}", result.err());

    if let Ok(response) = result {
        let obj = extract_obj(&response);
        // CLN docs: getlog returns log array, bytes_max, bytes_used, created_at
        assert!(obj.contains_key("log"));
        assert!(obj.contains_key("bytes_max"));
        assert!(obj.contains_key("bytes_used"));
        assert!(obj.contains_key("created_at"));
    }
}

#[tokio::test]
async fn test_get_log_filtered() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service
        .get_log(Parameters(GetLogParams {
            level: Some("broken".to_string()),
        }))
        .await;
    assert!(
        result.is_ok(),
        "get_log broken failed: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_list_transactions() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service.list_transactions().await;
    assert!(
        result.is_ok(),
        "list_transactions failed: {:?}",
        result.err()
    );

    if let Ok(response) = result {
        let obj = extract_obj(&response);
        // CLN docs: listtransactions returns transactions array
        assert!(obj.contains_key("transactions"));
    }
}

// === Composite tool tests ===

#[tokio::test]
async fn test_node_health() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service.node_health().await;
    assert!(result.is_ok(), "node_health failed: {:?}", result.err());

    if let Ok(response) = result {
        let obj = extract_obj(&response);
        // Composite of getinfo + feerates + listpeerchannels
        assert!(obj.contains_key("version"));
        assert!(obj.contains_key("blockheight"));
        assert!(obj.contains_key("network"));
        assert!(obj.contains_key("num_peers"));
        assert!(obj.contains_key("num_active_channels"));
        assert!(obj.contains_key("num_inactive_channels"));
        assert!(obj.contains_key("pending_htlcs"));
        assert!(obj.contains_key("fees_collected_msat"));
        assert!(obj.contains_key("feerates"));
    }
}

#[tokio::test]
async fn test_channel_summary() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service.channel_summary().await;
    assert!(result.is_ok(), "channel_summary failed: {:?}", result.err());

    if let Ok(response) = result {
        let obj = extract_obj(&response);
        assert!(obj.contains_key("total_capacity_msat"));
        assert!(obj.contains_key("active_channels"));
        assert!(obj.contains_key("inactive_channels"));
        assert!(obj.contains_key("channels"));
        assert!(obj.contains_key("offline_peers"));
    }
}

#[tokio::test]
async fn test_fee_report() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service.fee_report().await;
    assert!(result.is_ok(), "fee_report failed: {:?}", result.err());

    if let Ok(response) = result {
        let obj = extract_obj(&response);
        assert!(obj.contains_key("total_fees_earned_msat"));
        assert!(obj.contains_key("total_forwards"));
        assert!(obj.contains_key("per_channel"));
    }
}

#[tokio::test]
async fn test_list_peers_by_feature() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service
        .list_peers_by_feature(Parameters(ListPeersByFeatureParams {
            feature: "splice".to_string(),
        }))
        .await;
    assert!(
        result.is_ok(),
        "list_peers_by_feature failed: {:?}",
        result.err()
    );

    if let Ok(response) = result {
        let obj = extract_obj(&response);
        assert!(obj.contains_key("feature"));
        assert!(obj.contains_key("matching_peers"));
        assert!(obj.contains_key("peers"));
    }
}

#[tokio::test]
async fn test_list_splice_peers() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service.list_splice_peers().await;
    assert!(
        result.is_ok(),
        "list_splice_peers failed: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_list_splice_nodes() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service.list_splice_nodes().await;
    assert!(
        result.is_ok(),
        "list_splice_nodes failed: {:?}",
        result.err()
    );
}

// === Parameterized tool tests ===

#[tokio::test]
async fn test_decode() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    // An incomplete bolt11 string must produce an ErrorData, not a panic
    let result = service
        .decode(Parameters(DecodeParams {
            string: "lnbcrt1".to_string(),
        }))
        .await;
    assert!(
        result.is_err(),
        "decode of an invalid string should return an error, got: {:?}",
        result.ok()
    );
}

#[tokio::test]
async fn test_show_runes() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service
        .show_runes(Parameters(ShowRunesParams { rune: None }))
        .await;
    assert!(result.is_ok(), "show_runes failed: {:?}", result.err());

    if let Ok(response) = result {
        let obj = extract_obj(&response);
        // CLN docs: showrunes returns runes array
        assert!(obj.contains_key("runes"));
    }
}

#[tokio::test]
async fn test_call_rpc_method() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service
        .call_rpc_method(Parameters(CallRpcMethodParams {
            method: "getinfo".to_string(),
            params: Some(json!({})),
        }))
        .await;
    assert!(result.is_ok(), "call_rpc_method failed: {:?}", result.err());

    if let Ok(response) = result {
        let obj = extract_obj(&response);
        assert!(obj.contains_key("id"));
        assert!(obj.contains_key("alias"));
    }
}

#[tokio::test]
async fn test_call_rpc_method_listpeers() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service
        .call_rpc_method(Parameters(CallRpcMethodParams {
            method: "listpeers".to_string(),
            params: Some(json!({})),
        }))
        .await;
    assert!(result.is_ok(), "call_rpc_method failed: {:?}", result.err());
}

#[tokio::test]
async fn test_call_rpc_method_with_params() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service
        .call_rpc_method(Parameters(CallRpcMethodParams {
            method: "feerates".to_string(),
            params: Some(json!({"style": "perkb"})),
        }))
        .await;
    assert!(result.is_ok(), "call_rpc_method failed: {:?}", result.err());
}
