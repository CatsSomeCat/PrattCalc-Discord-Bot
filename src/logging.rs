use std::env;
use std::io::Write;
use chrono::Local;
use env_logger::Builder;
use log::{LevelFilter};

/// Sets up an enhanced logger with custom formatting and error logs directed to stderr
pub fn setup_logger() {
    // Create a custom builder from environment
    let mut builder = Builder::from_env(env_logger::Env::default());
    
    // Direct log output based on level
    builder.format(|_buf, record| {
        let level = record.level();
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let target = record.target();
        let args = record.args();
        
        // Only log errors to stderr, other levels to stdout
        match level {
            log::Level::Error | log::Level::Warn => {
                let stderr = std::io::stderr();
                let mut stderr_lock = stderr.lock();
                writeln!(
                    stderr_lock,
                    "[{} {:5} {}] {}",
                    timestamp,
                    level,
                    target,
                    args
                )
            },
            _ => {
                let stdout = std::io::stdout();
                let mut stdout_lock = stdout.lock();
                writeln!(
                    stdout_lock,
                    "[{} {:5} {}] {}",
                    timestamp,
                    level,
                    target,
                    args
                )
            }
        }
    });
    
    // Set the default log level from env or fallback to info
    let log_level = match env::var("RUST_LOG") {
        Ok(level) => match level.to_lowercase().as_str() {
            "trace" => LevelFilter::Trace,
            "debug" => LevelFilter::Debug,
            "info" => LevelFilter::Info,
            "warn" => LevelFilter::Warn,
            "error" => LevelFilter::Error,
            _ => LevelFilter::Info
        },
        Err(_) => LevelFilter::Info
    };
    
    builder.filter_level(log_level);
    
    // Initialize the logger
    builder.init();
} 
