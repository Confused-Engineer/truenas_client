const APP: &str = "/api/v2.0/app/upgrade_summary";

use crate::server::Server;


pub fn post(server: &mut Server, app: &str) -> Result<AvailableUpgrade, reqwest::Error>
{
    let url = format!("{}{}", server.url(), APP);
    let body = format!("{{ \"app_name\":\"{}\", \"options\": {{}} }}", app);

    let res = crate::api_commands::post::<AvailableUpgrade>(&url, &server.key(), body)?;
    Ok(res)
    
}


use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableUpgrade {
    #[serde(rename = "latest_version")]
    pub latest_version: String,
    #[serde(rename = "latest_human_version")]
    pub latest_human_version: String,
    #[serde(rename = "upgrade_version")]
    pub upgrade_version: String,
    #[serde(rename = "upgrade_human_version")]
    pub upgrade_human_version: String,
    pub changelog: Value,
    #[serde(rename = "available_versions_for_upgrade")]
    pub available_versions_for_upgrade: Vec<AvailableVersionsForUpgrade>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableVersionsForUpgrade {
    pub version: String,
    #[serde(rename = "human_version")]
    pub human_version: String,
}
