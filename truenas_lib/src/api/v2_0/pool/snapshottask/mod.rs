const DIR: &str = "/api/v2.0/pool/snapshottask";

use crate::server::Server;

pub fn get(server: &mut Server) -> Result<SnapshotTasks, reqwest::Error>
{
    let url = format!("{}{}", server.url(), DIR);
    let res = crate::api_commands::get::<SnapshotTasks>(&url, &server.key())?;
    Ok(res)
}




use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

type SnapshotTasks = Vec<Task>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    id: i64,
    dataset: String,
    recursive: bool,
    #[serde(rename = "lifetime_value")]
    lifetime_value: i64,
    #[serde(rename = "lifetime_unit")]
    lifetime_unit: String,
    enabled: bool,
    exclude: Vec<Value>,
    #[serde(rename = "naming_schema")]
    naming_schema: String,
    #[serde(rename = "allow_empty")]
    allow_empty: bool,
    schedule: Schedule,
    #[serde(rename = "vmware_sync")]
    vmware_sync: bool,
    state: State,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Schedule {
    minute: String,
    hour: String,
    dom: String,
    month: String,
    dow: String,
    begin: String,
    end: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct State {
    state: String,
    datetime: Datetime,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Datetime {
    #[serde(rename = "$date")]
    date: i64,
}