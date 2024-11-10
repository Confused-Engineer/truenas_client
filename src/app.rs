#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

pub struct Dashboard
{
    pub connections: crate::server_info::ConnectionData,
    page: AppState,

    pub truenas: truenas_lib::server::Server,
    pub prometheus: prometheus_lib::server::Prometheus,

    pub truenas_url_temp: String,
    pub truenas_key_temp: String,
    pub prometheus_url_temp: String,

    pub truenas_is_ok: bool,
    pub prometheus_is_ok: bool,
    pub truenas_multithread_started: bool,
    pub prometheus_multithread_started: bool,

    pub cpu_usage: f64,
    pub multithread_cpu_usage: (std::sync::mpsc::Sender<f64>, std::sync::mpsc::Receiver<f64>),

    pub memory_usage: prometheus_lib::api::v1::query::memory::Memory,
    pub multithread_memory_usage: (std::sync::mpsc::Sender<prometheus_lib::api::v1::query::memory::Memory>, std::sync::mpsc::Receiver<prometheus_lib::api::v1::query::memory::Memory>),

    pub applist: truenas_lib::api::v2_0::app::AppList,
    pub multithread_applist: (std::sync::mpsc::Sender<truenas_lib::api::v2_0::app::AppList>, std::sync::mpsc::Receiver<truenas_lib::api::v2_0::app::AppList>),

    pub virtualmachinelist: truenas_lib::api::v2_0::vm::AllVMs,
    pub multithread_virtualmachinelist: (std::sync::mpsc::Sender<truenas_lib::api::v2_0::vm::AllVMs>, std::sync::mpsc::Receiver<truenas_lib::api::v2_0::vm::AllVMs>),

    pub interface_details: truenas_lib::api::v2_0::interface::AllInterfaces,
    pub multithread_interface_details:(std::sync::mpsc::Sender<truenas_lib::api::v2_0::interface::AllInterfaces>, std::sync::mpsc::Receiver<truenas_lib::api::v2_0::interface::AllInterfaces>),

    pub disk_details: truenas_lib::api::v2_0::disk::AllDisks,
    pub multithread_disk_details:(std::sync::mpsc::Sender<truenas_lib::api::v2_0::disk::AllDisks>, std::sync::mpsc::Receiver<truenas_lib::api::v2_0::disk::AllDisks>),

    pub pool_details: truenas_lib::api::v2_0::pool::AllPools,
    pub multithread_pool_details:(std::sync::mpsc::Sender<truenas_lib::api::v2_0::pool::AllPools>, std::sync::mpsc::Receiver<truenas_lib::api::v2_0::pool::AllPools>),

    pub snapshot_details: truenas_lib::api::v2_0::pool::snapshottask::SnapshotTasks,
    pub multithread_snapshot_details:(std::sync::mpsc::Sender<truenas_lib::api::v2_0::pool::snapshottask::SnapshotTasks>, std::sync::mpsc::Receiver<truenas_lib::api::v2_0::pool::snapshottask::SnapshotTasks>),

    pub service_details: truenas_lib::api::v2_0::service::AllServices,
    pub multithread_service_details:(std::sync::mpsc::Sender<truenas_lib::api::v2_0::service::AllServices>, std::sync::mpsc::Receiver<truenas_lib::api::v2_0::service::AllServices>),

}

impl Default for Dashboard
{
    fn default() -> Self {
        Self 
        {
            connections: crate::server_info::ConnectionData::load(),
            page: Self::load_data().2,
            truenas: Self::load_data().0,
            prometheus: Self::load_data().1,

            truenas_url_temp: String::new(),
            truenas_key_temp: String::new(),
            prometheus_url_temp: String::new(),

            truenas_is_ok: Self::load_data().3,
            prometheus_is_ok: Self::load_data().4,

            truenas_multithread_started: false,
            prometheus_multithread_started: false,

            cpu_usage: 0.0,
            multithread_cpu_usage: std::sync::mpsc::channel(),

            memory_usage: prometheus_lib::api::v1::query::memory::Memory::new(),
            multithread_memory_usage: std::sync::mpsc::channel(),

            applist: Vec::new(),
            multithread_applist: std::sync::mpsc::channel(),

            virtualmachinelist: Vec::new(),
            multithread_virtualmachinelist: std::sync::mpsc::channel(),

            interface_details: Vec::new(),
            multithread_interface_details: std::sync::mpsc::channel(),

            disk_details: Vec::new(),
            multithread_disk_details: std::sync::mpsc::channel(),

            pool_details: Vec::new(),
            multithread_pool_details: std::sync::mpsc::channel(),

            snapshot_details: Vec::new(),
            multithread_snapshot_details: std::sync::mpsc::channel(),
            
            service_details: Vec::new(),
            multithread_service_details: std::sync::mpsc::channel(),
            
        }
    }
}

impl Dashboard {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self
    {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        
        Default::default()
    }

    
    fn load_data() -> (truenas_lib::server::Server, prometheus_lib::server::Prometheus, AppState, bool, bool)
    {
        let mut con = crate::server_info::ConnectionData::load();
        let truenas = (con.get_truenas_url(), con.get_truenas_key());
        let mut truenas_server = truenas_lib::server::Server::new(&truenas.0, &truenas.1);

        let prometheus = con.get_prometheus_url();
        let mut prometheus_server = prometheus_lib::server::Prometheus::new(&prometheus);

        if truenas_server.test().is_ok() & prometheus_server.test().is_ok()
        {
            return (truenas_server, prometheus_server, AppState::Main, true, true);
        }

        if truenas_server.test().is_ok() && prometheus_server.test().is_err()
        {
            return (truenas_server, prometheus_lib::server::Prometheus::new(""), AppState::Main, true, false);
        }
        

        if truenas_server.test().is_err() && prometheus_server.test().is_ok()
        {
            return (truenas_lib::server::Server::new("", ""), prometheus_server, AppState::Main, false, true);
        }

        return (truenas_lib::server::Server::new("", ""), prometheus_lib::server::Prometheus::new(""), AppState::Preferences, false, false)
    }

    
    


    
}

impl eframe::App for Dashboard {

    
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        
        ctx.request_repaint();

        if !self.truenas_is_ok
        {
            self.page = AppState::Preferences
        }
        
        if (self.truenas_is_ok && !self.truenas_multithread_started) || (self.prometheus_is_ok && !self.prometheus_multithread_started) {
            self.start_multi();
        } 

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui: &mut egui::Ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Settings", |ui| {
                    if ui.button("Preferences").clicked()
                    {
                        self.page = AppState::Preferences;
                        ui.close_menu();
                    }

                    ui.add_space(10.0);

                    if ui.button("Quit").clicked()
                    {
                        self.connections.save();
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                if self.page != AppState::Main
                {
                    if ui.button("Return").clicked()
                    {
                        self.page = AppState::Main;
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.page {
                AppState::Main => {
                    ui.heading("Dashboard");
                    ui.separator();
                    self.main(ui);
                },
                AppState::Preferences => {
                    ui.heading("Preferences");
                    ui.separator();
                    self.preferences(ui);
                },
            }
        });
    }
}


#[derive(PartialEq)]
enum AppState
{
    Main,
    Preferences,
}







