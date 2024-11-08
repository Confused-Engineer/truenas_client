const DIR: &str = "/api/v2.0/app/start";
use crate::server::Server;
use serde_json::Value;

pub fn post(application: String ,server: &mut Server) -> Result<Value, reqwest::Error>
{
    let url = format!("{}{}", server.url(), DIR);
    let body = format!("\"{}\"", application);
    let res = crate::api_commands::post::<Value>(&url, &server.key(), body)?;
    
    Ok(res)
}


