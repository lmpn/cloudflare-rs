use chrono::{offset::Utc, DateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::endpoints::cfd_tunnel::{ActiveConnection, TunnelStatusType};
use crate::framework::response::ApiResult;

/// A Warp Connector Tunnel.
///
/// Mirrors the JSON returned by the Cloudflare API when creating or fetching a
/// warp_connector tunnel, including its credentials file and connection token.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WarpConnectorTunnel {
    pub id: Uuid,
    pub account_tag: String,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub name: String,
    pub connections: Vec<ActiveConnection>,
    pub conns_active_at: Option<DateTime<Utc>>,
    pub conns_inactive_at: Option<DateTime<Utc>>,
    pub tun_type: String,
    pub metadata: serde_json::Value,
    pub status: TunnelStatusType,
    /// Present on create responses; absent on delete responses.
    #[serde(flatten)]
    pub credentials: Option<WarpConnectorCredentials>,
}

/// Credentials bundle returned only when a Warp Connector Tunnel is created.
///
/// Flattened into [`WarpConnectorTunnel`] so it inlines `credentials_file` and
/// `token` at the top level of the JSON.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WarpConnectorCredentials {
    pub credentials_file: WarpConnectorCredentialsFile,
    pub token: String,
}

/// Credentials file contents for a Warp Connector Tunnel.
///
/// Field names follow the API's PascalCase convention.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WarpConnectorCredentialsFile {
    #[serde(rename = "AccountTag")]
    pub account_tag: String,
    #[serde(rename = "TunnelID")]
    pub tunnel_id: Uuid,
    #[serde(rename = "TunnelName")]
    pub tunnel_name: String,
    #[serde(rename = "TunnelSecret")]
    pub tunnel_secret: String,
}

impl ApiResult for WarpConnectorTunnel {}
impl ApiResult for Vec<WarpConnectorTunnel> {}
