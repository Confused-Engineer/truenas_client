const DIR: &str = "/api/v2.0/service";


use crate::server::Server;

pub fn get(server: &mut Server) -> Result<AllServices, reqwest::Error>
{
    let url = format!("{}{}", server.url(), DIR);
    let res = crate::api_commands::get::<AllServices>(&url, &server.key())?;
    Ok(res)
}

impl Service
{
    pub fn get_id(&mut self) -> i64
    {
        self.id.clone()
    }

    pub fn get_service(&mut self) -> String
    {
        self.service.clone()
    }

    pub fn is_enabled(&mut self) -> bool
    {
        self.enable.clone()
    }

    pub fn get_state(&mut self) -> String
    {
        self.state.clone()
    }
}


use serde::Deserialize;
use serde::Serialize;

pub type AllServices = Vec<Service>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    id: i64,
    service: String,
    enable: bool,
    state: String,
    pids: Vec<i64>,
}