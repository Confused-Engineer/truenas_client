const DIR: &str = "/api/v1/query?query=(sum(irate(node_cpu_seconds_total {mode!=\"idle\"} [1m])) by (instance)) / sum(irate(node_cpu_seconds_total[1m])) by (instance)*100";


use crate::server::Prometheus;


pub fn get(prometheus: &mut Prometheus) -> reqwest::Result<String>
{
    let url = format!("{}{}", prometheus.url(), DIR);
    let res = crate::api_commands::get::<Usage>(&url)?;

    let res_string = &res.data.result[0].value.1.clone()[0..3];
    Ok(res_string.to_string())
}





use serde::Deserialize;
use serde::Serialize;



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
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
    instance: String,
}
