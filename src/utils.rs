use sysinfo::{ComponentExt, SystemExt};

/// Configuration options for formatting time units
#[derive(Debug, Clone, Copy)]
pub struct TimeFormatOptions {
    pub include_days: bool,
    pub include_hours: bool,
    pub include_minutes: bool,
    pub include_seconds: bool,
    pub short_units: bool,
    pub max_units: usize,
}

impl Default for TimeFormatOptions {
    fn default() -> Self {
        Self {
            include_days: true,
            include_hours: true,
            include_minutes: true,
            include_seconds: true,
            short_units: false,
            max_units: 4,
        }
    }
}

/// Temperature display options
#[derive(Debug, Clone, Copy)]
pub struct TemperatureOptions {
    pub precision: usize,
    pub include_all: bool,
    pub use_fahrenheit: bool,
    pub include_labels: bool,
}

impl Default for TemperatureOptions {
    fn default() -> Self {
        Self {
            precision: 1,
            include_all: false,
            use_fahrenheit: false,
            include_labels: true,
        }
    }
}

/// Size display options
#[derive(Debug, Clone, Copy)]
pub enum SizeUnit {
    Auto,
    Bytes,
    KB,
    MB,
    GB,
    TB,
}

/// Progress bar configuration
#[derive(Debug, Clone)]
pub struct ProgressBarOptions {
    pub width: usize,
    pub fill_char: char,
    pub empty_char: char,
    pub show_percentage: bool,
    pub show_values: bool,
    pub value_precision: usize,
    pub percentage_precision: usize,
    pub unit: String,
}

impl Default for ProgressBarOptions {
    fn default() -> Self {
        Self {
            width: 20,
            fill_char: '█',
            empty_char: '░',
            show_percentage: true,
            show_values: true,
            value_precision: 1,
            percentage_precision: 1,
            unit: "GB".to_string(),
        }
    }
}

/// Format uptime with configurable display options.
pub fn format_uptime(seconds: u64, options: Option<TimeFormatOptions>) -> String {
    let opts = options.unwrap_or_default();
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    
    let mut parts = Vec::new();
    
    if opts.include_days && days > 0 {
        parts.push(format_time_part(days, if opts.short_units { "d" } else { "day" }));
    }
    
    if opts.include_hours && hours > 0 {
        parts.push(format_time_part(hours, if opts.short_units { "h" } else { "hour" }));
    }
    
    if opts.include_minutes && (minutes > 0 || parts.is_empty()) {
        parts.push(format_time_part(minutes, if opts.short_units { "m" } else { "minute" }));
    }
    
    if opts.include_seconds && (seconds > 0 || parts.is_empty()) {
        parts.push(format_time_part(seconds, if opts.short_units { "s" } else { "second" }));
    }
    
    // Limit to max_units if specified
    if opts.max_units > 0 && opts.max_units < parts.len() {
        parts.truncate(opts.max_units);
    }
    
    parts.join(", ")
}

fn format_time_part(value: u64, unit: &str) -> String {
    if value == 1 {
        format!("1 {}", unit)
    } else {
        format!("{} {}s", value, unit)
    }
}

/// Convert Celsius to Fahrenheit
fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    celsius * 1.8 + 32.0
}

/// Formats temperature information from system sensors.
pub fn format_temperature(system: &mut sysinfo::System, options: Option<TemperatureOptions>) -> String {
    let opts = options.unwrap_or_default();
    let components = system.components();
    
    if components.is_empty() {
        return "No temperature sensors available".to_string();
    }
    
    if opts.include_all {
        // Show all sensors
        let mut temps = Vec::new();
        for component in components {
            let temp = if opts.use_fahrenheit {
                celsius_to_fahrenheit(component.temperature())
            } else {
                component.temperature()
            };
            
            let unit = if opts.use_fahrenheit { "°F" } else { "°C" };
            
            if opts.include_labels {
                temps.push(format!(
                    "{}: `{:.precision$}{}` ", 
                    component.label(), 
                    temp,
                    unit,
                    precision = opts.precision
                ));
            } else {
                temps.push(format!(
                    "`{:.precision$}{}`", 
                    temp,
                    unit,
                    precision = opts.precision
                ));
            }
        }
        temps.join("\n")
    } else {
        // Show only the highest temperature
        let max_temp = components.iter()
            .map(|c| c.temperature())
            .fold(0.0_f32, |acc: f32, temp: f32| acc.max(temp));
        
        let displayed_temp = if opts.use_fahrenheit {
            celsius_to_fahrenheit(max_temp)
        } else {
            max_temp
        };
        
        let unit = if opts.use_fahrenheit { "°F" } else { "°C" };
        
        format!("Max: `{:.precision$}{}`", displayed_temp, unit, precision = opts.precision)
    }
}

/// Create a text-based progress bar.
pub fn create_progress_bar(value: f64, max: f64, options: Option<ProgressBarOptions>) -> String {
    let opts = options.unwrap_or_default();
    let percentage = (value / max * 100.0).min(100.0);
    let filled_width = ((value / max) * opts.width as f64).round() as usize;
    let empty_width = opts.width.saturating_sub(filled_width);
    
    let filled = opts.fill_char.to_string().repeat(filled_width);
    let empty = opts.empty_char.to_string().repeat(empty_width);
    
    let mut result = format!("`{}{}`", filled, empty);
    
    if opts.show_values {
        result = format!(
            "{} {:.v_prec$}/{:.v_prec$} {}",
            result,
            value,
            max,
            opts.unit,
            v_prec = opts.value_precision
        );
    }
    
    if opts.show_percentage {
        result = format!(
            "{} ({:.p_prec$}%)",
            result,
            percentage,
            p_prec = opts.percentage_precision
        );
    }
    
    result
}

/// Format file size with appropriate units.
pub fn format_file_size(size: u64, unit: Option<SizeUnit>, precision: Option<usize>) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;
    
    let prec = precision.unwrap_or(2);
    
    match unit.unwrap_or(SizeUnit::Auto) {
        SizeUnit::Bytes => format!("{} bytes", size),
        SizeUnit::KB => format!("{:.prec$} KB", size as f64 / KB as f64, prec = prec),
        SizeUnit::MB => format!("{:.prec$} MB", size as f64 / MB as f64, prec = prec),
        SizeUnit::GB => format!("{:.prec$} GB", size as f64 / GB as f64, prec = prec),
        SizeUnit::TB => format!("{:.prec$} TB", size as f64 / TB as f64, prec = prec),
        SizeUnit::Auto => {
            if size >= TB {
                format!("{:.prec$} TB", size as f64 / TB as f64, prec = prec)
            } else if size >= GB {
                format!("{:.prec$} GB", size as f64 / GB as f64, prec = prec)
            } else if size >= MB {
                format!("{:.prec$} MB", size as f64 / MB as f64, prec = prec)
            } else if size >= KB {
                format!("{:.prec$} KB", size as f64 / KB as f64, prec = prec)
            } else {
                format!("{} bytes", size)
            }
        }
    }
}

/// Options for duration formatting
#[derive(Debug, Clone, Copy)]
pub enum DurationFormat {
    /// Automatic selection based on duration length
    Auto,
    /// Always show in milliseconds
    Milliseconds,
    /// Always show in seconds
    Seconds, 
    /// Always show in minutes and seconds
    MinutesSeconds,
    /// Always show in hours, minutes, and seconds
    HoursMinutesSeconds,
}

/// Format time duration with options for precision and format.
pub fn format_duration(ms: u128, format: Option<DurationFormat>, precision: Option<usize>) -> String {
    let prec = precision.unwrap_or(1);
    
    match format.unwrap_or(DurationFormat::Auto) {
        DurationFormat::Milliseconds => {
            format!("{}ms", ms)
        },
        DurationFormat::Seconds => {
            format!("{:.prec$}s", ms as f64 / 1000.0, prec = prec)
        },
        DurationFormat::MinutesSeconds => {
            let seconds = ms / 1000;
            let minutes = seconds / 60;
            let seconds = seconds % 60;
            format!("{}m {}s", minutes, seconds)
        },
        DurationFormat::HoursMinutesSeconds => {
            let seconds = ms / 1000;
            let minutes = seconds / 60;
            let hours = minutes / 60;
            let minutes = minutes % 60;
            let seconds = seconds % 60;
            format!("{}h {}m {}s", hours, minutes, seconds)
        },
        DurationFormat::Auto => {
            if ms < 1 {
                "< 1ms".to_string()
            } else if ms < 1000 {
                format!("{}ms", ms)
            } else if ms < 60000 {
                format!("{:.prec$}s", ms as f64 / 1000.0, prec = prec)
            } else if ms < 3600000 {
                let seconds = ms / 1000;
                let minutes = seconds / 60;
                let seconds = seconds % 60;
                format!("{}m {}s", minutes, seconds)
            } else {
                let seconds = ms / 1000;
                let minutes = seconds / 60;
                let hours = minutes / 60;
                let minutes = minutes % 60;
                let seconds = seconds % 60;
                format!("{}h {}m {}s", hours, minutes, seconds)
            }
        }
    }
}

/// Extracts code from code blocks in a message.
/// Supports both ```code``` and `code` formats.
pub fn extract_code_from_message(content: &str) -> Option<String> {
    // Check for triple backtick code blocks
    if let Some(start) = content.find("```") {
        let after_start = &content[start + 3..];
        
        // Find language identifier if any
        let content_start = if let Some(newline) = after_start.find('\n') {
            // Skip language identifier
            start + 3 + newline + 1
        } else {
            // No language identifier
            start + 3
        };
        
        if let Some(end) = content[content_start..].find("```") {
            return Some(content[content_start..content_start + end].to_string());
        }
    }
    
    // Check for single backtick inline code
    if let Some(start) = content.find('`') {
        if let Some(end) = content[start + 1..].find('`') {
            return Some(content[start + 1..start + 1 + end].to_string());
        }
    }
    
    None
} 
