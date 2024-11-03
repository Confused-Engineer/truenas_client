fn main() {
    println!("Hello, world!");
}








#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truenas() {
        let mut server = truenas_lib::server::Server::new("https://tn.a5f.org", include_str!("../assets/api_key"));
        //let apps = truenas_lib::api::v2_0::app::upgrade::post(&mut server, "actual-budget");
        let apps = truenas_lib::api::v2_0::app::get(&mut server);
        println!("{:#?}", apps) 
    }

    #[test]
    fn prometheus() {
        let mut prometheus = prometheus_lib::server::Prometheus::new("https://prometheus.a5f.org");
        //let apps = truenas_lib::api::v2_0::app::upgrade::post(&mut server, "actual-budget");
        let mut apps = prometheus_lib::api::v1::query::memory::Memory::load(&mut prometheus);
        println!("{} | {} | {}", apps.get_used(), apps.in_gb().get_used(), apps.in_gb().get_total() ) 
    }

    #[test]
    fn string() {
        let stl = String::from("hello");
        println!("{}", &stl[0..2])
    }




}