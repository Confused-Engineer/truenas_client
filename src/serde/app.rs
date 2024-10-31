use serde::Deserialize;
use serde::Serialize;


pub type Root = Vec<Root2>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root2 {
    pub name: String,
    pub id: String,
    #[serde(rename = "active_workloads")]
    pub active_workloads: ActiveWorkloads,
    pub state: String,
    #[serde(rename = "upgrade_available")]
    pub upgrade_available: bool,
    #[serde(rename = "image_updates_available")]
    pub image_updates_available: bool,
    #[serde(rename = "custom_app")]
    pub custom_app: bool,
    #[serde(rename = "human_version")]
    pub human_version: String,
    pub metadata: Metadata,
    pub migrated: bool,
    pub notes: String,
    pub portals: Portals,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveWorkloads {
    pub containers: i64,
    #[serde(rename = "used_ports")]
    pub used_ports: Vec<UsedPort>,
    #[serde(rename = "container_details")]
    pub container_details: Vec<ContainerDetail>,
    pub volumes: Vec<Volume>,
    pub images: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsedPort {
    #[serde(rename = "container_port")]
    pub container_port: i64,
    pub protocol: String,
    #[serde(rename = "host_ports")]
    pub host_ports: Vec<HostPort>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostPort {
    #[serde(rename = "host_port")]
    pub host_port: i64,
    #[serde(rename = "host_ip")]
    pub host_ip: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerDetail {
    #[serde(rename = "service_name")]
    pub service_name: String,
    pub image: String,
    #[serde(rename = "port_config")]
    pub port_config: Vec<PortConfig>,
    pub state: String,
    #[serde(rename = "volume_mounts")]
    pub volume_mounts: Vec<VolumeMount>,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortConfig {
    #[serde(rename = "container_port")]
    pub container_port: i64,
    pub protocol: String,
    #[serde(rename = "host_ports")]
    pub host_ports: Vec<HostPort2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostPort2 {
    #[serde(rename = "host_port")]
    pub host_port: i64,
    #[serde(rename = "host_ip")]
    pub host_ip: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeMount {
    pub source: String,
    pub destination: String,
    pub mode: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    pub source: String,
    pub destination: String,
    pub mode: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    #[serde(rename = "app_version")]
    pub app_version: String,
    pub capabilities: Vec<Capability>,
    pub categories: Vec<String>,
    pub description: String,
    pub home: String,
    #[serde(rename = "host_mounts")]
    pub host_mounts: Vec<HostMount>,
    pub icon: String,
    pub keywords: Vec<String>,
    #[serde(rename = "lib_version")]
    pub lib_version: String,
    #[serde(rename = "lib_version_hash")]
    pub lib_version_hash: String,
    pub maintainers: Vec<Maintainer>,
    pub name: String,
    #[serde(rename = "run_as_context")]
    pub run_as_context: Vec<RunAsContext>,
    pub screenshots: Vec<String>,
    pub sources: Vec<String>,
    pub title: String,
    pub train: String,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Capability {
    pub description: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostMount {
    pub description: String,
    #[serde(rename = "host_path")]
    pub host_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Maintainer {
    pub email: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunAsContext {
    pub description: String,
    pub gid: i64,
    #[serde(rename = "group_name")]
    pub group_name: String,
    pub uid: i64,
    #[serde(rename = "user_name")]
    pub user_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Portals {
    #[serde(rename = "Web UI")]
    pub web_ui: String,
}
