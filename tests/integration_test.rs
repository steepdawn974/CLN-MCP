use cln_mcp::{NodeService, RestBackend};
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
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("lightning_dir"));
        assert!(obj.contains_key("id"));
        assert!(obj.contains_key("alias"));
        assert!(obj.contains_key("version"));
    }
}

#[tokio::test]
async fn test_list_configs() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service.list_configs().await;
    assert!(result.is_ok(), "list_configs failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("configs"));
    }
}

#[tokio::test]
async fn test_list_addresses() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service.list_addresses().await;
    assert!(result.is_ok(), "list_addresses failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("addresses"));
    }
}

#[tokio::test]
async fn test_list_channels() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service.list_channels().await;
    assert!(result.is_ok(), "list_channels failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("channels"));
    }
}

#[tokio::test]
async fn test_list_peer_channels() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service.list_peer_channels().await;
    assert!(
        result.is_ok(),
        "list_peer_channels failed: {:?}",
        result.err()
    );

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("channels"));
    }
}

#[tokio::test]
async fn test_list_closed_channels() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service.list_closed_channels().await;
    assert!(
        result.is_ok(),
        "list_closed_channels failed: {:?}",
        result.err()
    );

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("closedchannels"));
    }
}

#[tokio::test]
async fn test_list_htlcs() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service.list_htlcs().await;
    assert!(result.is_ok(), "list_htlcs failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("htlcs"));
    }
}

#[tokio::test]
async fn test_list_pays() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service.list_pays().await;
    assert!(result.is_ok(), "list_pays failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("pays"));
    }
}

#[tokio::test]
async fn test_list_send_pays() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service.list_send_pays().await;
    assert!(result.is_ok(), "list_send_pays failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("payments"));
    }
}

#[tokio::test]
async fn test_list_forwards() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service.list_forwards().await;
    assert!(result.is_ok(), "list_forwards failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("forwards"));
    }
}

#[tokio::test]
async fn test_list_invoices() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service.list_invoices().await;
    assert!(result.is_ok(), "list_invoices failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("invoices"));
    }
}

#[tokio::test]
async fn test_list_peers() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service.list_peers().await;
    assert!(result.is_ok(), "list_peers failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("peers"));
    }
}

#[tokio::test]
async fn test_list_nodes() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service.list_nodes().await;
    assert!(result.is_ok(), "list_nodes failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("nodes"));
    }
}

#[tokio::test]
async fn test_list_funds() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service.list_funds().await;
    assert!(result.is_ok(), "list_funds failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

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

    let result = service.list_offers().await;
    assert!(result.is_ok(), "list_offers failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("offers"));
    }
}

#[tokio::test]
async fn test_list_datastore() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service.list_datastore().await;
    assert!(result.is_ok(), "list_datastore failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("datastore"));
    }
}

#[tokio::test]
async fn test_feerates() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service.feerates().await;
    assert!(result.is_ok(), "feerates failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("perkw"));
        assert!(obj.contains_key("perkb"));
        assert!(obj.contains_key("onchain_fee_estimates"));
    }
}

#[tokio::test]
async fn test_get_log() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service.get_log().await;
    assert!(result.is_ok(), "get_log failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("log"));
    }
}

#[tokio::test]
async fn test_bkpr_channels_apy() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service.bkpr_channels_apy().await;
    assert!(
        result.is_ok(),
        "bkpr_channels_apy failed: {:?}",
        result.err()
    );

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("channels_apy"));
    }
}

#[tokio::test]
async fn test_bkpr_list_balances() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service.bkpr_list_balances().await;
    assert!(
        result.is_ok(),
        "bkpr_list_balances failed: {:?}",
        result.err()
    );

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("accounts"));
    }
}

#[tokio::test]
async fn test_bkpr_list_income() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service.bkpr_list_income().await;
    assert!(
        result.is_ok(),
        "bkpr_list_income failed: {:?}",
        result.err()
    );

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("income_events"));
    }
}

#[tokio::test]
async fn test_bkpr_list_account_events() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service.bkpr_list_account_events().await;
    assert!(
        result.is_ok(),
        "bkpr_list_account_events failed: {:?}",
        result.err()
    );

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("events"));
    }
}

// === New parameterized tool tests ===

#[tokio::test]
async fn test_decode() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    // An incomplete bolt11 string must produce an ErrorData, not a panic
    let result = service
        .decode(rmcp::handler::server::wrapper::Parameters(
            cln_mcp::client::params::DecodeParams {
                string: "lnbcrt1".to_string(),
            },
        ))
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
        .show_runes(rmcp::handler::server::wrapper::Parameters(
            cln_mcp::client::params::ShowRunesParams { rune: None },
        ))
        .await;
    assert!(result.is_ok(), "show_runes failed: {:?}", result.err());
}

#[tokio::test]
async fn test_call_rpc_method() {
    let Some(service) = setup_rest_env().await else {
        eprintln!("Skipping: CLN_RUNE not set");
        return;
    };

    let result = service
        .call_rpc_method(rmcp::handler::server::wrapper::Parameters(
            cln_mcp::client::params::CallRpcMethodParams {
                method: "getinfo".to_string(),
                params: Some(json!({})),
            },
        ))
        .await;
    assert!(result.is_ok(), "call_rpc_method failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

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
        .call_rpc_method(rmcp::handler::server::wrapper::Parameters(
            cln_mcp::client::params::CallRpcMethodParams {
                method: "listpeers".to_string(),
                params: Some(json!({})),
            },
        ))
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
        .call_rpc_method(rmcp::handler::server::wrapper::Parameters(
            cln_mcp::client::params::CallRpcMethodParams {
                method: "feerates".to_string(),
                params: Some(json!({"style": "perkb"})),
            },
        ))
        .await;
    assert!(result.is_ok(), "call_rpc_method failed: {:?}", result.err());
}
