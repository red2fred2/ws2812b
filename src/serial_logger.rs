//! Implements a logger over serial
//!
//! This allows the dev to just use the log crate instead of worrying about
//! complicated USB implementations.

use core::fmt::{Arguments, Write};
use log::{warn, Level, LevelFilter, Log};

use crate::hardware::Hardware;

static mut FAILED_INIT: bool = false;
static mut LOGGER: Option<SerialLogger> = None;

/// Implements a logger for the log crate
///
/// This implementation is only useful with a usb manager class that supports
/// core::write_str
pub struct SerialLogger {}

impl SerialLogger {
    /// Creates a new logger
    pub fn new() -> SerialLogger {
        SerialLogger {}
    }

    /// Sets up the log interface after a logger is created
    ///
    /// If this fails, it'll just disable logging and hope everything works out.
    pub fn init(level: LevelFilter) {
        unsafe {
            LOGGER = Some(SerialLogger::new());
            let logger = LOGGER.as_ref().unwrap();

            critical_section::with(|_| {
                let result = log::set_logger_racy(logger);
                log::set_max_level_racy(level);

                // Disable logging if it fails to set up
                if result.is_err() {
                    FAILED_INIT = true;
                }
            });
        }
    }

    /// Write the trailing part of the message
    ///
    /// This part will reset color code, then add a carriage return and newline.
    fn write_affix() {
        // Skip this if hardware isn't set up yet
        let Some(hardware) = Hardware::get() else {
            return
        };
        let usb = hardware.get_usb();

        let result = usb.write_str("\x1b[0m\r\n");

        if result.is_err() {
            warn!("Failed to write log affix");
        }
    }

    /// Writes the color escape code for this log level
    fn write_coloring(level: &Level) {
        // Skip this if hardware isn't set up yet
        let Some(hardware) = Hardware::get() else {
            return
        };
        let usb = hardware.get_usb();

        let color_string = match level {
            Level::Error => "\x1b[31;1m",
            Level::Warn => "\x1b[33;1m",
            Level::Info => "\x1b[37m",
            Level::Debug => "\x1b[35m",
            Level::Trace => "\x1b[36m",
        };

        let result = usb.write_str(color_string);

        if result.is_err() {
            warn!("Failed to write log color escape");
        }
    }

    /// Write the message part of the log
    fn write_message(message: &Arguments) {
        // Skip this if hardware isn't set up yet
        let Some(hardware) = Hardware::get() else {
            return
        };
        let usb = hardware.get_usb();
        let result = usb.write_fmt(*message);

        if result.is_err() {
            warn!("Failed to write log message");
        }
    }
}

impl Default for SerialLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl Log for SerialLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        Self::write_coloring(&record.level());
        Self::write_message(record.args());
        Self::write_affix();
    }

    fn flush(&self) {}
}
