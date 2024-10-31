const APP: &str = "/app";
const CORE_DEBUG_MODE_ENABLED: &str = "/core/debug_mode_enabled";
use serde::{Deserialize};

//#[derive(Deserialize, Debug)]


struct Api
{
    key: String,
    url: String,
}

impl Api
{
    fn new(url: &str, key: &str)
    {
        
    }
}


#[tokio::main]
async fn get<T: serde::de::DeserializeOwned>()-> reqwest::Result<T>
{
    let client = reqwest::Client::new();
    let resp = client.get("")
        .bearer_auth("")
        .send()
        .await?.json::<T>()
        .await?;

    return Ok(resp);

    
}


#[cfg(test)]
mod tests {

    

    use super::*;

    #[test]
    fn app_get() {
        println!("{:#?}", get::<crate::serde::app::Root>());


    }
}