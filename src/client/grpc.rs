use anyhow::{anyhow, Result};
use async_trait::async_trait;
use cln_grpc::pb::node_client::NodeClient;
use cln_grpc::pb::*;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{transport::Channel, Request};

use super::backend::ClnBackend;

pub struct GrpcBackend {
    client: Arc<Mutex<NodeClient<Channel>>>,
}

impl GrpcBackend {
    pub fn new(channel: Channel) -> Self {
        Self {
            client: Arc::new(Mutex::new(NodeClient::new(channel))),
        }
    }
}

macro_rules! grpc_call {
    ($self:expr, $method:ident, $req:expr) => {{
        let mut client = $self.client.lock().await;
        let res = client.$method(Request::new($req)).await;
        match res {
            Ok(response) => serde_json::to_value(response.into_inner())
                .map_err(|e| anyhow!("Failed to serialize gRPC response: {}", e)),
            Err(e) => Err(anyhow!("gRPC call failed: {}", e)),
        }
    }};
}

#[async_trait]
impl ClnBackend for GrpcBackend {
    async fn call(&self, method: &str, _params: Value) -> Result<Value> {
        let mut client = self.client.lock().await;

        match method {
            "getinfo" => {
                let res = client.getinfo(Request::new(GetinfoRequest::default())).await;
                match res {
                    Ok(r) => serde_json::to_value(r.into_inner())
                        .map_err(|e| anyhow!("Failed to serialize: {}", e)),
                    Err(e) => Err(anyhow!("gRPC call failed: {}", e)),
                }
            }
            "listconfigs" => {
                let res = client.list_configs(Request::new(ListconfigsRequest::default())).await;
                match res {
                    Ok(r) => serde_json::to_value(r.into_inner())
                        .map_err(|e| anyhow!("Failed to serialize: {}", e)),
                    Err(e) => Err(anyhow!("gRPC call failed: {}", e)),
                }
            }
            "listaddresses" => {
                let res = client.list_addresses(Request::new(ListaddressesRequest::default())).await;
                match res {
                    Ok(r) => serde_json::to_value(r.into_inner())
                        .map_err(|e| anyhow!("Failed to serialize: {}", e)),
                    Err(e) => Err(anyhow!("gRPC call failed: {}", e)),
                }
            }
            "listchannels" => {
                let res = client.list_channels(Request::new(ListchannelsRequest::default())).await;
                match res {
                    Ok(r) => serde_json::to_value(r.into_inner())
                        .map_err(|e| anyhow!("Failed to serialize: {}", e)),
                    Err(e) => Err(anyhow!("gRPC call failed: {}", e)),
                }
            }
            "listpeerchannels" => {
                let res = client.list_peer_channels(Request::new(ListpeerchannelsRequest::default())).await;
                match res {
                    Ok(r) => serde_json::to_value(r.into_inner())
                        .map_err(|e| anyhow!("Failed to serialize: {}", e)),
                    Err(e) => Err(anyhow!("gRPC call failed: {}", e)),
                }
            }
            "listclosedchannels" => {
                let res = client.list_closed_channels(Request::new(ListclosedchannelsRequest::default())).await;
                match res {
                    Ok(r) => serde_json::to_value(r.into_inner())
                        .map_err(|e| anyhow!("Failed to serialize: {}", e)),
                    Err(e) => Err(anyhow!("gRPC call failed: {}", e)),
                }
            }
            "listhtlcs" => {
                let res = client.list_htlcs(Request::new(ListhtlcsRequest::default())).await;
                match res {
                    Ok(r) => serde_json::to_value(r.into_inner())
                        .map_err(|e| anyhow!("Failed to serialize: {}", e)),
                    Err(e) => Err(anyhow!("gRPC call failed: {}", e)),
                }
            }
            "listpays" => {
                let res = client.list_pays(Request::new(ListpaysRequest::default())).await;
                match res {
                    Ok(r) => serde_json::to_value(r.into_inner())
                        .map_err(|e| anyhow!("Failed to serialize: {}", e)),
                    Err(e) => Err(anyhow!("gRPC call failed: {}", e)),
                }
            }
            "listsendpays" => {
                let res = client.list_send_pays(Request::new(ListsendpaysRequest::default())).await;
                match res {
                    Ok(r) => serde_json::to_value(r.into_inner())
                        .map_err(|e| anyhow!("Failed to serialize: {}", e)),
                    Err(e) => Err(anyhow!("gRPC call failed: {}", e)),
                }
            }
            "listforwards" => {
                let res = client.list_forwards(Request::new(ListforwardsRequest::default())).await;
                match res {
                    Ok(r) => serde_json::to_value(r.into_inner())
                        .map_err(|e| anyhow!("Failed to serialize: {}", e)),
                    Err(e) => Err(anyhow!("gRPC call failed: {}", e)),
                }
            }
            "listinvoices" => {
                let res = client.list_invoices(Request::new(ListinvoicesRequest::default())).await;
                match res {
                    Ok(r) => serde_json::to_value(r.into_inner())
                        .map_err(|e| anyhow!("Failed to serialize: {}", e)),
                    Err(e) => Err(anyhow!("gRPC call failed: {}", e)),
                }
            }
            "listpeers" => {
                let res = client.list_peers(Request::new(ListpeersRequest::default())).await;
                match res {
                    Ok(r) => serde_json::to_value(r.into_inner())
                        .map_err(|e| anyhow!("Failed to serialize: {}", e)),
                    Err(e) => Err(anyhow!("gRPC call failed: {}", e)),
                }
            }
            "listnodes" => {
                let res = client.list_nodes(Request::new(ListnodesRequest::default())).await;
                match res {
                    Ok(r) => serde_json::to_value(r.into_inner())
                        .map_err(|e| anyhow!("Failed to serialize: {}", e)),
                    Err(e) => Err(anyhow!("gRPC call failed: {}", e)),
                }
            }
            "listfunds" => {
                let res = client.list_funds(Request::new(ListfundsRequest::default())).await;
                match res {
                    Ok(r) => serde_json::to_value(r.into_inner())
                        .map_err(|e| anyhow!("Failed to serialize: {}", e)),
                    Err(e) => Err(anyhow!("gRPC call failed: {}", e)),
                }
            }
            "listoffers" => {
                let res = client.list_offers(Request::new(ListoffersRequest::default())).await;
                match res {
                    Ok(r) => serde_json::to_value(r.into_inner())
                        .map_err(|e| anyhow!("Failed to serialize: {}", e)),
                    Err(e) => Err(anyhow!("gRPC call failed: {}", e)),
                }
            }
            "listdatastore" => {
                let res = client.list_datastore(Request::new(ListdatastoreRequest::default())).await;
                match res {
                    Ok(r) => serde_json::to_value(r.into_inner())
                        .map_err(|e| anyhow!("Failed to serialize: {}", e)),
                    Err(e) => Err(anyhow!("gRPC call failed: {}", e)),
                }
            }
            "feerates" => {
                let res = client.feerates(Request::new(FeeratesRequest::default())).await;
                match res {
                    Ok(r) => serde_json::to_value(r.into_inner())
                        .map_err(|e| anyhow!("Failed to serialize: {}", e)),
                    Err(e) => Err(anyhow!("gRPC call failed: {}", e)),
                }
            }
            "getlog" => {
                let res = client.get_log(Request::new(GetlogRequest::default())).await;
                match res {
                    Ok(r) => serde_json::to_value(r.into_inner())
                        .map_err(|e| anyhow!("Failed to serialize: {}", e)),
                    Err(e) => Err(anyhow!("gRPC call failed: {}", e)),
                }
            }
            "bkpr-channelsapy" => {
                let res = client.bkpr_channels_apy(Request::new(BkprchannelsapyRequest::default())).await;
                match res {
                    Ok(r) => serde_json::to_value(r.into_inner())
                        .map_err(|e| anyhow!("Failed to serialize: {}", e)),
                    Err(e) => Err(anyhow!("gRPC call failed: {}", e)),
                }
            }
            "bkpr-listbalances" => {
                let res = client.bkpr_list_balances(Request::new(BkprlistbalancesRequest::default())).await;
                match res {
                    Ok(r) => serde_json::to_value(r.into_inner())
                        .map_err(|e| anyhow!("Failed to serialize: {}", e)),
                    Err(e) => Err(anyhow!("gRPC call failed: {}", e)),
                }
            }
            "bkpr-listincome" => {
                let res = client.bkpr_list_income(Request::new(BkprlistincomeRequest::default())).await;
                match res {
                    Ok(r) => serde_json::to_value(r.into_inner())
                        .map_err(|e| anyhow!("Failed to serialize: {}", e)),
                    Err(e) => Err(anyhow!("gRPC call failed: {}", e)),
                }
            }
            "bkpr-listaccountevents" => {
                let res = client.bkpr_list_account_events(Request::new(BkprlistaccounteventsRequest::default())).await;
                match res {
                    Ok(r) => serde_json::to_value(r.into_inner())
                        .map_err(|e| anyhow!("Failed to serialize: {}", e)),
                    Err(e) => Err(anyhow!("gRPC call failed: {}", e)),
                }
            }
            _ => Err(anyhow!(
                "Method '{}' is not supported by the gRPC backend. Use the REST backend for full method coverage.",
                method
            )),
        }
    }
}
