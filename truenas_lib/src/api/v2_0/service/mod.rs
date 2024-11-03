const DIR: &str = "/api/v2.0/service";


use crate::server::Server;

pub fn get(server: &mut Server) -> Result<AllServices, reqwest::Error>
{
    let url = format!("{}{}", server.url(), DIR);
    let res = crate::api_commands::get::<AllServices>(&url, &server.key())?;
    Ok(res)
}




use serde::Deserialize;
use serde::Serialize;

type AllServices = Vec<Service>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    id: i64,
    service: String,
    enable: bool,
    state: String,
    pids: Vec<i64>,
}