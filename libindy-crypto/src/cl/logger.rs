extern crate env_logger;
extern crate log;
extern crate libc;

use self::env_logger::Builder;
use self::log::LevelFilter;
use std::env;
use std::io::Write;
use crate::log::{Record, Metadata};

use crate::errors::IndyCryptoError;

use self::libc::{c_void, c_char};
use std::ffi::CString;
use std::ptr;

pub type EnabledCB = extern fn(context: *const c_void,
                               level: u32,
                               target: *const c_char) -> bool;

pub type LogCB = extern fn(context: *const c_void,
                           level: u32,
                           target: *const c_char,
                           message: *const c_char,
                           module_path: *const c_char,
                           file: *const c_char,
                           line: u32);

pub type FlushCB = extern fn(context: *const c_void);

pub struct IndyCryptoLogger {
    context: *const c_void,
    enabled: Option<EnabledCB>,
    log: LogCB,
    flush: Option<FlushCB>,
}

impl IndyCryptoLogger {
    fn new(context: *const c_void, enabled: Option<EnabledCB>, log: LogCB, flush: Option<FlushCB>) -> Self {
        IndyCryptoLogger { context, enabled, log, flush }
    }
}

impl log::Log for IndyCryptoLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        if let Some(enabled_cb) = self.enabled {
            let level = metadata.level() as u32;
            let target = CString::new(metadata.target()).unwrap();

            enabled_cb(self.context,
                       level,
                       target.as_ptr(),
            )
        } else { true }
    }

    fn log(&self, record: &Record) {
        let log_cb = self.log;

        let level = record.level() as u32;
        let target = CString::new(record.target()).unwrap();
        let message = CString::new(record.args().to_string()).unwrap();

        let module_path = record.module_path().map(|a| CString::new(a).unwrap());
        let file = record.file().map(|a| CString::new(a).unwrap());
        let line = record.line().unwrap_or(0);

        log_cb(self.context,
               level,
               target.as_ptr(),
               message.as_ptr(),
               module_path.as_ref().map(|p| p.as_ptr()).unwrap_or(ptr::null()),
               file.as_ref().map(|p| p.as_ptr()).unwrap_or(ptr::null()),
               line,
        )
    }

    fn flush(&self) {
        if let Some(flush) = self.flush {
            flush(self.context)
        }
    }
}

unsafe impl Sync for IndyCryptoLogger {}

unsafe impl Send for IndyCryptoLogger {}

impl IndyCryptoLogger {
    pub fn init(context: *const c_void, enabled: Option<EnabledCB>, log: LogCB, flush: Option<FlushCB>) -> Result<(), IndyCryptoError> {
        let logger = IndyCryptoLogger::new(context, enabled, log, flush);

        log::set_boxed_logger(Box::new(logger))?;
        log::set_max_level(LevelFilter::Trace);

        Ok(())
    }
}

pub struct IndyCryptoDefaultLogger;

impl IndyCryptoDefaultLogger {
    pub fn init(pattern: Option<String>) -> Result<(), IndyCryptoError> {
        let pattern = pattern.or(env::var("RUST_LOG").ok());

        Builder::new()
            .format(|buf, record| writeln!(buf, "{:>5}|{:<30}|{:>35}:{:<4}| {}", record.level(), record.target(), record.file().get_or_insert(""), record.line().get_or_insert(0), record.args()))
            .filter(None, LevelFilter::Off)
            .parse(pattern.as_ref().map(String::as_str).unwrap_or(""))
            .try_init()?;

        Ok(())
    }
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! secret {
    ($val:expr) => {{ $val }};
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! secret {
    ($val:expr) => {{ "_" }};
}