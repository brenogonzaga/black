use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};

#[derive(Error, Debug)]
pub enum BlackBoxError {
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to acquire Mutex lock")]
    Lock,
}

pub struct BlackBox {
    log_file: Arc<Mutex<std::fs::File>>,
}

impl Clone for BlackBox {
    fn clone(&self) -> Self {
        BlackBox {
            log_file: Arc::clone(&self.log_file),
        }
    }
}

impl std::fmt::Debug for BlackBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlackBox")
            .field("log_file", &"Arc<Mutex<File>>")
            .finish()
    }
}

impl Default for BlackBox {
    fn default() -> Self {
        BlackBox::new("default.log").expect("Unable to create default logger")
    }
}

impl BlackBox {
    pub fn new(file_path: &str) -> Result<Self> {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_path)
            .with_context(|| format!("Could not open or create log file at path: {}", file_path))?;
        Ok(BlackBox {
            log_file: Arc::new(Mutex::new(file)),
        })
    }

    fn log(&self, level: &str, message: &str) -> Result<()> {
        let now = Local::now();
        let log_line = format!(
            "[{}] [{}] {}\n",
            now.format("%Y-%m-%d %H:%M:%S"),
            level,
            message
        );

        let mut file = self.log_file.lock().map_err(|_| BlackBoxError::Lock)?;
        file.write_all(log_line.as_bytes())
            .with_context(|| "Could not write to log file")?;
        Ok(())
    }

    pub fn log_debug(&self, message: &str) -> Result<()> {
        self.log("DEBUG", message)
    }

    pub fn log_info(&self, message: &str) -> Result<()> {
        self.log("INFO", message)
    }

    pub fn log_warn(&self, message: &str) -> Result<()> {
        self.log("WARN", message)
    }

    pub fn log_event(&self, message: &str) -> Result<()> {
        self.log("EVENT", message)
    }

    pub fn log_error(&self, message: &str) -> Result<()> {
        self.log("ERROR", message)
    }
}

pub trait HasLogger {
    fn logger(&self) -> &BlackBox;
}

pub trait Loggable: HasLogger {
    fn log_debug(&self, message: &str) -> Result<()> {
        self.logger().log_debug(message)
    }

    fn log_info(&self, message: &str) -> Result<()> {
        self.logger().log_info(message)
    }

    fn log_warn(&self, message: &str) -> Result<()> {
        self.logger().log_warn(message)
    }

    fn log_event(&self, message: &str) -> Result<()> {
        self.logger().log_event(message)
    }

    fn log_error(&self, message: &str) -> Result<()> {
        self.logger().log_error(message)
    }
}

impl<T: HasLogger> Loggable for T {}

pub use anyhow::{Context, Result};
pub use chrono::Local;
pub use thiserror::Error;
