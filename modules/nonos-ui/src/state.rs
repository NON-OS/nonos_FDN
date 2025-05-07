use sysinfo::System;

pub struct AppState {
    pub sys: System,
    pub ticks: u64,
    pub relay_status: u8,
}

impl Default for AppState {
    fn default() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        Self {
            sys,
            ticks: 0,
            relay_status: 0,
        }
    }
}

impl AppState {
    pub fn tick(&mut self) {
        self.ticks += 1;
        self.sys.refresh_memory();
    }

    pub fn toggle_relay(&mut self) {
        self.relay_status = if self.relay_status == 0 { 1 } else { 0 };
    }
}
