#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{egui, epaint::PathShape};
use truenas_lib::api::v2_0::pool::VdevOptions;
use egui_gauge::Gauge;
fn main() -> eframe::Result 
{

    let options = eframe::NativeOptions
    {
        viewport: egui::ViewportBuilder::default()
        //.with_fullscreen(true)
        .with_title("TrueNAS API Client")
        .with_icon(
            eframe::icon_data::from_png_bytes(&include_bytes!("../icon.png")[..]).unwrap()
        ),
        ..Default::default()
    };

    eframe::run_native(
        "TrueNAS API Client", 
        options, 
        Box::new(|cc|
            {
                egui_extras::install_image_loaders(&cc.egui_ctx);
                Ok(Box::new(Dashboard::new(cc)))
            }),
    )
}



pub struct Dashboard
{
    connections: ConnectionData,
    page: AppState,

    truenas: truenas_lib::server::Server,
    prometheus: prometheus_lib::server::Prometheus,

    truenas_url_temp: String,
    truenas_key_temp: String,
    prometheus_url_temp: String,

    truenas_is_ok: bool,
    prometheus_is_ok: bool,
    truenas_multithread_started: bool,
    prometheus_multithread_started: bool,

    cpu_usage: f64,
    multithread_cpu_usage: (std::sync::mpsc::Sender<f64>, std::sync::mpsc::Receiver<f64>),

    memory_usage: prometheus_lib::api::v1::query::memory::Memory,
    multithread_memory_usage: (std::sync::mpsc::Sender<prometheus_lib::api::v1::query::memory::Memory>, std::sync::mpsc::Receiver<prometheus_lib::api::v1::query::memory::Memory>),

    applist: truenas_lib::api::v2_0::app::AppList,
    multithread_applist: (std::sync::mpsc::Sender<truenas_lib::api::v2_0::app::AppList>, std::sync::mpsc::Receiver<truenas_lib::api::v2_0::app::AppList>),

    virtualmachinelist: truenas_lib::api::v2_0::vm::AllVMs,
    multithread_virtualmachinelist: (std::sync::mpsc::Sender<truenas_lib::api::v2_0::vm::AllVMs>, std::sync::mpsc::Receiver<truenas_lib::api::v2_0::vm::AllVMs>),

    interface_details: truenas_lib::api::v2_0::interface::AllInterfaces,
    multithread_interface_details:(std::sync::mpsc::Sender<truenas_lib::api::v2_0::interface::AllInterfaces>, std::sync::mpsc::Receiver<truenas_lib::api::v2_0::interface::AllInterfaces>),

    disk_details: truenas_lib::api::v2_0::disk::AllDisks,
    multithread_disk_details:(std::sync::mpsc::Sender<truenas_lib::api::v2_0::disk::AllDisks>, std::sync::mpsc::Receiver<truenas_lib::api::v2_0::disk::AllDisks>),

    pool_details: truenas_lib::api::v2_0::pool::AllPools,
    multithread_pool_details:(std::sync::mpsc::Sender<truenas_lib::api::v2_0::pool::AllPools>, std::sync::mpsc::Receiver<truenas_lib::api::v2_0::pool::AllPools>),

    snapshot_details: truenas_lib::api::v2_0::pool::snapshottask::SnapshotTasks,
    multithread_snapshot_details:(std::sync::mpsc::Sender<truenas_lib::api::v2_0::pool::snapshottask::SnapshotTasks>, std::sync::mpsc::Receiver<truenas_lib::api::v2_0::pool::snapshottask::SnapshotTasks>),

    service_details: truenas_lib::api::v2_0::service::AllServices,
    multithread_service_details:(std::sync::mpsc::Sender<truenas_lib::api::v2_0::service::AllServices>, std::sync::mpsc::Receiver<truenas_lib::api::v2_0::service::AllServices>),

}

impl Default for Dashboard
{
    fn default() -> Self {
        Self 
        {
            connections: ConnectionData::load(),
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
    fn new(cc: &eframe::CreationContext<'_>) -> Self
    {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        Default::default()
    }

    
    fn load_data() -> (truenas_lib::server::Server, prometheus_lib::server::Prometheus, AppState, bool, bool)
    {
        let mut con = ConnectionData::load();
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

    
    

    fn preferences(&mut self, ui: &mut egui::Ui)
    {
        ui.heading("TrueNAS API");
        ui.label("URL");
        ui.text_edit_singleline(&mut self.truenas_url_temp).on_hover_text("Example: https://truenas.domain.com");
        ui.label("API Key");
        ui.add(egui::widgets::TextEdit::singleline(&mut self.truenas_key_temp).password(true)).on_hover_text("Generate key at: User Settings > API Keys");
        ui.horizontal(|ui| {
            if ui.add_sized([50.0,20.0], egui::widgets::Button::new("Clear Fields")).clicked()
            {
                self.truenas_url_temp = String::new();
                self.truenas_key_temp = String::new();
            }
            if ui.add_sized([50.0,20.0], egui::widgets::Button::new("Test Connection")).clicked()
            {
                self.truenas = truenas_lib::server::Server::new(&self.truenas_url_temp, &self.truenas_key_temp);
                if self.truenas.test().is_ok()
                {
                    self.truenas_is_ok = true;
                } else {
                    self.truenas_is_ok = false;
                }
            }

            if self.truenas_is_ok
            {
                ui.label("Connection Verified");
            } else {
                ui.label("Unverified");
            }
        });

        ui.separator();

        ui.heading("Prometheus API");
        ui.label("URL");
        ui.text_edit_singleline(&mut self.prometheus_url_temp);
        ui.horizontal(|ui| {
            if ui.add_sized([50.0,20.0], egui::widgets::Button::new("Clear Fields")).clicked()
            {
                self.prometheus_url_temp = String::new();
            }
            if ui.add_sized([50.0,20.0], egui::widgets::Button::new("Test Connection")).clicked()
            {
                self.prometheus = prometheus_lib::server::Prometheus::new(&self.prometheus_url_temp);
                if self.prometheus.test().is_ok()
                {
                    self.prometheus_is_ok = true;
                } else {
                    self.prometheus_is_ok = false;
                }
            }

            if self.prometheus_is_ok
            {
                ui.label("Connection Verified");
            } else {
                ui.label("Unverified");
            }
        });

        ui.separator();

        ui.horizontal(|ui| {
            if ui.add_sized([75.0,20.0], egui::widgets::Button::new("Save All")).clicked()
            {
                self.connections.set_truenas_url(&self.truenas_url_temp);
                self.connections.set_truenas_key(&self.truenas_key_temp);
                self.connections.set_prometheus_url(&self.prometheus_url_temp);
                self.connections.save();
            }
            if ui.add_sized([75.0,20.0], egui::widgets::Button::new("Load Saved")).clicked()
            {
                self.truenas_url_temp = self.connections.get_truenas_url();
                self.truenas_key_temp = self.connections.get_truenas_key();
                self.prometheus_url_temp = self.connections.get_prometheus_url();
                self.connections.save();
            }
        });
        

    }

    fn main(&mut self, ui: &mut egui::Ui)
    {
        ui.columns(4, |ui|{

            egui::ScrollArea::vertical().id_salt("first_scroll_area").show(&mut ui[0], |ui| {

                // App List
                ui.vertical_centered(|ui| {ui.heading(egui::RichText::new("Applications").size(20.0))});

                let temp = self.multithread_applist.1.try_recv();
                if temp.is_ok()
                {
                    self.applist = temp.unwrap();
                }
                
                ui.separator();
                for mut app in self.applist.clone()
                {
                    
                    ui.horizontal(|ui|{
                        ui.add_sized([75.0,75.0], egui::Image::new(app.icon_url()));
                        
                        ui.vertical(|ui|
                        {
                            ui.heading(app.get_name());

                            ui.horizontal(|ui| {
                                ui.label(format!("State: {}", app.get_state()));
                                
                                match app.get_state().as_str() {
                                    "RUNNING" => {
                                        ui.menu_button("v", |ui| {
                                            if ui.button("Stop").clicked()
                                            {
                                                let _ = truenas_lib::api::v2_0::app::stop::post(app.get_name(), &mut self.truenas);
                                                ui.close_menu();
                                            }
                                        });
                                    },
                                    "STOPPED" => {
                                        ui.menu_button("v", |ui| {
                                            if ui.button("Start").clicked()
                                            {
                                                let _ = truenas_lib::api::v2_0::app::start::post(app.get_name(), &mut self.truenas);
                                                ui.close_menu();
                                            }

                                        });
                                    },
                                    _ => {}
                                }
                                
                                
                            });
                            
                            ui.label(format!("Version: {}", app.get_version()));
                            if app.upgrade_available()
                            {
                                if ui.button("Upgrade App").clicked()
                                {
                                    let _ = truenas_lib::api::v2_0::app::upgrade::post(&mut self.truenas, &app.get_name());
                                }
                            } else {
                                ui.label("Latest Version");
                            }
                            
                        });
                        
                    });
                    
                    ui.separator();
                }

                // VM List
                ui.vertical_centered(|ui| {ui.heading(egui::RichText::new("Virtual Machines").size(20.0))});

                let temp = self.multithread_virtualmachinelist.1.try_recv();
                if temp.is_ok()
                {
                    self.virtualmachinelist = temp.unwrap();
                }

                ui.separator();
                for mut vm in self.virtualmachinelist.clone()
                {
                    
                    ui.heading(vm.get_name());
                    ui.label(format!("Cores: {} | Threads per Core: {}", vm.get_cpu().0, vm.get_cpu().1));
                    ui.label(format!("Assigned RAM: {}", vm.get_memory()));
                    ui.label(format!("Status: {}", vm.get_status()));
                    ui.separator();
                }
            });

            egui::ScrollArea::vertical().id_salt("second_scroll_area").show(&mut ui[1], |ui| {
                
                //CPU Usage
                
                ui.vertical_centered(|ui| {ui.heading(egui::RichText::new("CPU and RAM Usage").size(20.0))});

                let temp = self.multithread_cpu_usage.1.try_recv();
                {
                    if temp.is_ok()
                    {
                        self.cpu_usage = temp.unwrap();
                    }
                }

                let temp = self.multithread_memory_usage.1.try_recv();
                {
                    if temp.is_ok()
                    {
                        self.memory_usage = temp.unwrap();
                    }
                }

                ui.separator();

                

                
                ui.columns(2, |ui| {
                    ui[0].add(Gauge::new(self.cpu_usage, 0.0..=100.0, 200.0, egui::Color32::from_rgb(0, 92, 128)).text("CPU")).on_hover_text("CPU Usage in GB");
                    ui[1].add(Gauge::new(self.memory_usage.in_gb().get_used(), 0..=self.memory_usage.in_gb().get_total(), 200.0, egui::Color32::from_rgb(0, 92, 128)).text("RAM")).on_hover_text("RAM Usage in GB");
                });
                ui.separator();


                

                // network inerfaces

                ui.vertical_centered(|ui| {ui.heading(egui::RichText::new("Network Interfaces").size(20.0))});
                ui.separator();
                let temp = self.multithread_interface_details.1.try_recv();
                {
                    if temp.is_ok()
                    {
                        self.interface_details = temp.unwrap();
                    }
                }

                for mut interface in self.interface_details.clone().into_iter()
                {
                    ui.heading(format!("Name: {}, ID: {}", interface.get_name(), interface.get_id()));
                    ui.label(format!("Virtual Interface: {}, DHCP Enabled: {}", interface.is_fake(), interface.ipv4_is_dhcp()));
                    ui.label(format!("Ip Addr / Netmask: {}/{}", interface.addr_netmask().0, interface.addr_netmask().1));
                }
                
                ui.separator();
                // disks

                ui.vertical_centered(|ui| {ui.heading(egui::RichText::new("Disks").size(20.0))});
                ui.separator();
                let temp = self.multithread_disk_details.1.try_recv();
                {
                    if temp.is_ok()
                    {
                        self.disk_details = temp.unwrap();
                    }
                }
                
                egui::Grid::new("id_salt").min_col_width(50.0).striped(true).show(ui, |ui| {

                    ui.label("Name");
                    ui.label("Capacity");
                    ui.label("Model");
                    ui.label("Serial");
                    ui.end_row();

                    for mut disk in self.disk_details.clone().into_iter()
                    {
                        
                        

                        ui.label(disk.get_name());
                        ui.label(format!("{}GB", disk.get_capaticity()));
                        ui.label(format!("{}", disk.get_model()));
                        ui.label(format!("{}", disk.get_serial()));
                        ui.end_row();
                    }

                });

                
            });

            egui::ScrollArea::vertical().id_salt("third_scroll_area").show(&mut ui[2], |ui| {
                ui.vertical_centered(|ui| {ui.heading(egui::RichText::new("Pools").size(20.0))});
                ui.separator();

                
                
                let temp = self.multithread_pool_details.1.try_recv();
                {
                    if temp.is_ok()
                    {
                        self.pool_details = temp.unwrap();
                    }
                }

                for mut pool in self.pool_details.clone().into_iter()
                {
                    ui.heading(pool.get_name());
                    
                    
                    
                    ui.label(format!("Capacity: {}GB, Free: {}GB, Used: {}GB", pool.get_capacity(), pool.get_free(), pool.get_used()));
                    ui.add(egui::widgets::ProgressBar::new(pool.get_used_normalized()));
                    
                    
                    ui.label(format!("Path: {}", pool.get_path()));
                    ui.label(format!("Healthy: {}, Error Count: {}", pool.is_healthy(), pool.scan_err()));
                    
                    ui.add_space(10.0);
                    ui.heading("Topology:");

                    let data_vdev = pool.get_topology().get_data_vdev();


                    ui.heading("⮩ Data VDEV's");

                    
                    for mut data in data_vdev.into_iter()
                    {
                        ui.horizontal(|ui|{
                            ui.add_space(10.0);
                            ui.label(format!("⮩ Name: {}, Type: {}", data.get_name(), data.get_type()));
                        });
                        
                        ui.horizontal(|ui|{
                            ui.add_space(25.0);
                            ui.label(format!("Errors; Read: {}, Write: {}, Checksum: {}", data.get_r_w_checksume_errors().0, data.get_r_w_checksume_errors().1, data.get_r_w_checksume_errors().2));
                        });
                        ui.add_space(10.0);
                    }

                    let spare_vdev = pool.get_topology().get_spare_vdev();

                    

                    
                    for mut data in spare_vdev.into_iter()
                    {
                        ui.heading("⮩ Spare VDEV's");
                        
                        ui.horizontal(|ui|{
                            ui.add_space(10.0);
                            ui.label(format!("⮩ Name: {}, Type: {}", data.get_name(), data.get_type()));
                        });
                        
                        ui.horizontal(|ui|{
                            ui.add_space(25.0);
                            ui.label(format!("Errors; Read: {}, Write: {}, Checksum: {}", data.get_r_w_checksume_errors().0, data.get_r_w_checksume_errors().1, data.get_r_w_checksume_errors().2));
                        });
                        ui.add_space(10.0);
                    }

                    ui.separator();

                }

            });

            egui::ScrollArea::vertical().id_salt("fourth_scroll_area").show(&mut ui[3], |ui| {
                ui.vertical_centered(|ui| {ui.heading(egui::RichText::new("Snapshot Tasks").size(20.0))});
                ui.separator();

                let temp = self.multithread_snapshot_details.1.try_recv();
                {
                    if temp.is_ok()
                    {
                        self.snapshot_details = temp.unwrap();
                    }
                }

                for mut snapshot in self.snapshot_details.clone().into_iter()
                {
                    ui.heading(snapshot.get_dataset());
                    ui.label(format!("ID: {}, Recursive: {}", snapshot.get_id(), snapshot.is_recursive()));
                    ui.label(format!("Schedule: {}, Lifetime: {} {}", snapshot.get_schedule(), snapshot.lifetime().0, snapshot.lifetime().1));
                    ui.separator();
                
                }

                ui.vertical_centered(|ui| {ui.heading(egui::RichText::new("Services").size(20.0))});
                ui.separator();

                let temp = self.multithread_service_details.1.try_recv();
                {
                    if temp.is_ok()
                    {
                        self.service_details = temp.unwrap();
                    }
                }

                for mut service in self.service_details.clone().into_iter()
                {
                    ui.heading(service.get_service());
                    ui.label(format!("State: {}, Enabled: {}, ID: {}", service.get_state(), service.is_enabled(), service.get_id()));
                    ui.separator();
                
                }

            });
            
        });
    }

    fn start_multi(&mut self)
    {
        if self.truenas_is_ok && !self.truenas_multithread_started
        {

            self.truenas_multithread_started = true;

            let mut svr_clone = self.truenas.clone();
            let applist_tx = self.multithread_applist.0.clone();
            std::thread::spawn(move || {
                loop {
                    let applist_thread = truenas_lib::api::v2_0::app::get(&mut svr_clone);
                    if applist_thread.is_ok()
                    {
                        let unwrap = applist_thread.unwrap();
                        let _ = applist_tx.send(unwrap);
                    }
                    std::thread::sleep(std::time::Duration::from_secs(5));
                }
            });
    
            let mut svr_clone = self.truenas.clone();
            let virtualmachine_tx = self.multithread_virtualmachinelist.0.clone();
            std::thread::spawn(move || {
                loop {
                    let virtual_thread = truenas_lib::api::v2_0::vm::get(&mut svr_clone);
                    if virtual_thread.is_ok()
                    {
                        let unwrap = virtual_thread.unwrap();
                        let _ = virtualmachine_tx.send(unwrap);
                    }
                    std::thread::sleep(std::time::Duration::from_secs(40));
                }
            });
    
            let mut svr_clone = self.truenas.clone();
            let interface_details_tx = self.multithread_interface_details.0.clone();
            std::thread::spawn(move || {
                loop {
                    let interface_thread = truenas_lib::api::v2_0::interface::get(&mut svr_clone);
                    if interface_thread.is_ok()
                    {
                        let unwrap = interface_thread.unwrap();
                        let _ = interface_details_tx.send(unwrap);
                    }
                    std::thread::sleep(std::time::Duration::from_secs(60));
                }
            });
    
    

    
            let mut svr_clone = self.truenas.clone();
            let transmitter = self.multithread_disk_details.0.clone();
            std::thread::spawn(move || {
                loop {
                    let temp = truenas_lib::api::v2_0::disk::get(&mut svr_clone);
                    if temp.is_ok()
                    {
                        let unwrap = temp.unwrap();
                        let _ = transmitter.send(unwrap);
                    }
                    std::thread::sleep(std::time::Duration::from_secs(60));
                }
            });
    
            let mut svr_clone = self.truenas.clone();
            let transmitter = self.multithread_pool_details.0.clone();
            std::thread::spawn(move || {
                loop {
                    let temp = truenas_lib::api::v2_0::pool::get(&mut svr_clone);
                    if temp.is_ok()
                    {
                        let unwrap = temp.unwrap();
                        let _ = transmitter.send(unwrap);
                    }
                    std::thread::sleep(std::time::Duration::from_secs(60));
                }
            });
    
            let mut svr_clone = self.truenas.clone();
            let transmitter = self.multithread_snapshot_details.0.clone();
            std::thread::spawn(move || {
                loop {
                    let temp = truenas_lib::api::v2_0::pool::snapshottask::get(&mut svr_clone);
                    if temp.is_ok()
                    {
                        let unwrap = temp.unwrap();
                        let _ = transmitter.send(unwrap);
                    }
                    std::thread::sleep(std::time::Duration::from_secs(60));
                }
            });
    
            let mut svr_clone = self.truenas.clone();
            let transmitter = self.multithread_service_details.0.clone();
            std::thread::spawn(move || {
                loop {
                    let temp = truenas_lib::api::v2_0::service::get(&mut svr_clone);
                    if temp.is_ok()
                    {
                        let unwrap = temp.unwrap();
                        let _ = transmitter.send(unwrap);
                    }
                    std::thread::sleep(std::time::Duration::from_secs(60));
                }
            });
        }
        


        if self.prometheus_is_ok && !self.prometheus_multithread_started
        {
            self.prometheus_multithread_started = true;

            let mut svr_clone = self.prometheus.clone();
            let cpu_usage_tx = self.multithread_cpu_usage.0.clone();
            std::thread::spawn(move || {
                loop {
                    let cpu = prometheus_lib::api::v1::query::cpu::usage::get(&mut svr_clone);
                    if cpu.is_ok()
                    {
                        let unwrap = cpu.unwrap();
                        let _ = cpu_usage_tx.send(unwrap);
                    }
                    std::thread::sleep(std::time::Duration::from_secs(5));
                }
            });
    
            let mut svr_clone = self.prometheus.clone();
            let memory_usage_tx = self.multithread_memory_usage.0.clone();
            std::thread::spawn(move || {
                loop {
                    let memory = prometheus_lib::api::v1::query::memory::Memory::load(&mut svr_clone);
                    let _ = memory_usage_tx.send(memory);
                    std::thread::sleep(std::time::Duration::from_secs(5));
                }
            });
        }


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





const TN_URL: &str = "data_0";
const TN_KEY: &str = "data_1";
const PROM_URL: &str = "data_2";
struct ConnectionData
{
    truenas_url: String,
    truenas_key: String,
    prometheus_url: String,
}


impl ConnectionData
{
    fn load() -> Self
    {
        let mut tn_key = truenas_client::encrypt::Encrypt::new(TN_KEY);
        let mut tn_url = truenas_client::encrypt::Encrypt::new(TN_URL);
        let mut prom_url = truenas_client::encrypt::Encrypt::new(PROM_URL);

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

    fn save(&mut self) -> &mut Self
    {
        let mut tn_key = truenas_client::encrypt::Encrypt::new(TN_KEY);
        let mut tn_url = truenas_client::encrypt::Encrypt::new(TN_URL);
        let mut prom_url = truenas_client::encrypt::Encrypt::new(PROM_URL);

        let _ = tn_key.set_key(&self.truenas_key).save_file();
        let _ = tn_url.set_key(&self.truenas_url).save_file();
        let _ = prom_url.set_key(&self.prometheus_url).save_file();

        self
    }


    fn set_truenas_url(&mut self, url: &str) -> &mut Self
    {
        self.truenas_url = String::from(url);
        self
    }

    fn set_truenas_key(&mut self, key: &str) -> &mut Self
    {
        self.truenas_key = String::from(key);
        self
    }

    fn set_prometheus_url(&mut self, url: &str) -> &mut Self
    {
        self.prometheus_url = String::from(url);
        self
    }

    fn get_truenas_url(&mut self) -> String
    {
        self.truenas_url.clone()
    }

    fn get_truenas_key(&mut self) -> String
    {
        self.truenas_key.clone()
    }

    fn get_prometheus_url(&mut self) -> String
    {
        self.prometheus_url.clone()
    }
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
        let cpu = prometheus_lib::api::v1::query::cpu::usage::get(&mut prometheus);
        println!("{} | {} | {} | {}", apps.get_used(), apps.in_gb().get_used(), apps.in_gb().get_total(), cpu.unwrap()) 
    }

    #[test]
    fn string() {
        let stl = String::from("hello");
        println!("{}", &stl[0..2])
    }

    #[test]
    fn num() {
        let stl: i32 = 4685555;
        println!("{}", (stl.checked_ilog10().unwrap_or(0)+1));
    }




}