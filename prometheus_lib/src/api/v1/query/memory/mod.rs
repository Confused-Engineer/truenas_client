const TOTAL: &str = "/api/v1/query?query=node_memory_MemTotal_bytes";
const FREE: &str = "/api/v1/query?query=node_memory_MemFree_bytes";


use crate::server::Prometheus;


pub struct Memory
{
    total: u64,
    used: u64,
    free: u64,
    in_gb: bool,
    prom_svr: Prometheus
}

impl Memory
{
    pub fn load(prometheus: &mut Prometheus) -> Self
    {
        let byte_str = get(prometheus).unwrap_or((0,0));
        if byte_str.0 == 0
        {
            let fail = Self {
                total: 0,
                used: 0,
                free: 0,
                in_gb: false,
                prom_svr: prometheus.clone(),
            };
            
            
            return fail;
        }

        Self {
            total: byte_str.0,
            used: (byte_str.0 - byte_str.1),
            free: byte_str.1,
            in_gb: false,
            prom_svr: prometheus.clone(),
        }
    }

    pub fn refresh(&mut self) -> &mut Self
    {
        let byte_str = get(&mut self.prom_svr).unwrap_or((0,0));
        if byte_str.0 == 0
        {
            return self;
        }

        self.total = byte_str.0;
        self.used = byte_str.0 - byte_str.1;
        self.free = byte_str.1;

        self

    }

    pub fn get_total(&mut self) -> u64
    {
        if self.in_gb
        {
            self.in_gb = false;
            return self.total.clone() / 1000000000;
        }
        self.total.clone()
    }

    pub fn get_free(&mut self) -> u64
    {
        if self.in_gb
        {
            self.in_gb = false;
            return self.free.clone() / 1000000000;
        }
        self.free.clone()
    }

    pub fn get_used(&mut self) -> u64
    {
        if self.in_gb
        {
            self.in_gb = false;
            return self.used.clone() / 1000000000;
        }
        self.used.clone()
    }

    pub fn in_gb(&mut self) -> &mut Self
    {
        self.in_gb = true;
        self
    }
    
}



fn get(prometheus: &mut Prometheus) -> reqwest::Result<(u64,u64)>
{
    let total = format!("{}{}", prometheus.url(), TOTAL);
    let free = format!("{}{}", prometheus.url(), FREE);

    let res1 = crate::api_commands::get::<MemoryAPI>(&total)?;
    let res2 = crate::api_commands::get::<MemoryAPI>(&free)?;

    let total_bytes = res1.data.result[0].value.1.clone().parse::<u64>().unwrap_or(0);
    let free_bytes = res2.data.result[0].value.1.clone().parse::<u64>().unwrap_or(0);

    Ok((total_bytes,free_bytes))
}



use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MemoryAPI {
    status: String,
    data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    result_type: String,
    result: Vec<Result>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Result {
    metric: Metric,
    value: (f64, String),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Metric {
    #[serde(rename = "__name__")]
    name: String,
    instance: String,
    job: String,
}
