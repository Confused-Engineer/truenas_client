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
    fn new(url: &str, key: &str) -> Self
    {
        Self 
        {
            key: key.to_string(), 
            url: url.to_string()
        }
    }

    fn app_get(&mut self)
    {
        let api_url = format!("{}{}", self.url, APP);
        let result = get::<crate::api::v2_0::app::Root>(&api_url, &self.key);

        if result.is_err()
        {
            return;
        } else {
            println!("{:#?}", result.unwrap());
        }

    }
}


#[tokio::main]
async fn get<T: serde::de::DeserializeOwned>(url: &str, key: &str)-> reqwest::Result<T>
{
    let client = reqwest::Client::new();
    let resp = client.get(url)
        .bearer_auth(key)
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

        let mut api = Api::new("https://tn.a5f.org/api/v2.0", include_str!("../assets/api_key"));
        api.app_get();


    }
}