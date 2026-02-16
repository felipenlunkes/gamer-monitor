use std::process::Command;
use regex::Regex;

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

    pub fn update(&mut self) {

        self.update_cpu_info();
        self.update_gpu_info();
        self.update_sensors();
        self.update_ram();
    }

    fn update_cpu_info(&mut self) {

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
        self.cpu_usage = self.get_cpu_usage();
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

    fn update_gpu_info(&mut self) {

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
                            last_bracket_content = line[absolute_start+1..absolute_end].to_string();
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

    fn update_sensors(&mut self) {

        if let Ok(output) = Command::new("sensors").output() {
            let sensors_output = String::from_utf8_lossy(&output.stdout);

            // Parse CPU temperature
            if let Some(temp) = self.extract_sensor_value(&sensors_output, r"Tctl:\s+\+([0-9.]+)") {
                self.cpu_temp = temp;
            } else if let Some(temp) = self.extract_sensor_value(&sensors_output, r"Tdie:\s+\+([0-9.]+)") {
                self.cpu_temp = temp;
            } else if let Some(temp) = self.extract_sensor_value(&sensors_output, r"Package id 0:\s+\+([0-9.]+)") {
                self.cpu_temp = temp;
            } else if let Some(temp) = self.extract_sensor_value(&sensors_output, r"Core 0:\s+\+([0-9.]+)") {
                self.cpu_temp = temp;
            }

            if self.gpu_name.contains("Radeon") || sensors_output.contains("amdgpu") {

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
                    if in_gpu_section && trimmed.ends_with(':') && trimmed.contains("-pci-") && !trimmed.starts_with("amdgpu-pci-") {
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

                        if !self.gpu_hotspot.is_empty() && !self.gpu_memory.is_empty() {
                            break;
                        }
                    }
                }
            }

            // Parse NVME temperatures
            self.nvme_temps.clear();
            let re = Regex::new(r"Composite:\s+\+([0-9.]+)").unwrap();
            for cap in re.captures_iter(&sensors_output) {
                if let Some(temp) = cap.get(1) {
                    self.nvme_temps.push(temp.as_str().to_string());
                }
            }

            // Parse fan speeds
            if let Some(fan) = self.extract_sensor_value(&sensors_output, r"fan2:\s+([0-9]+)\s+RPM") {
                self.cpu_fan = format!("{} RPM", fan);
            }
            if let Some(fan) = self.extract_sensor_value(&sensors_output, r"fan3:\s+([0-9]+)\s+RPM") {
                self.chassis_fan1 = format!("{} RPM", fan);
            }

            let patterns = vec![
                r"nct6799-isa-0290.*?fan1:\s+([0-9]+)\s+RPM",
                r"nct6798-isa-0290.*?fan1:\s+([0-9]+)\s+RPM",
            ];

            for pattern in patterns {
                if let Ok(re) = Regex::new(pattern) {
                    if let Some(cap) = re.captures(&sensors_output) {
                        if let Some(fan) = cap.get(1) {
                            self.chassis_fan2 = format!("{} RPM", fan.as_str());
                            break;
                        }
                    }
                }
            }
        }
    }

    fn extract_sensor_value(&self, text: &str, pattern: &str) -> Option<String> {

        let re = Regex::new(pattern).ok()?;
        let cap = re.captures(text)?;
        Some(cap.get(1)?.as_str().to_string())
    }

    fn update_ram(&mut self) {

        if let Ok(output) = Command::new("free")
            .arg("-m")
            .env("LC_ALL", "C")
            .output()
        {

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