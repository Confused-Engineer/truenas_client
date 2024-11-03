const APP: &str = "/api/v2.0/app/upgrade";

use crate::server::Server;


pub fn post(server: &mut Server, app: &str) -> Result<Value, reqwest::Error>
{
    let url = format!("{}{}", server.url(), APP);
    let body = format!("{{ \"app_name\":\"{}\", \"options\": {{}} }}", app);
    let res = crate::api_commands::post::<Value>(&url, &server.key(), body.clone())?;
    
    Ok(res)
    
}

use serde_json::Value;