const DIR: &str = "/api/v2.0/pool/snapshottask";

use crate::server::Server;

pub fn get(server: &mut Server) -> Result<SnapshotTasks, reqwest::Error>
{
    let url = format!("{}{}", server.url(), DIR);
    let res = crate::api_commands::get::<SnapshotTasks>(&url, &server.key())?;
    Ok(res)
}


impl Task
{
    pub fn get_id(&mut self) -> i64
    {
        self.id.clone()
    }

    pub fn get_dataset(&mut self) -> String
    {
        self.dataset.clone()
    }

    pub fn is_recursive(&mut self) -> bool
    {
        self.recursive.clone()
    }

    pub fn lifetime(&mut self) -> (i64, String)
    {
        (self.lifetime_value.clone(), self.lifetime_unit.clone())
    }

    pub fn get_schedule(&mut self) -> String
    {
        format!("{}{}{}{}{}", self.schedule.minute.clone(), self.schedule.hour.clone(), self.schedule.dom.clone(), self.schedule.month.clone(), self.schedule.dow.clone())
    }
}


use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

pub type SnapshotTasks = Vec<Task>;

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