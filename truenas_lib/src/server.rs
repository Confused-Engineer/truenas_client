pub struct Server
{
    url: String,
    key: String
}

impl Server
{
    pub fn new(url: &str, key: &str) -> Self
    {
        let url = String::from(url);
        let key = String::from(key);
        Self {
            url: url,
            key: key
        }
    }

    pub fn test(&mut self) -> reqwest::Result<()>
    {
        crate::api::v2_0::system::version_short::get(self)?;
        Ok(())
    }

    pub(crate) fn url(&mut self) -> String
    {
        self.url.clone()
    }

    pub(crate) fn key(&mut self) -> String
    {
        self.key.clone()
    }
}