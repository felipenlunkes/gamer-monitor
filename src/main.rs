mod sensors;

use glib::timeout_add_seconds_local;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, Frame, Grid, Label, Orientation, ProgressBar, ScrolledWindow};
use sensors::SensorData;
use std::cell::RefCell;
use std::rc::Rc;

const APP_ID: &str = "com.lunx.GamerMonitor";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_TITLE: &str = "Gamer Monitor";

fn main() {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {

    // Create sensor data
    let sensor_data = Rc::new(RefCell::new(SensorData::new()));
    sensor_data.borrow_mut().identify_hardware();
    sensor_data.borrow_mut().update();

    let app_title = APP_TITLE.to_string() + " v" + VERSION;

    // Create main window
    let window = ApplicationWindow::builder()
        .application(app)
        .title(app_title)
        .default_width(600)
        .default_height(700)
        .build();

    // Main container
    let main_box = Box::new(Orientation::Vertical, 10);
    main_box.set_margin_top(10);
    main_box.set_margin_bottom(10);
    main_box.set_margin_start(10);
    main_box.set_margin_end(10);

    // Title
    let title = Label::new(Some("Gamer Monitor"));
    title.add_css_class("title-1");
    main_box.append(&title);

    // CPU Section
    let cpu_frame = create_cpu_section(&sensor_data);
    main_box.append(&cpu_frame);

    // GPU Section
    let gpu_frame = create_gpu_section(&sensor_data);
    main_box.append(&gpu_frame);

    // Storage Section
    let storage_frame = create_storage_section(&sensor_data);
    main_box.append(&storage_frame);

    // RAM Section
    let ram_frame = create_ram_section(&sensor_data);
    main_box.append(&ram_frame);

    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scrolled_window.set_child(Some(&main_box));
    window.set_child(Some(&scrolled_window));

    let sensor_data_clone = sensor_data.clone();

    sensor_data_clone.borrow_mut().identify_hardware();

    // Update every 5 seconds
    timeout_add_seconds_local(5, move || {
        sensor_data_clone.borrow_mut().update();
        glib::ControlFlow::Continue
    });

    window.present();
}

fn create_cpu_section(sensor_data: &Rc<RefCell<SensorData>>) -> Frame {

    let frame = Frame::new(Some("CPU"));
    let grid = Grid::new();
    grid.set_margin_top(10);
    grid.set_margin_bottom(10);
    grid.set_margin_start(10);
    grid.set_margin_end(10);
    grid.set_row_spacing(8);
    grid.set_column_spacing(10);

    // CPU Name
    let cpu_name_label = Label::new(Some("Model:"));
    cpu_name_label.set_halign(gtk4::Align::Start);

    let cpu_name_value = Label::new(Some(&sensor_data.borrow().cpu_name));
    cpu_name_value.set_halign(gtk4::Align::Start);
    cpu_name_value.add_css_class("dim-label");
    grid.attach(&cpu_name_label, 0, 0, 1, 1);
    grid.attach(&cpu_name_value, 1, 0, 2, 1);

    // CPU Temperature
    let cpu_temp_label = Label::new(Some("Temperature (°C):"));
    cpu_temp_label.set_halign(gtk4::Align::Start);
    
    let cpu_temp_value = Label::new(Some(&sensor_data.borrow().cpu_temp));
    cpu_temp_value.set_halign(gtk4::Align::Start);
    grid.attach(&cpu_temp_label, 0, 1, 1, 1);
    grid.attach(&cpu_temp_value, 1, 1, 1, 1);

    // CPU Usage
    let cpu_usage_label = Label::new(Some("Load:"));
    cpu_usage_label.set_halign(gtk4::Align::Start);

    let cpu_progress = ProgressBar::new();
    cpu_progress.set_hexpand(true);
    cpu_progress.set_fraction(sensor_data.borrow().cpu_usage as f64 / 100.0);
    cpu_progress.set_show_text(true);
    cpu_progress.set_text(Some(&format!("{:.1}%", sensor_data.borrow().cpu_usage)));
    grid.attach(&cpu_usage_label, 0, 2, 1, 1);
    grid.attach(&cpu_progress, 1, 2, 2, 1);

    // Update closure
    let cpu_name_value_clone = cpu_name_value.clone();
    let cpu_temp_value_clone = cpu_temp_value.clone();
    let cpu_progress_clone = cpu_progress.clone();
    let sensor_data_clone = sensor_data.clone();
    
    timeout_add_seconds_local(2, move || {
        let data = sensor_data_clone.borrow();
        cpu_name_value_clone.set_text(&data.cpu_name);
        cpu_temp_value_clone.set_text(&data.cpu_temp);
        cpu_progress_clone.set_fraction(data.cpu_usage as f64 / 100.0);
        cpu_progress_clone.set_text(Some(&format!("{:.1}%", data.cpu_usage)));
        glib::ControlFlow::Continue
    });

    frame.set_child(Some(&grid));
    frame
}

fn create_gpu_section(sensor_data: &Rc<RefCell<SensorData>>) -> Frame {

    let frame = Frame::new(Some("GPU (graphics card)"));
    let grid = Grid::new();
    grid.set_margin_top(10);
    grid.set_margin_bottom(10);
    grid.set_margin_start(10);
    grid.set_margin_end(10);
    grid.set_row_spacing(8);
    grid.set_column_spacing(10);

    let mut row = 0;

    // GPU Name
    let gpu_name_label = Label::new(Some("Model:"));
    gpu_name_label.set_halign(gtk4::Align::Start);
    let gpu_name_value = Label::new(Some(&sensor_data.borrow().gpu_name));
    gpu_name_value.set_halign(gtk4::Align::Start);
    gpu_name_value.add_css_class("dim-label");
    grid.attach(&gpu_name_label, 0, row, 1, 1);
    grid.attach(&gpu_name_value, 1, row, 1, 1);
    row += 1;

    // Check GPU vendor
    let is_radeon = sensor_data.borrow().gpu_name.contains("Radeon");
    let is_nvidia = sensor_data.borrow().gpu_name.to_lowercase().contains("nvidia");

    let hotspot_value = Label::new(Some(&sensor_data.borrow().gpu_hotspot));
    let edge_value = Label::new(Some(&sensor_data.borrow().gpu_edge));
    let memory_value = Label::new(Some(&sensor_data.borrow().gpu_memory));
    let fan_value = Label::new(Some(&sensor_data.borrow().gpu_fan));
    let power_value = Label::new(Some(&sensor_data.borrow().gpu_power));
    let vram_used = sensor_data.borrow().gpu_vram_used.clone();
    let vram_total = sensor_data.borrow().gpu_vram_total.clone();
    let vram_fraction = vram_used.parse::<f64>().unwrap_or(0.0)
        / vram_total.parse::<f64>().unwrap_or(1.0).max(1.0);
    let vram_progress = ProgressBar::new();
    vram_progress.set_hexpand(true);
    vram_progress.set_fraction(vram_fraction);
    vram_progress.set_show_text(true);
    vram_progress.set_text(Some(&format!("{} / {} MiB", vram_used, vram_total)));

    let gpu_util_progress = ProgressBar::new();
    gpu_util_progress.set_hexpand(true);
    gpu_util_progress.set_fraction(sensor_data.borrow().gpu_utilization as f64 / 100.0);
    gpu_util_progress.set_show_text(true);
    gpu_util_progress.set_text(Some(&format!("{:.0}%", sensor_data.borrow().gpu_utilization)));

    if is_radeon {
        // Hotspot
        let hotspot_label = Label::new(Some("Hotspot (°C):"));
        hotspot_label.set_halign(gtk4::Align::Start);
        hotspot_value.set_halign(gtk4::Align::Start);
        grid.attach(&hotspot_label, 0, row, 1, 1);
        grid.attach(&hotspot_value, 1, row, 1, 1);
        row += 1;

        // Edge
        let edge_label = Label::new(Some("Edge (°C):"));
        edge_label.set_halign(gtk4::Align::Start);
        edge_value.set_halign(gtk4::Align::Start);
        grid.attach(&edge_label, 0, row, 1, 1);
        grid.attach(&edge_value, 1, row, 1, 1);
        row += 1;

        // Memory
        let memory_label = Label::new(Some("Memory (°C):"));
        memory_label.set_halign(gtk4::Align::Start);
        memory_value.set_halign(gtk4::Align::Start);
        grid.attach(&memory_label, 0, row, 1, 1);
        grid.attach(&memory_value, 1, row, 1, 1);
        row += 1;

        // Fan
        let fan_label = Label::new(Some("Fan (RPM):"));
        fan_label.set_halign(gtk4::Align::Start);
        fan_value.set_halign(gtk4::Align::Start);
        grid.attach(&fan_label, 0, row, 1, 1);
        grid.attach(&fan_value, 1, row, 1, 1);
    } else if is_nvidia {
        // Temperature
        let temp_label = Label::new(Some("Temperature (°C):"));
        temp_label.set_halign(gtk4::Align::Start);
        edge_value.set_halign(gtk4::Align::Start);
        grid.attach(&temp_label, 0, row, 1, 1);
        grid.attach(&edge_value, 1, row, 1, 1);
        row += 1;

        // Fan speed
        let fan_label = Label::new(Some("Fan:"));
        fan_label.set_halign(gtk4::Align::Start);
        fan_value.set_halign(gtk4::Align::Start);
        grid.attach(&fan_label, 0, row, 1, 1);
        grid.attach(&fan_value, 1, row, 1, 1);
        row += 1;

        // VRAM usage
        let vram_label = Label::new(Some("VRAM:"));
        vram_label.set_halign(gtk4::Align::Start);
        grid.attach(&vram_label, 0, row, 1, 1);
        grid.attach(&vram_progress, 1, row, 2, 1);
        row += 1;

        // Power draw
        let power_label = Label::new(Some("Power:"));
        power_label.set_halign(gtk4::Align::Start);
        power_value.set_halign(gtk4::Align::Start);
        grid.attach(&power_label, 0, row, 1, 1);
        grid.attach(&power_value, 1, row, 1, 1);
        row += 1;

        // GPU utilization
        let util_label = Label::new(Some("Load:"));
        util_label.set_halign(gtk4::Align::Start);
        grid.attach(&util_label, 0, row, 1, 1);
        grid.attach(&gpu_util_progress, 1, row, 2, 1);
    } else {
        // Fallback for Intel/unknown
        let temp_label = Label::new(Some("Temperature (°C):"));
        temp_label.set_halign(gtk4::Align::Start);
        edge_value.set_halign(gtk4::Align::Start);
        grid.attach(&temp_label, 0, row, 1, 1);
        grid.attach(&edge_value, 1, row, 1, 1);
    }

    // Update closure
    let gpu_name_value_clone = gpu_name_value.clone();
    let hotspot_value_clone = hotspot_value.clone();
    let edge_value_clone = edge_value.clone();
    let memory_value_clone = memory_value.clone();
    let fan_value_clone = fan_value.clone();
    let power_value_clone = power_value.clone();
    let vram_progress_clone = vram_progress.clone();
    let gpu_util_progress_clone = gpu_util_progress.clone();
    let sensor_data_clone = sensor_data.clone();

    timeout_add_seconds_local(2, move || {
        let data = sensor_data_clone.borrow();
        gpu_name_value_clone.set_text(&data.gpu_name);
        hotspot_value_clone.set_text(&data.gpu_hotspot);
        edge_value_clone.set_text(&data.gpu_edge);
        memory_value_clone.set_text(&data.gpu_memory);
        fan_value_clone.set_text(&data.gpu_fan);
        power_value_clone.set_text(&data.gpu_power);
        let used = data.gpu_vram_used.parse::<f64>().unwrap_or(0.0);
        let total = data.gpu_vram_total.parse::<f64>().unwrap_or(1.0).max(1.0);
        vram_progress_clone.set_fraction(used / total);
        vram_progress_clone.set_text(Some(&format!("{} / {} MiB", data.gpu_vram_used, data.gpu_vram_total)));
        gpu_util_progress_clone.set_fraction(data.gpu_utilization as f64 / 100.0);
        gpu_util_progress_clone.set_text(Some(&format!("{:.0}%", data.gpu_utilization)));
        glib::ControlFlow::Continue
    });

    frame.set_child(Some(&grid));
    frame
}

fn create_storage_section(sensor_data: &Rc<RefCell<SensorData>>) -> Frame {
    let frame = Frame::new(Some("Storage (NVME)"));
    let storage_box = Box::new(Orientation::Vertical, 5);
    storage_box.set_margin_top(10);
    storage_box.set_margin_bottom(10);
    storage_box.set_margin_start(10);
    storage_box.set_margin_end(10);

    // Create initial labels based on detected NVMe drives
    let nvme_count = sensor_data.borrow().nvme_temps.len();
    let mut initial_labels: Vec<Label> = Vec::new();

    if nvme_count == 0 {
        let label = Label::new(Some("No NVMe devices detected"));
        label.set_halign(gtk4::Align::Start);
        storage_box.append(&label);
        initial_labels.push(label);
    } else {
        for i in 0..nvme_count {
            let temp = &sensor_data.borrow().nvme_temps[i];
            let label = Label::new(Some(&format!("NVME {}: {} °C", i + 1, temp)));
            label.set_halign(gtk4::Align::Start);
            storage_box.append(&label);
            initial_labels.push(label);
        }
    }

    let nvme_labels = Rc::new(RefCell::new(initial_labels));

    // Update closure
    let sensor_data_clone = sensor_data.clone();
    let storage_box_clone = storage_box.clone();
    let nvme_labels_clone = nvme_labels.clone();
    timeout_add_seconds_local(2, move || {
        let data = sensor_data_clone.borrow();
        let mut labels = nvme_labels_clone.borrow_mut();
        let count = data.nvme_temps.len();

        // Remove all existing labels
        for label in labels.iter() {
            storage_box_clone.remove(label);
        }
        labels.clear();

        if count == 0 {
            let label = Label::new(Some("No NVMe devices detected"));
            label.set_halign(gtk4::Align::Start);
            storage_box_clone.append(&label);
            labels.push(label);
        } else {
            for (i, temp) in data.nvme_temps.iter().enumerate() {
                let label = Label::new(Some(&format!("NVME {}: {} °C", i + 1, temp)));
                label.set_halign(gtk4::Align::Start);
                storage_box_clone.append(&label);
                labels.push(label);
            }
        }

        glib::ControlFlow::Continue
    });

    frame.set_child(Some(&storage_box));
    frame
}

fn create_ram_section(sensor_data: &Rc<RefCell<SensorData>>) -> Frame {
    let frame = Frame::new(Some("RAM Memory"));
    let grid = Grid::new();
    grid.set_margin_top(10);
    grid.set_margin_bottom(10);
    grid.set_margin_start(10);
    grid.set_margin_end(10);
    grid.set_row_spacing(8);
    grid.set_column_spacing(10);

    // Total
    let total_label = Label::new(Some("Total:"));
    total_label.set_halign(gtk4::Align::Start);
    let total_value = Label::new(Some(&format!("{:.1} GB", sensor_data.borrow().ram_total)));
    total_value.set_halign(gtk4::Align::Start);
    grid.attach(&total_label, 0, 0, 1, 1);
    grid.attach(&total_value, 1, 0, 1, 1);

    // Used
    let used_label = Label::new(Some("Used:"));
    used_label.set_halign(gtk4::Align::Start);
    let used_value = Label::new(Some(&format!("{:.1} GB", sensor_data.borrow().ram_used)));
    used_value.set_halign(gtk4::Align::Start);
    grid.attach(&used_label, 0, 1, 1, 1);
    grid.attach(&used_value, 1, 1, 1, 1);

    // Free
    let free_label = Label::new(Some("Free:"));
    free_label.set_halign(gtk4::Align::Start);
    let free_value = Label::new(Some(&format!("{:.1} GB", sensor_data.borrow().ram_free)));
    free_value.set_halign(gtk4::Align::Start);
    grid.attach(&free_label, 0, 2, 1, 1);
    grid.attach(&free_value, 1, 2, 1, 1);

    // Available
    let available_label = Label::new(Some("Available:"));
    available_label.set_halign(gtk4::Align::Start);
    let available_value = Label::new(Some(&format!("{:.1} GB", sensor_data.borrow().ram_available)));
    available_value.set_halign(gtk4::Align::Start);
    grid.attach(&available_label, 0, 3, 1, 1);
    grid.attach(&available_value, 1, 3, 1, 1);

    // Load bar
    let load_label = Label::new(Some("Load:"));
    load_label.set_halign(gtk4::Align::Start);
    let ram_progress = ProgressBar::new();
    ram_progress.set_hexpand(true);
    ram_progress.set_fraction(sensor_data.borrow().ram_percent as f64 / 100.0);
    ram_progress.set_show_text(true);
    ram_progress.set_text(Some(&format!("{:.1}%", sensor_data.borrow().ram_percent)));
    grid.attach(&load_label, 0, 4, 1, 1);
    grid.attach(&ram_progress, 1, 4, 2, 1);

    // Update closure
    let total_value_clone = total_value.clone();
    let used_value_clone = used_value.clone();
    let free_value_clone = free_value.clone();
    let available_value_clone = available_value.clone();
    let ram_progress_clone = ram_progress.clone();
    let sensor_data_clone = sensor_data.clone();

    timeout_add_seconds_local(5, move || {

        let data = sensor_data_clone.borrow();
        total_value_clone.set_text(&format!("{:.1} GB", data.ram_total));
        used_value_clone.set_text(&format!("{:.1} GB", data.ram_used));
        free_value_clone.set_text(&format!("{:.1} GB", data.ram_free));
        available_value_clone.set_text(&format!("{:.1} GB", data.ram_available));
        ram_progress_clone.set_fraction(data.ram_percent as f64 / 100.0);
        ram_progress_clone.set_text(Some(&format!("{:.1}%", data.ram_percent)));
        glib::ControlFlow::Continue
    });

    frame.set_child(Some(&grid));
    frame
}
