use log::error;
use serenity::all::*;
use serenity::builder::CreateEmbed;
use tokio::time::Instant;
use sysinfo::{
    System, 
    SystemExt, 
    CpuExt
};

use crate::discord::ShardManagerContainer;
use crate::utils;
use crate::utils::{TimeFormatOptions, TemperatureOptions, ProgressBarOptions};

/// Enhanced statistics command with comprehensive metrics and bot statistics.
pub async fn handle_statistics(
    context: &Context,
    interaction: &CommandInteraction,
) {
    // Acknowledge interaction immediately to prevent timeouts during data collection
    let thinking_response = CreateInteractionResponse::Defer(
        CreateInteractionResponseMessage::new().ephemeral(false)
    );
    
    if let Err(error) = interaction.create_response(&context.http, thinking_response).await {
        error!("Failed to create deferring response: {:?}", error);
        return;
    }
    
    // Start timer to measure data collection time
    let start_time = Instant::now();
    
    // Access the ShardManager from Context.data
    let data_read = context.data.read().await;
    let shard_manager_lock = data_read
        .get::<ShardManagerContainer>()
        .expect("Expected ShardManagerContainer in TypeMap")
        .clone();

    // Look up the single shard's runner info (shard ID 0)
    let runners = &shard_manager_lock.runners;
    let runners_guard = runners.lock().await;
    let runner_info = runners_guard
        .get(&ShardId(0))
        .expect("Shard 0 runner not found");

    // Retrieve the WebSocket latency
    let latency_display = match runner_info.latency {
        Some(duration) => utils::format_duration(duration.as_millis(), None, None),
        None => "`N/A`".to_string(),
    };

    // Bot statistics
    let mut system = System::new();
    
    // Refresh system information for accurate metrics
    system.refresh_cpu();                // Need this for CPU info
    system.refresh_memory();             // Need this for memory info
    system.refresh_components_list();    // loads the available sensors
    system.refresh_components();         // refreshes temperature readings
    system.refresh_processes();          // Need this for process count
    system.refresh_all();                // Get complete system info
    
    // Get CPU usage percentage for progress bar
    let cpu_usage = system.global_cpu_info().cpu_usage();
    let cpu_progress_options = ProgressBarOptions {
        width: 20,
        fill_char: '█',
        empty_char: '░',
        show_percentage: true,
        show_values: false,
        value_precision: 1,
        percentage_precision: 1,
        unit: "%".to_string(),
    };
    
    let cpu_bar = utils::create_progress_bar(cpu_usage as f64, 100.0, Some(cpu_progress_options));

    // Format memory information with visual bar
    let used_mem = system.used_memory() as f64 / 1024.0 / 1024.0;
    let total_mem = system.total_memory() as f64 / 1024.0 / 1024.0;
    
    let progress_options = ProgressBarOptions {
        width: 20,
        fill_char: '█',
        empty_char: '░',
        show_percentage: true,
        show_values: true,
        value_precision: 1,
        percentage_precision: 1,
        unit: "MB".to_string(),
    };
    
    let memory_bar = utils::create_progress_bar(used_mem, total_mem, Some(progress_options));

    // Gather disk information
    let disk_info = match std::env::current_dir() {
        Ok(path) => {
            if let Ok(stats) = std::fs::metadata(&path) {
                format!("Current directory size: `{}`", utils::format_file_size(stats.len(), None, None))
            } else {
                "Disk info unavailable".to_string()
            }
        },
        Err(_) => "Failed to get current directory".to_string()
    };
    
    // Enhanced uptime format with custom options
    let time_options = TimeFormatOptions {
        include_days: true,
        include_hours: true,
        include_minutes: true,
        include_seconds: true,
        short_units: false,
        max_units: 3,  // Limit to 3 most significant units
    };
    
    let uptime_formatted = utils::format_uptime(system.uptime(), Some(time_options));
    
    // Time taken to gather metrics
    let collection_time = utils::format_duration(
        start_time.elapsed().as_millis(),
        None,
        None
    );

    // Temperature options
    let temp_options = TemperatureOptions {
        precision: 1,
        include_all: false,
        use_fahrenheit: false,
        include_labels: true,
    };

    // System info for field
    let system_info = format!(
        "OS: `{} ({})`\n\
         Hostname: `{}`",
        std::env::consts::OS,
        std::env::consts::ARCH,
        system.host_name().unwrap_or_else(|| "Unknown".to_string())
    );

    // Format system information
    let embed = CreateEmbed::new()
        .title("Statistics")
        .colour(Colour::DARK_GREEN)
        .field("Bot Status", format!(
            "Discord WebSocket latency: `{}`\n\
             Computer uptime: `{}`",
            latency_display, uptime_formatted
        ), false)
        .field("CPU Cores", format!(
            "`{}` logical, `{}` physical", 
            system.cpus().len(),
            system.physical_core_count().unwrap_or(0)
        ), true)
        .field("CPU Usage", format!(
            "{}",
            cpu_bar
        ), false)
        .field("Memory Usage", memory_bar, false)
        .field("Processes", format!(
            "`{}` processes", 
            system.processes().len()
        ), true)
        .field("Temperature", utils::format_temperature(&mut system, Some(temp_options)), true)
        .field("Storage", disk_info, false)
        .field("Metrics Collection Time", format!("`{}`", collection_time), true)
        .field("System Information", system_info, false);

    // Fix the edit_response call with the correct type
    if let Err(error) = interaction.edit_response(&context.http, 
        EditInteractionResponse::new().embed(embed)
    ).await {
        error!("Failed to send status response: {:?}", error);
    }
}
