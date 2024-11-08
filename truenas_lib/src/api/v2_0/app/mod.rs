pub mod upgrade_summary;
pub mod upgrade;
pub mod start;
pub mod stop;

const DIR: &str = "/api/v2.0/app";


use crate::server::Server;

/// Retrieves the list of installed apps from the server
/// ```
/// let mut server: Server = Server::new("ip", "key");
/// let result = truenas_lib::get::<AppList>(&url, &server.key());
/// println!("{:#?}", result);
/// ```
pub fn get(server: &mut Server) -> Result<AppList, reqwest::Error>
{
    let url = format!("{}{}", server.url(), DIR);
    let res = crate::api_commands::get::<AppList>(&url, &server.key())?;
    Ok(res)
}



use serde::Deserialize;
use serde::Serialize;


pub type AppList = Vec<Application>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    name: String,
    id: String,
    #[serde(rename = "active_workloads")]
    active_workloads: ActiveWorkloads,
    state: String,
    #[serde(rename = "upgrade_available")]
    upgrade_available: bool,
    #[serde(rename = "image_updates_available")]
    image_updates_available: bool,
    #[serde(rename = "custom_app")]
    custom_app: bool,
    #[serde(rename = "human_version")]
    human_version: String,
    metadata: Metadata,
    migrated: bool,
    notes: String,
    portals: Portals,
    version: String,
}

impl Application
{
    pub fn get_name(&mut self) -> String
    {
        self.name.clone()
    }

    pub fn get_id(&mut self) -> String
    {
        self.id.clone()
    }

    pub fn get_state(&mut self) -> String
    {
        self.state.clone()
    }

    pub fn get_version(&mut self) -> String
    {
        self.version.clone()
    }

    pub fn upgrade_available(&mut self) -> bool
    {
        self.upgrade_available.clone()
    }

    pub fn icon_url(&mut self) -> String
    {
        self.metadata.icon.clone()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ActiveWorkloads {
    containers: i64,
    #[serde(rename = "used_ports")]
    used_ports: Vec<UsedPort>,
    #[serde(rename = "container_details")]
    container_details: Vec<ContainerDetail>,
    volumes: Vec<Volume>,
    images: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UsedPort {
    #[serde(rename = "container_port")]
    container_port: i64,
    protocol: String,
    #[serde(rename = "host_ports")]
    host_ports: Vec<HostPort>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HostPort {
    #[serde(rename = "host_port")]
    host_port: i64,
    #[serde(rename = "host_ip")]
    host_ip: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContainerDetail {
    #[serde(rename = "service_name")]
    service_name: String,
    image: String,
    #[serde(rename = "port_config")]
    port_config: Vec<PortConfig>,
    state: String,
    #[serde(rename = "volume_mounts")]
    volume_mounts: Vec<VolumeMount>,
    id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PortConfig {
    #[serde(rename = "container_port")]
    container_port: i64,
    protocol: String,
    #[serde(rename = "host_ports")]
    host_ports: Vec<HostPort2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HostPort2 {
    #[serde(rename = "host_port")]
    host_port: i64,
    #[serde(rename = "host_ip")]
    host_ip: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VolumeMount {
    source: String,
    destination: String,
    mode: String,
    #[serde(rename = "type")]
    type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Volume {
    source: String,
    destination: String,
    mode: String,
    #[serde(rename = "type")]
    type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Metadata {
    #[serde(rename = "app_version")]
    app_version: String,
    capabilities: Vec<Capability>,
    categories: Vec<String>,
    description: String,
    home: String,
    #[serde(rename = "host_mounts")]
    host_mounts: Vec<HostMount>,
    icon: String,
    keywords: Vec<String>,
    #[serde(rename = "lib_version")]
    lib_version: String,
    #[serde(rename = "lib_version_hash")]
    lib_version_hash: String,
    maintainers: Vec<Maintainer>,
    name: String,
    #[serde(rename = "run_as_context")]
    run_as_context: Vec<RunAsContext>,
    screenshots: Vec<String>,
    sources: Vec<String>,
    title: String,
    train: String,
    version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Capability {
    description: String,
    name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HostMount {
    description: String,
    #[serde(rename = "host_path")]
    host_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Maintainer {
    email: String,
    name: String,
    url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RunAsContext {
    description: String,
    gid: i64,
    #[serde(rename = "group_name")]
    group_name: String,
    uid: i64,
    #[serde(rename = "user_name")]
    user_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Portals {
    #[serde(rename = "Web UI")]
    web_ui: String,
}
