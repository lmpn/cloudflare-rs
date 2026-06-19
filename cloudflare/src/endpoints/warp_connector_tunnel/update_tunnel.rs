use serde::Serialize;
use serde_with::serde_as;

use crate::framework::response::ApiSuccess;
use crate::{
    endpoints::warp_connector_tunnel::data_structures::WarpConnectorTunnel,
    framework::endpoint::{EndpointSpec, Method, RequestBody},
};

/// Update a Warp Connector tunnel
/// <https://developers.cloudflare.com/api/resources/zero_trust/subresources/tunnels/subresources/warp_connector/methods/update>
#[derive(Debug)]
pub struct UpdateTunnel<'a> {
    pub account_identifier: &'a str,
    pub tunnel_id: &'a str,
    pub params: Params<'a>,
}

impl EndpointSpec for UpdateTunnel<'_> {
    type JsonResponse = WarpConnectorTunnel;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PATCH
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/warp_connector/{}",
            self.account_identifier, self.tunnel_id
        )
    }
    #[inline]
    fn body(&'_ self) -> Option<RequestBody<'_>> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

/// Params for updating a Warp Connector Tunnel
#[serde_as]
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct Params<'a> {
    /// The name for the Tunnel to be updated. It must be unique within the account.
    pub name: &'a str,
    pub tunnel_secret: &'a str,
}
