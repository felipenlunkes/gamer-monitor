use regex::Regex;
use std::io;
use std::process::Command;
use once_cell::sync::Lazy;

static RE_TCTL: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"Tctl:\s+\+([0-9.]+)").unwrap());

static RE_TDIE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"Tdie:\s+\+([0-9.]+)").unwrap());

static RE_PACKAGE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"Package id 0:\s+\+([0-9.]+)").unwrap());

static RE_CORE0: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"Core 0:\s+\+([0-9.]+)").unwrap());

static RE_NVME: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"Composite:\s+\+([0-9.]+)").unwrap());

static RE_CPU_FAN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"fan2:\s+([0-9]+)\s+RPM").unwrap());

static RE_CHASSIS1: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"fan3:\s+([0-9]+)\s+RPM").unwrap());

static RE_CHASSIS2_A: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"nct6799-isa-0290.*?fan1:\s+([0-9]+)\s+RPM").unwrap());

static RE_CHASSIS2_B: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"nct6798-isa-0290.*?fan1:\s+([0-9]+)\s+RPM").unwrap());

#[derive(Debug, Clone, Default)]
pub struct SensorData {
    pub cpu_name: String,
    pub cpu_temp: String,
    pub cpu_usage: f32,

    pub gpu_name: String,
    pub gpu_edge: String,
    pub gpu_hotspot: String,
    pub gpu_memory: String,
    pub gpu_fan: String,

    pub nvme_temps: Vec<String>,

    pub cpu_fan: String,
    pub chassis_fan1: String,
    pub chassis_fan2: String,

    pub ram_total: f32,
    pub ram_used: f32,
    pub ram_free: f32,
    pub ram_available: f32,
    pub ram_percent: f32,
}

impl SensorData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn identify_hardware(&mut self) {
        self.identify_processor();
        self.identify_gpu();
    }

    pub fn update(&mut self) {
        self.cpu_usage = self.get_cpu_usage();
        self.update_sensors();
        self.update_ram();
    }

    fn identify_processor(&mut self) {
        if let Ok(cpuinfo) = std::fs::read_to_string("/proc/cpuinfo") {
            for line in cpuinfo.lines() {
                if line.starts_with("model name") {
                    if let Some(name) = line.split(':').nth(1) {
                        self.cpu_name = name.trim().to_string();
                        break;
                    }
                }
            }
        }
    }

    fn get_cpu_usage(&self) -> f32 {
        let stat1 = std::fs::read_to_string("/proc/stat").unwrap_or_default();
        let values1: Vec<u64> = stat1
            .lines()
            .next()
            .unwrap_or("")
            .split_whitespace()
            .skip(1)
            .filter_map(|s| s.parse().ok())
            .collect();

        std::thread::sleep(std::time::Duration::from_millis(100));

        let stat2 = std::fs::read_to_string("/proc/stat").unwrap_or_default();
        let values2: Vec<u64> = stat2
            .lines()
            .next()
            .unwrap_or("")
            .split_whitespace()
            .skip(1)
            .filter_map(|s| s.parse().ok())
            .collect();

        if values1.len() >= 4 && values2.len() >= 4 {
            let idle1 = values1[3] + values1.get(4).unwrap_or(&0);
            let idle2 = values2[3] + values2.get(4).unwrap_or(&0);

            let total1: u64 = values1.iter().take(8).sum();
            let total2: u64 = values2.iter().take(8).sum();

            let total_diff = total2.saturating_sub(total1);
            let idle_diff = idle2.saturating_sub(idle1);

            if total_diff > 0 {
                return ((total_diff - idle_diff) as f32 / total_diff as f32) * 100.0;
            }
        }
        0.0
    }

    fn identify_gpu(&mut self) {
        if let Ok(output) = Command::new("nvidia-smi")
            .arg("--query-gpu=name")
            .arg("--format=csv,noheader")
            .output()
        {
            if output.status.success() {
                let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !name.is_empty() {
                    self.gpu_name = name;
                    return;
                }
            }
        }

        if let Ok(output) = Command::new("lspci").output() {
            let lspci_out = String::from_utf8_lossy(&output.stdout);

            for line in lspci_out.lines() {
                if line.contains("VGA compatible controller") {
                    let mut last_bracket_content = String::new();
                    let mut current_pos = 0;

                    while let Some(start) = line[current_pos..].find('[') {
                        let absolute_start = current_pos + start;
                        if let Some(end) = line[absolute_start..].find(']') {
                            let absolute_end = absolute_start + end;
                            last_bracket_content =
                                line[absolute_start + 1..absolute_end].to_string();
                            current_pos = absolute_end + 1;
                        } else {
                            break;
                        }
                    }

                    if !last_bracket_content.is_empty() && last_bracket_content != "AMD/ATI" {
                        self.gpu_name = last_bracket_content;
                        return;
                    }

                    if let Some(controller_part) = line.split("controller:").nth(1) {
                        let gpu_part = controller_part.trim().to_string();
                        if !gpu_part.is_empty() {
                            self.gpu_name = gpu_part;
                            return;
                        }
                    }
                }
            }
        }

        if self.gpu_name.is_empty() {
            self.gpu_name = "Unknown GPU".to_string();
        }
    }

    fn update_nvidia_gpu_info(&mut self) {

        if let Ok(output) = Command::new("nvidia-smi")
            .arg("--query-gpu=temperature.gpu,fan.speed")
            .arg("--format=csv,noheader,nounits")
            .output()
        {
            if output.status.success() {
                let out = String::from_utf8_lossy(&output.stdout);
                let parts: Vec<&str> = out.trim().split(',').collect();

                if let Some(temp) = parts.get(0) {
                    self.gpu_edge = temp.trim().to_string();
                }

                if let Some(fan) = parts.get(1) {
                    self.gpu_fan = format!("{} RPM", fan.trim());
                }
            }
        }

        return;
    }

    fn update_radeon_gpu_info(&mut self, sensors_output: &String) {

        let lines: Vec<&str> = sensors_output.lines().collect();
        let mut in_gpu_section = false;

        for line in lines {
            let trimmed = line.trim_start();

            // Detects a Radeon graphics card
            if trimmed.starts_with("amdgpu-pci-") {
                in_gpu_section = true;

                self.gpu_edge.clear();
                self.gpu_hotspot.clear();
                self.gpu_memory.clear();
                self.gpu_fan.clear();

                continue;
            }

            // Exit if found another PCI device
            if in_gpu_section
                && !trimmed.starts_with("amdgpu-pci-")
                && !trimmed.is_empty()
                && !trimmed.starts_with(' ')
                && !trimmed.starts_with('\t')
                && (trimmed.contains("-isa-") || trimmed.contains("-pci-") || trimmed.contains("-i2c-")) {
                in_gpu_section = false;
            }

            if in_gpu_section {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.is_empty() {
                    continue;
                }

                match parts[0] {
                    label if label.starts_with("junction") && parts.len() >= 2 => {
                        self.gpu_hotspot = parts[1].trim_start_matches('+').to_string();
                    }
                    label if label.starts_with("edge") && parts.len() >= 2 => {
                        self.gpu_edge = parts[1].trim_start_matches('+').to_string();
                    }
                    label if label.starts_with("mem") && parts.len() >= 2 => {
                        self.gpu_memory = parts[1].trim_start_matches('+').to_string();
                    }
                    label if label.starts_with("fan") && parts.len() >= 2 => {
                        self.gpu_fan = format!("{} RPM", parts[1]);
                    }
                    _ => {}
                }
            }
        }
    }

    fn update_gpu_info(&mut self, sensors_output: &String) {

        if self.gpu_name.to_lowercase().contains("nvidia") {
            self.update_nvidia_gpu_info();
        }

        if self.gpu_name.contains("Radeon") || sensors_output.contains("amdgpu") {
            self.update_radeon_gpu_info(sensors_output);
        }
    }

    fn get_sensors_data(&mut self) -> Result<String, io::Error> {
        let output = Command::new("sensors").output()?;
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }

    fn update_cpu_info(&mut self, sensors_output: &String) {
        // Parse CPU temperature
        if let Some(temp) = self.extract_sensor_value(&sensors_output, &RE_TCTL) {
            self.cpu_temp = temp;
        } else if let Some(temp) =
            self.extract_sensor_value(&sensors_output, &RE_TDIE)
        {
            self.cpu_temp = temp;
        } else if let Some(temp) =
            self.extract_sensor_value(&sensors_output, &RE_PACKAGE)
        {
            self.cpu_temp = temp;
        } else if let Some(temp) =
            self.extract_sensor_value(&sensors_output, &RE_CORE0)
        {
            self.cpu_temp = temp;
        }
    }

    fn update_nvme_info(&mut self, sensors_output: &String) {
        self.nvme_temps.clear();
        for cap in RE_NVME.captures_iter(sensors_output) {
            if let Some(temp) = cap.get(1) {
                self.nvme_temps.push(temp.as_str().to_string());
            }
        }
    }

    fn update_fan_info(&mut self, sensors_output: &String) {
        if let Some(fan) = RE_CPU_FAN.captures(sensors_output).and_then(|c| c.get(1)) {
            self.cpu_fan = format!("{} RPM", fan.as_str());
        }

        if let Some(fan) = RE_CHASSIS1.captures(sensors_output).and_then(|c| c.get(1)) {
            self.chassis_fan1 = format!("{} RPM", fan.as_str());
        }

        if let Some(fan) = RE_CHASSIS2_A.captures(sensors_output)
            .or_else(|| RE_CHASSIS2_B.captures(sensors_output))
            .and_then(|c| c.get(1))
        {
            self.chassis_fan2 = format!("{} RPM", fan.as_str());
        }
    }

    fn update_sensors(&mut self) {
        let sensors_data_found = self.get_sensors_data();
        if sensors_data_found.is_err() {
            return;
        }

        let sensors_output = sensors_data_found.unwrap();

        self.update_cpu_info(&sensors_output);
        self.update_gpu_info(&sensors_output);
        self.update_nvme_info(&sensors_output);
        self.update_fan_info(&sensors_output);
    }

    fn extract_sensor_value(&self, text: &str, re: &Regex) -> Option<String> {
        let cap = re.captures(text)?;
        Some(cap.get(1)?.as_str().to_string())
    }

    fn update_ram(&mut self) {
        if let Ok(output) = Command::new("free").arg("-m").env("LC_ALL", "C").output() {
            let free_output = String::from_utf8_lossy(&output.stdout);

            for line in free_output.lines() {
                if line.starts_with("Mem:") || line.starts_with("Mem.:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();

                    if parts.len() >= 7 {
                        if let Ok(total) = parts[1].parse::<f32>() {
                            self.ram_total = total / 1024.0;
                        }
                        if let Ok(used) = parts[2].parse::<f32>() {
                            self.ram_used = used / 1024.0;
                        }
                        if let Ok(free) = parts[3].parse::<f32>() {
                            self.ram_free = free / 1024.0;
                        }
                        if let Ok(available) = parts[6].parse::<f32>() {
                            self.ram_available = available / 1024.0;
                        }

                        if self.ram_total > 0.0 {
                            self.ram_percent = (self.ram_used / self.ram_total) * 100.0;
                        }
                    }
                    break;
                }
            }
        }
    }
}
