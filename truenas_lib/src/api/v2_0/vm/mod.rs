const DIR: &str = "/api/v2.0/vm";


use crate::server::Server;

pub fn get(server: &mut Server) -> Result<AllVMs, reqwest::Error>
{
    let url = format!("{}{}", server.url(), DIR);
    
    let res = crate::api_commands::get::<AllVMs>(&url, &server.key())?;

    
    

    Ok(res)
}




use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

pub type AllVMs = Vec<VM>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VM {
    id: i64,
    name: String,
    description: String,
    vcpus: i64,
    memory: i64,
    #[serde(rename = "min_memory")]
    min_memory: Value,
    autostart: bool,
    time: String,
    bootloader: String,
    cores: i64,
    threads: i64,
    #[serde(rename = "hyperv_enlightenments")]
    hyperv_enlightenments: bool,
    #[serde(rename = "shutdown_timeout")]
    shutdown_timeout: i64,
    #[serde(rename = "cpu_mode")]
    cpu_mode: String,
    #[serde(rename = "cpu_model")]
    cpu_model: Value,
    cpuset: String,
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
    arch_type: Value,
    #[serde(rename = "machine_type")]
    machine_type: Value,
    uuid: String,
    #[serde(rename = "command_line_args")]
    command_line_args: String,
    #[serde(rename = "bootloader_ovmf")]
    bootloader_ovmf: String,
    #[serde(rename = "trusted_platform_module")]
    trusted_platform_module: bool,
    #[serde(rename = "enable_cpu_topology_extension")]
    enable_cpu_topology_extension: bool,
    devices: Vec<Device>,
    #[serde(rename = "display_available")]
    display_available: bool,
    status: Status,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Device {
    id: i64,
    dtype: String,
    attributes: Attributes,
    order: i64,
    vm: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Attributes {
    path: Option<String>,
    #[serde(rename = "type")]
    type_field: Option<String>,
    #[serde(rename = "logical_sectorsize")]
    logical_sectorsize: Value,
    #[serde(rename = "physical_sectorsize")]
    physical_sectorsize: Value,
    iotype: Option<String>,
    port: Option<i64>,
    bind: Option<String>,
    password: Option<String>,
    web: Option<bool>,
    resolution: Option<String>,
    #[serde(rename = "web_port")]
    web_port: Option<i64>,
    wait: Option<bool>,
    pptdev: Option<String>,
    mac: Option<String>,
    #[serde(rename = "nic_attach")]
    nic_attach: Option<String>,
    #[serde(rename = "trust_guest_rx_filters")]
    trust_guest_rx_filters: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Status {
    state: String,
    pid: i64,
    #[serde(rename = "domain_state")]
    domain_state: String,
}
