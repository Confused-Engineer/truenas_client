 

#[tokio::main]
pub(crate) async fn get<T: serde::de::DeserializeOwned>(url: &str)-> reqwest::Result<T>
{
    let client = reqwest::Client::new();
    let resp = client.get(url)
        .send()
        .await?.json::<T>()
        .await?;

    return Ok(resp);
}

#[tokio::main]
pub(crate) async fn post<T: serde::de::DeserializeOwned>(url: &str, body: String)-> reqwest::Result<T>
{
    let client = reqwest::Client::new();
    let resp = client.post(url)
        .body(body)
        .send()
        .await?.json::<T>()
        .await?;

    return Ok(resp);
}



#[tokio::main]
async fn _put<T: serde::de::DeserializeOwned>(url: &str)-> reqwest::Result<T>
{
    let client = reqwest::Client::new();
    let resp = client.put(url)
        .send()
        .await?.json::<T>()
        .await?;

    return Ok(resp);
}

#[tokio::main]
async fn _delete<T: serde::de::DeserializeOwned>(url: &str)-> reqwest::Result<T>
{
    let client = reqwest::Client::new();
    let resp = client.delete(url)
        .send()
        .await?.json::<T>()
        .await?;

    return Ok(resp);
}



