const DIR: &str = "/api/v2.0/vm";


use crate::server::Server;

pub fn get(server: &mut Server) -> Result<AllVMs, reqwest::Error>
{
    let url = format!("{}{}", server.url(), DIR);
    
    let res = crate::api_commands::get::<AllVMs>(&url, &server.key())?;

    
    

    Ok(res)
}


impl VM
{
    pub fn get_name(&mut self) -> String
    {
        self.name.clone()
    }

    pub fn get_cpu(&mut self) -> (i64, i64)
    {
        (self.cores.clone(), self.threads.clone())
    }

    pub fn get_memory(&mut self) -> i64
    {
        self.memory.clone()
    }

    pub fn get_status(&mut self) -> String
    {
        self.status.state.clone()
    }
}

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

pub type AllVMs = Vec<VM>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VM {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "description")]
    description: String,

    #[serde(rename = "vcpus")]
    vcpus: i64,

    #[serde(rename = "memory")]
    memory: i64,

    #[serde(rename = "min_memory")]
    min_memory: Option<serde_json::Value>,

    #[serde(rename = "autostart")]
    autostart: bool,

    #[serde(rename = "time")]
    time: String,

    #[serde(rename = "bootloader")]
    bootloader: String,

    #[serde(rename = "cores")]
    cores: i64,

    #[serde(rename = "threads")]
    threads: i64,

    #[serde(rename = "hyperv_enlightenments")]
    hyperv_enlightenments: bool,

    #[serde(rename = "shutdown_timeout")]
    shutdown_timeout: i64,

    #[serde(rename = "cpu_mode")]
    cpu_mode: String,

    #[serde(rename = "cpu_model")]
    cpu_model: Option<serde_json::Value>,

    #[serde(rename = "cpuset")]
    cpuset: String,

    #[serde(rename = "nodeset")]
    nodeset: String,

    #[serde(rename = "pin_vcpus")]
    pin_vcpus: bool,

    #[serde(rename = "hide_from_msr")]
    hide_from_msr: bool,

    #[serde(rename = "suspend_on_snapshot")]
    suspend_on_snapshot: bool,

    #[serde(rename = "ensure_display_device")]
    ensure_display_device: bool,

    #[serde(rename = "arch_type")]
    arch_type: Option<serde_json::Value>,

    #[serde(rename = "machine_type")]
    machine_type: Option<serde_json::Value>,

    #[serde(rename = "uuid")]
    uuid: String,

    #[serde(rename = "command_line_args")]
    command_line_args: String,

    #[serde(rename = "bootloader_ovmf")]
    bootloader_ovmf: String,

    #[serde(rename = "trusted_platform_module")]
    trusted_platform_module: bool,

    #[serde(rename = "enable_cpu_topology_extension")]
    enable_cpu_topology_extension: bool,

    #[serde(rename = "devices")]
    devices: Vec<Device>,

    #[serde(rename = "display_available")]
    display_available: bool,

    #[serde(rename = "status")]
    status: Status,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Device {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "dtype")]
    dtype: String,

    #[serde(rename = "attributes")]
    attributes: Attributes,

    #[serde(rename = "order")]
    order: i64,

    #[serde(rename = "vm")]
    vm: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Attributes {
    #[serde(rename = "path")]
    path: Option<String>,

    #[serde(rename = "type")]
    attributes_type: Option<String>,

    #[serde(rename = "logical_sectorsize")]
    logical_sectorsize: Option<serde_json::Value>,

    #[serde(rename = "physical_sectorsize")]
    physical_sectorsize: Option<serde_json::Value>,

    #[serde(rename = "iotype")]
    iotype: Option<String>,

    #[serde(rename = "port")]
    port: Option<i64>,

    #[serde(rename = "bind")]
    bind: Option<String>,

    #[serde(rename = "password")]
    password: Option<String>,

    #[serde(rename = "web")]
    web: Option<bool>,

    #[serde(rename = "resolution")]
    resolution: Option<String>,

    #[serde(rename = "web_port")]
    web_port: Option<i64>,

    #[serde(rename = "wait")]
    wait: Option<bool>,

    #[serde(rename = "pptdev")]
    pptdev: Option<String>,

    #[serde(rename = "mac")]
    mac: Option<String>,

    #[serde(rename = "nic_attach")]
    nic_attach: Option<String>,

    #[serde(rename = "trust_guest_rx_filters")]
    trust_guest_rx_filters: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Status {
    #[serde(rename = "state")]
    state: String,

    #[serde(rename = "pid")]
    pid: i64,

    #[serde(rename = "domain_state")]
    domain_state: String,
}