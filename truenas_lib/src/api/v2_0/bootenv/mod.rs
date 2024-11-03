const DIR: &str = "/api/v2.0/bootenv";


use crate::server::Server;

/// Retrieves the list of installed apps from the server
/// ```
/// let mut server: Server = Server::new("ip", "key");
/// let result = truenas_lib::get::<AllDisks>(&url, &server.key());
/// println!("{:#?}", result);
/// ```

pub fn get(server: &mut Server) -> Result<AllBootenv, reqwest::Error>
{
    let url = format!("{}{}", server.url(), DIR);
    let res = crate::api_commands::get::<AllBootenv>(&url, &server.key())?;
    Ok(res)
}


use serde::Deserialize;
use serde::Serialize;

type AllBootenv = Vec<Bootenv>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bootenv {
    id: String,
    realname: String,
    name: String,
    active: String,
    activated: bool,
    #[serde(rename = "can_activate")]
    can_activate: bool,
    mountpoint: String,
    space: String,
    created: Created,
    keep: bool,
    rawspace: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Created {
    #[serde(rename = "$date")]
    date: i64,
}