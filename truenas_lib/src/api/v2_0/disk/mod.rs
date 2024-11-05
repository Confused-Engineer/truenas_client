const DIR: &str = "/api/v2.0/disk";


use crate::server::Server;

/// Retrieves the list of installed apps from the server
/// ```
/// let mut server: Server = Server::new("ip", "key");
/// let result = truenas_lib::get::<AllDisks>(&url, &server.key());
/// println!("{:#?}", result);
/// ```

pub fn get(server: &mut Server) -> Result<AllDisks, reqwest::Error>
{
    let url = format!("{}{}", server.url(), DIR);
    let res = crate::api_commands::get::<AllDisks>(&url, &server.key())?;
    Ok(res)
}


impl Disk
{
    pub fn get_name(&mut self) -> String
    {
        self.name.clone()
    }

    pub fn get_serial(&mut self) -> String
    {
        self.serial.clone()
    }

    pub fn get_model(&mut self) -> String
    {
        self.model.clone()
    }

    pub fn get_capaticity(&mut self) -> i64
    {
        self.size / 1000000000
    }
}




use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

pub type AllDisks = Vec<Disk>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Disk {
    identifier: String,
    name: String,
    subsystem: String,
    number: i64,
    serial: String,
    lunid: String,
    size: i64,
    description: String,
    transfermode: String,
    hddstandby: String,
    advpowermgmt: String,
    togglesmart: bool,
    smartoptions: String,
    expiretime: Value,
    critical: Value,
    difference: Value,
    informational: Value,
    model: String,
    rotationrate: Option<i64>,
    #[serde(rename = "type")]
    type_field: String,
    #[serde(rename = "zfs_guid")]
    zfs_guid: Option<String>,
    bus: String,
    devname: String,
    enclosure: Value,
    #[serde(rename = "supports_smart")]
    supports_smart: Value,
    pool: Value,
}

