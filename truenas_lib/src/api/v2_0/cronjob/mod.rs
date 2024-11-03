const DIR: &str = "/api/v2.0/cronjob";


use crate::server::Server;

/// Retrieves the list of installed apps from the server
/// ```
/// let mut server: Server = Server::new("ip", "key");
/// let result = truenas_lib::get::<AllDisks>(&url, &server.key());
/// println!("{:#?}", result);
/// ```

pub fn get(server: &mut Server) -> Result<AllCronjobs, reqwest::Error>
{
    let url = format!("{}{}", server.url(), DIR);
    let res = crate::api_commands::get::<AllCronjobs>(&url, &server.key())?;
    Ok(res)
}


use serde::Deserialize;
use serde::Serialize;

type AllCronjobs = Vec<Cronjob>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cronjob {
    id: i64,
    user: String,
    command: String,
    description: String,
    enabled: bool,
    stdout: bool,
    stderr: bool,
    schedule: Schedule,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Schedule {
    minute: String,
    hour: String,
    dom: String,
    month: String,
    dow: String,
}