use colored::{ColoredString, Colorize};
use log::{Record, Level, Metadata, SetLoggerError, LevelFilter};

static LOGGER: ColoredStdoutLogger = ColoredStdoutLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
}

struct ColoredStdoutLogger;

impl log::Log for ColoredStdoutLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level = match record.level() {
                Level::Error => format!("{}:", "ERROR".red()).bold(),
                Level::Warn => format!("{}: ", "WARN".yellow()).bold(),
                Level::Info => format!("{}: ", "INFO".white()).bold(),
                Level::Debug => ColoredString::from(format!("{}:", "DEBUG".white())),
                Level::Trace => ColoredString::from(format!("{}:", "TRACE".white())),
            };
            println!("{level} {}", record.args());
        }
    }

    fn flush(&self) {}
}