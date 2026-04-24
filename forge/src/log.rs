mod util;

use sys::log::Level as SysLevel;
use sys::log::{forge_log, forge_log_getLevel};

pub mod prelude {
    pub use log::{debug, error, info, trace, warn};
}

use core::fmt::Write;
use util::FixedCStringWriter;

struct ForgeLogger;
static LOGGER: ForgeLogger = ForgeLogger;

const LOG_FORMAT: &[u8; 3] = b"%s\0";

pub fn init() -> Result<(), log::SetLoggerError> {
    log::set_logger(&LOGGER)?;
    log::set_max_level(log::LevelFilter::Trace);
    Ok(())
}

impl log::Log for ForgeLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        let sys_level = Self::to_sys_level(metadata.level());
        let current_level = unsafe { forge_log_getLevel() };
        sys_level as u8 >= current_level as u8
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let sys_level = Self::to_sys_level(record.level());
        let mut message = [0u8; 512];
        let mut writer = FixedCStringWriter::new(&mut message);

        let _ = write!(writer, "{}", record.args());

        unsafe {
            forge_log(sys_level, LOG_FORMAT.as_ptr().cast(), writer.as_c_str_ptr());
        }
    }

    fn flush(&self) {}
}

impl ForgeLogger {
    fn to_sys_level(level: log::Level) -> SysLevel {
        match level {
            log::Level::Error => SysLevel::Error,
            log::Level::Warn => SysLevel::Warn,
            log::Level::Info => SysLevel::Info,
            log::Level::Debug => SysLevel::Debug,
            log::Level::Trace => SysLevel::Debug,
        }
    }
}
