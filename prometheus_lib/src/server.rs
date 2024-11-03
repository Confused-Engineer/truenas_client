#[derive(Clone)]
pub struct Prometheus
{
    url: String,
}


impl Prometheus
{
    pub fn new(url: &str) -> Self
    {
        let url = String::from(url);
        Self {
            url: url,

        }
    }

    pub fn test(&mut self) -> reqwest::Result<()>
    {
        crate::api::v1::query::cpu::usage::get(self)?;
        Ok(())
    }

    pub(crate) fn url(&mut self) -> String
    {
        self.url.clone()
    }


}