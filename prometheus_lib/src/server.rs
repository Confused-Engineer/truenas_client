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

    pub(crate) fn url(&mut self) -> String
    {
        self.url.clone()
    }


}