const DIR: &str = "/api/v2.0/system/version_short";


use crate::server::Server;

pub fn get(server: &mut Server) -> Result<String, reqwest::Error>
{
    let url = format!("{}{}", server.url(), DIR);
    let res = crate::api_commands::get::<String>(&url, &server.key())?;
    Ok(res)
}