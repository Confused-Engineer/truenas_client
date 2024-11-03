const DIR: &str = "/api/v2.0/interface";


use crate::server::Server;

/// Retrieves the list of installed apps from the server
/// ```
/// let mut server: Server = Server::new("ip", "key");
/// let result = truenas_lib::get::<AllDisks>(&url, &server.key());
/// println!("{:#?}", result);
/// ```

pub fn get(server: &mut Server) -> Result<AllInterfaces, reqwest::Error>
{
    let url = format!("{}{}", server.url(), DIR);
    let res = crate::api_commands::get::<AllInterfaces>(&url, &server.key())?;
    Ok(res)
}


use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;


type AllInterfaces = Vec<Interface>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interface {
    id: String,
    name: String,
    fake: bool,
    #[serde(rename = "type")]
    type_field: String,
    state: State,
    aliases: Vec<Alias2>,
    #[serde(rename = "ipv4_dhcp")]
    ipv4_dhcp: bool,
    #[serde(rename = "ipv6_auto")]
    ipv6_auto: bool,
    description: String,
    mtu: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct State {
    name: String,
    #[serde(rename = "orig_name")]
    orig_name: String,
    description: String,
    mtu: i64,
    cloned: bool,
    flags: Vec<String>,
    #[serde(rename = "nd6_flags")]
    nd6_flags: Vec<String>,
    capabilities: Vec<String>,
    #[serde(rename = "link_state")]
    link_state: String,
    #[serde(rename = "media_type")]
    media_type: String,
    #[serde(rename = "media_subtype")]
    media_subtype: String,
    #[serde(rename = "active_media_type")]
    active_media_type: String,
    #[serde(rename = "active_media_subtype")]
    active_media_subtype: String,
    #[serde(rename = "supported_media")]
    supported_media: Vec<String>,
    #[serde(rename = "media_options")]
    media_options: Value,
    #[serde(rename = "link_address")]
    link_address: String,
    #[serde(rename = "permanent_link_address")]
    permanent_link_address: String,
    #[serde(rename = "hardware_link_address")]
    hardware_link_address: String,
    aliases: Vec<Alias>,
    #[serde(rename = "vrrp_config")]
    vrrp_config: Value,
    #[serde(rename = "rx_queues")]
    rx_queues: i64,
    #[serde(rename = "tx_queues")]
    tx_queues: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Alias {
    #[serde(rename = "type")]
    type_field: String,
    address: String,
    netmask: Option<i64>,
    broadcast: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Alias2 {
    #[serde(rename = "type")]
    type_field: String,
    address: String,
    netmask: i64,
}
