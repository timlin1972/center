use actix_web::{HttpResponse, Responder};
use log::info;
use serde::Serialize;
use serde_json::json;
use systemstat::{saturating_sub_bytes, Platform, System};

#[derive(Serialize, Debug)]
struct LoadAverage {
    one: f32,
    five: f32,
    fifteen: f32,
}

impl Default for LoadAverage {
    fn default() -> Self {
        Self::new()
    }
}

impl LoadAverage {
    pub fn new() -> LoadAverage {
        LoadAverage {
            one: 0.0,
            five: 0.0,
            fifteen: 0.0,
        }
    }
}

#[derive(Serialize, Debug)]
struct Memory {
    used: u64,
    total: u64,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    pub fn new() -> Memory {
        Memory { used: 0, total: 0 }
    }
}

#[derive(Serialize, Debug)]
struct Battery {
    remaining_capacity: f32,
    remaining_secs: u64,
}

impl Default for Battery {
    fn default() -> Self {
        Self::new()
    }
}

impl Battery {
    pub fn new() -> Battery {
        Battery {
            remaining_capacity: 0.0,
            remaining_secs: 0,
        }
    }
}

#[derive(Serialize, Debug)]
struct SysStat {
    battery: Battery,
    ac_power: bool,
    memory: Memory,
    load_average: LoadAverage,
    uptime: u64,
    boot_time: String,
    cpu_temp: f32,
}

impl Default for SysStat {
    fn default() -> Self {
        Self::new()
    }
}

impl SysStat {
    pub fn new() -> SysStat {
        SysStat {
            battery: Battery::new(),
            ac_power: false,
            memory: Memory::new(),
            load_average: LoadAverage::new(),
            uptime: 0,
            boot_time: "".to_owned(),
            cpu_temp: 0.0,
        }
    }
}

pub async fn get() -> impl Responder {
    info!(">>> recv: get");

    let mut sys_stat = SysStat::new();

    let sys = System::new();

    if let Ok(battery) = sys.battery_life() {
        sys_stat.battery.remaining_capacity = battery.remaining_capacity;
        sys_stat.battery.remaining_secs = battery.remaining_time.as_secs();
    }

    if let Ok(power) = sys.on_ac_power() {
        sys_stat.ac_power = power;
    }

    if let Ok(mem) = sys.memory() {
        sys_stat.memory.used = saturating_sub_bytes(mem.total, mem.free).as_u64();
        sys_stat.memory.total = mem.total.as_u64();
    }

    if let Ok(loadavg) = sys.load_average() {
        sys_stat.load_average.one = loadavg.one;
        sys_stat.load_average.five = loadavg.five;
        sys_stat.load_average.fifteen = loadavg.fifteen;
    }

    if let Ok(uptime) = sys.uptime() {
        sys_stat.uptime = uptime.as_secs();
    }

    if let Ok(boot_time) = sys.boot_time() {
        sys_stat.boot_time = boot_time.to_string();
    }

    if let Ok(cpu_temp) = sys.cpu_temp() {
        sys_stat.cpu_temp = cpu_temp;
    }

    HttpResponse::Ok().body(json!(sys_stat).to_string())
}
