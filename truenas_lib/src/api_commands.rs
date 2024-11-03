 

#[tokio::main]
pub(crate) async fn get<T: serde::de::DeserializeOwned>(url: &str, key: &str)-> reqwest::Result<T>
{
    let client = reqwest::Client::new();
    let resp = client.get(url)
        .bearer_auth(key)
        .send()
        .await?.json::<T>()
        .await?;

    return Ok(resp);
}

#[tokio::main]
pub(crate) async fn post<T: serde::de::DeserializeOwned>(url: &str, key: &str, body: String)-> reqwest::Result<T>
{
    let client = reqwest::Client::new();
    let resp = client.post(url)
        .bearer_auth(key)
        .body(body)
        .send()
        .await?.json::<T>()
        .await?;

    return Ok(resp);
}



#[tokio::main]
async fn _put<T: serde::de::DeserializeOwned>(url: &str, key: &str)-> reqwest::Result<T>
{
    let client = reqwest::Client::new();
    let resp = client.put(url)
        .bearer_auth(key)
        .send()
        .await?.json::<T>()
        .await?;

    return Ok(resp);
}

#[tokio::main]
async fn _delete<T: serde::de::DeserializeOwned>(url: &str, key: &str)-> reqwest::Result<T>
{
    let client = reqwest::Client::new();
    let resp = client.delete(url)
        .bearer_auth(key)
        .send()
        .await?.json::<T>()
        .await?;

    return Ok(resp);
}



