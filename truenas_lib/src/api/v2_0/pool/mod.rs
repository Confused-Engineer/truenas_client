pub mod snapshottask;

const DIR: &str = "/api/v2.0/pool";

use crate::server::Server;

pub fn get(server: &mut Server) -> Result<AllPools, reqwest::Error>
{
    let url = format!("{}{}", server.url(), DIR);
    let res = crate::api_commands::get::<AllPools>(&url, &server.key())?;
    Ok(res)
}


impl Pool
{
    pub fn get_name(&mut self) -> String
    {
        self.name.clone()
    }

    pub fn get_path(&mut self) -> String
    {
        self.path.clone()
    }

    pub fn is_healthy(&mut self) -> bool
    {
        self.healthy.clone()
    }

    pub fn scan_err(&mut self) -> i64
    {
        self.scan.errors.clone()
    }

    pub fn get_capacity(&mut self) -> i64
    {
        self.size / 1000000000
    }

    pub fn get_free(&mut self) -> i64
    {
        self.free / 1000000000
    }

    pub fn get_used(&mut self) -> i64
    {
        (self.size - self.free) / 1000000000
    }

    pub fn get_topology(&mut self) -> Topology
    {
        self.topology.clone()
    }
}

impl Topology
{
    pub fn get_data_vdev(&mut self) -> Vec<Daum>
    {
        self.data.clone()
    }

    pub fn get_spare_vdev(&mut self) -> Vec<Spare>
    {
        self.spare.clone()
    }
}

impl VdevOptions for Daum
{
    fn get_name(&mut self) -> String {
        self.name.clone()
    }

    fn get_type(&mut self) -> String {
        self.type_field.clone()
    }

    fn get_r_w_checksume_errors(&mut self) -> (i64, i64, i64) {
        (self.stats.read_errors.clone(), self.stats.write_errors.clone(), self.stats.checksum_errors.clone())
    }
}

impl VdevOptions for Spare
{
    fn get_name(&mut self) -> String {
        self.name.clone()
    }

    fn get_type(&mut self) -> String {
        self.type_field.clone()
    }

    fn get_r_w_checksume_errors(&mut self) -> (i64, i64, i64) {
        (self.stats.read_errors.clone(), self.stats.write_errors.clone(), self.stats.checksum_errors.clone())
    }
}


pub trait VdevOptions {
    fn get_name(&mut self) -> String;
    fn get_type(&mut self) -> String;
    fn get_r_w_checksume_errors(&mut self) -> (i64, i64, i64);
}

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

pub type AllPools = Vec<Pool>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pool {
    id: i64,
    name: String,
    guid: String,
    path: String,
    status: String,
    scan: Scan,
    expand: Value,
    topology: Topology,
    healthy: bool,
    warning: bool,
    #[serde(rename = "status_code")]
    status_code: String,
    #[serde(rename = "status_detail")]
    status_detail: Value,
    size: i64,
    allocated: i64,
    free: i64,
    freeing: i64,
    fragmentation: String,
    #[serde(rename = "size_str")]
    size_str: String,
    #[serde(rename = "allocated_str")]
    allocated_str: String,
    #[serde(rename = "free_str")]
    free_str: String,
    #[serde(rename = "freeing_str")]
    freeing_str: String,
    autotrim: Autotrim,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Scan {
    function: String,
    state: String,
    #[serde(rename = "start_time")]
    start_time: StartTime,
    #[serde(rename = "end_time")]
    end_time: EndTime,
    percentage: f64,
    #[serde(rename = "bytes_to_process")]
    bytes_to_process: i64,
    #[serde(rename = "bytes_processed")]
    bytes_processed: i64,
    #[serde(rename = "bytes_issued")]
    bytes_issued: i64,
    pause: Value,
    errors: i64,
    #[serde(rename = "total_secs_left")]
    total_secs_left: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StartTime {
    #[serde(rename = "$date")]
    date: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EndTime {
    #[serde(rename = "$date")]
    date: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Topology {
    data: Vec<Daum>,
    log: Vec<Value>,
    cache: Vec<Value>,
    spare: Vec<Spare>,
    special: Vec<Value>,
    dedup: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    name: String,
    #[serde(rename = "type")]
    type_field: String,
    path: Value,
    guid: String,
    status: String,
    stats: Stats,
    children: Vec<Children>,
    #[serde(rename = "unavail_disk")]
    unavail_disk: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Stats {
    timestamp: i64,
    #[serde(rename = "read_errors")]
    read_errors: i64,
    #[serde(rename = "write_errors")]
    write_errors: i64,
    #[serde(rename = "checksum_errors")]
    checksum_errors: i64,
    ops: Vec<i64>,
    bytes: Vec<i64>,
    size: i64,
    allocated: i64,
    fragmentation: i64,
    #[serde(rename = "self_healed")]
    self_healed: i64,
    #[serde(rename = "configured_ashift")]
    configured_ashift: i64,
    #[serde(rename = "logical_ashift")]
    logical_ashift: i64,
    #[serde(rename = "physical_ashift")]
    physical_ashift: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Children {
    name: String,
    #[serde(rename = "type")]
    type_field: String,
    path: String,
    guid: String,
    status: String,
    stats: Stats2,
    children: Vec<Value>,
    device: String,
    disk: String,
    #[serde(rename = "unavail_disk")]
    unavail_disk: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Stats2 {
    timestamp: i64,
    #[serde(rename = "read_errors")]
    read_errors: i64,
    #[serde(rename = "write_errors")]
    write_errors: i64,
    #[serde(rename = "checksum_errors")]
    checksum_errors: i64,
    ops: Vec<i64>,
    bytes: Vec<i64>,
    size: i64,
    allocated: i64,
    fragmentation: i64,
    #[serde(rename = "self_healed")]
    self_healed: i64,
    #[serde(rename = "configured_ashift")]
    configured_ashift: i64,
    #[serde(rename = "logical_ashift")]
    logical_ashift: i64,
    #[serde(rename = "physical_ashift")]
    physical_ashift: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spare {
    name: String,
    #[serde(rename = "type")]
    type_field: String,
    path: String,
    guid: String,
    status: String,
    stats: Stats3,
    children: Vec<String>,
    device: String,
    disk: String,
    #[serde(rename = "unavail_disk")]
    unavail_disk: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Stats3 {
    timestamp: i64,
    #[serde(rename = "read_errors")]
    read_errors: i64,
    #[serde(rename = "write_errors")]
    write_errors: i64,
    #[serde(rename = "checksum_errors")]
    checksum_errors: i64,
    ops: Vec<i64>,
    bytes: Vec<i64>,
    size: i64,
    allocated: i64,
    fragmentation: i64,
    #[serde(rename = "self_healed")]
    self_healed: i64,
    #[serde(rename = "configured_ashift")]
    configured_ashift: i64,
    #[serde(rename = "logical_ashift")]
    logical_ashift: i64,
    #[serde(rename = "physical_ashift")]
    physical_ashift: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Autotrim {
    value: String,
    rawvalue: String,
    parsed: String,
    source: String,
}

