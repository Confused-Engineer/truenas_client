use crate::app::Dashboard;
use eframe::egui;
use eframe::egui::vec2;
use truenas_lib::api::v2_0::pool::VdevOptions;
use egui_gauge::Gauge;


impl Dashboard
{
    
    pub fn main(&mut self, ui: &mut egui::Ui)
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
                                ui.label("State:");
                                
                                auto_color_label(&app.get_state(), ui);

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

                    ui.horizontal(|ui| {
                        ui.label("Status:");
                        auto_color_label(&vm.get_status(), ui);
                    });
                    
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
                
                egui::Grid::new("disk_grid").spacing(vec2(15.0, 5.0)).striped(true).show(ui, |ui| {

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
                    
                    if pool.is_healthy()
                    {
                        ui.horizontal(|ui| {
                            ui.label("Status:");
                            ui.colored_label(egui::Color32::GREEN, "Healthy");
                        });
                        
                    } else {
                        ui.horizontal(|ui| {
                            ui.label("Status:");
                            ui.colored_label(egui::Color32::RED, "Un-Healthy");
                            ui.label("|");
                            ui.label("Error Count:");
                            ui.colored_label(egui::Color32::RED,format!("{}", pool.scan_err()));
                            
                        });
                    }

                    ui.add_space(10.0);
                    ui.heading("Topology:");

                    for vdevs in pool.get_topology().get_all_vdevs()
                    {
                        match vdevs {
                            truenas_lib::api::v2_0::pool::PoolCompilation::Data(mut vdev) => {
                                ui.heading("⮩ Data VDEV's");
                                format_vdev(vdev.get_name(),vdev.get_r_w_checksume_errors(),vdev.get_type(),ui);
                            },
                            truenas_lib::api::v2_0::pool::PoolCompilation::Spare(mut vdev) => 
                            {
                                ui.heading("⮩ Spare VDEV's");
                                format_vdev(vdev.get_name(),vdev.get_r_w_checksume_errors(),vdev.get_type(),ui);
                            },
                        }
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
                    ui.horizontal(|ui| {
                        ui.label("State:");
                        auto_color_label(&service.get_state(), ui);
                        ui.label(format!("| Enabled: {} | ID: {}", service.is_enabled(), service.get_id()));
                    });
                    
                    
                    ui.separator();
                
                }

            });
            
        });
    }

    pub fn preferences(&mut self, ui: &mut egui::Ui)
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

    pub fn start_multi(&mut self)
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


fn auto_color_label(label: &str, ui: &mut egui::Ui)
{
    match label {
        "RUNNING" => {
            ui.colored_label(egui::Color32::GREEN, label);
        },
        "DEPLOYING" | "STOPPING" => {
            ui.colored_label(egui::Color32::YELLOW, label);
        },
        "STOPPED" => {
            ui.colored_label(egui::Color32::RED, label);
        },

        _ => {}
    }
}

fn format_vdev(name: String, r_w_checksum: (i64, i64, i64), vdevtype: String, ui: &mut egui::Ui)
{
    ui.horizontal(|ui|{
        ui.add_space(10.0);
        ui.label(format!("⮩ Name: {}, Type: {}", name, vdevtype));
    });



        
        ui.horizontal(|ui|{
            ui.add_space(25.0);
            if r_w_checksum.0 == 0 && r_w_checksum.1 == 0 && r_w_checksum.2 == 0
            {
                ui.colored_label(egui::Color32::GREEN, "No Errors");
            } else {
                ui.horizontal(|ui| {
                    ui.colored_label(egui::Color32::RED, "Errors:");
                    ui.label("Read:");

                    if r_w_checksum.0 == 0
                    {
                        ui.label(format!("{}", r_w_checksum.0));
                    } else {
                        ui.colored_label(egui::Color32::RED, format!("{}", r_w_checksum.0));
                    }

                    ui.label("Write:");

                    if r_w_checksum.1 == 0
                    {
                        ui.label(format!("{}", r_w_checksum.1));
                    } else {
                        ui.colored_label(egui::Color32::RED, format!("{}", r_w_checksum.1));
                    }

                    ui.label("Checksum:");

                    if r_w_checksum.2 == 0
                    {
                        ui.label(format!("{}", r_w_checksum.2));
                    } else {
                        ui.colored_label(egui::Color32::RED, format!("{}", r_w_checksum.2));
                    }

                    
                });
                
            }
        });
        ui.add_space(10.0);


}