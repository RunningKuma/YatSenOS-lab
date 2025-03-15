use log::{Metadata, Record};

pub fn init() {
    static LOGGER: Logger = Logger;
    log::set_logger(&LOGGER).unwrap();

    // FIXME: Configure the logger
    log::set_max_level(log::LevelFilter::Trace);
    info!("Logger Initialized.");
}

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        // FIXME: Implement the logger with serial output
        if self.enabled(record.metadata()){
            match record.level() {
                log::Level::Error => 
                    println!("\x1B[31m[ERROR] - at {}，Line {}: {} \x1B[0m",record.file_static().unwrap(),record.line().unwrap(),record.args()), // 红色
                
                log::Level::Warn => 
                    println!("\x1B[33m[WARN] - at {}，Line {}: {} \x1B[0m",record.file_static().unwrap(),record.line().unwrap(),record.args()),
                
                log::Level::Info => 
                    println!("\x1B[32m[INFO]\x1B[0m- at {}，Line {} :{}",record.file_static().unwrap(),record.line().unwrap(),record.args()), 
                
                log::Level::Debug => 
                    println!("\x1B[36m[DEBUG] - at {}，Line {}: {} \x1B[0m",record.file_static().unwrap(),record.line().unwrap(),record.args()),
                
                log::Level::Trace => 
                    println!("\x1B[90m[TRACE] - at {}，Line {}: {} \x1B[0m",record.file_static().unwrap(),record.line().unwrap(),record.args()),
            }
        }
    }

    fn flush(&self) {}
}
