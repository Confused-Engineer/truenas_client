const TN_URL: &str = "data_0";
const TN_KEY: &str = "data_1";
const PROM_URL: &str = "data_2";
pub struct ConnectionData
{
    truenas_url: String,
    truenas_key: String,
    prometheus_url: String,
}


impl ConnectionData
{
    pub fn load() -> Self
    {
        let mut tn_key = crate::encrypt::Encrypt::new(TN_KEY);
        let mut tn_url = crate::encrypt::Encrypt::new(TN_URL);
        let mut prom_url = crate::encrypt::Encrypt::new(PROM_URL);

        let mut truenas_url = String::new();
        let mut truenas_key = String::new();
        let mut prometheus_url = String::new();

        if tn_key.load_file().is_ok() && tn_url.load_file().is_ok()
        {
            truenas_url = tn_url.get_key();
            truenas_key = tn_key.get_key();
        }

        if prom_url.load_file().is_ok()
        {
            prometheus_url = prom_url.get_key();
        }

        
        Self { truenas_url: truenas_url, truenas_key: truenas_key, prometheus_url: prometheus_url }
    }

    pub fn save(&mut self) -> &mut Self
    {
        let mut tn_key = crate::encrypt::Encrypt::new(TN_KEY);
        let mut tn_url = crate::encrypt::Encrypt::new(TN_URL);
        let mut prom_url = crate::encrypt::Encrypt::new(PROM_URL);

        let _ = tn_key.set_key(&self.truenas_key).save_file();
        let _ = tn_url.set_key(&self.truenas_url).save_file();
        let _ = prom_url.set_key(&self.prometheus_url).save_file();

        self
    }


    pub fn set_truenas_url(&mut self, url: &str) -> &mut Self
    {
        self.truenas_url = String::from(url);
        self
    }

    pub fn set_truenas_key(&mut self, key: &str) -> &mut Self
    {
        self.truenas_key = String::from(key);
        self
    }

    pub fn set_prometheus_url(&mut self, url: &str) -> &mut Self
    {
        self.prometheus_url = String::from(url);
        self
    }

    pub fn get_truenas_url(&mut self) -> String
    {
        self.truenas_url.clone()
    }

    pub fn get_truenas_key(&mut self) -> String
    {
        self.truenas_key.clone()
    }

    pub fn get_prometheus_url(&mut self) -> String
    {
        self.prometheus_url.clone()
    }
}