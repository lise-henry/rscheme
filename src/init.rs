use log;
use log::{LogRecord, LogLevel, LogMetadata, SetLoggerError, LogLevelFilter};
use std::env;

pub struct Args {
    verbose:bool
}

impl Args {
    fn new() -> Args {
        Args {verbose:false}
    }

    fn from_args() -> Args {
        let mut res = Args::new();
        let mut iter = env::args();
        for a in iter {
            println!("{}", a);
            match a.as_ref() {
                "--verbose" => res.verbose = {println!("verbose!");
                                              true},
                s => println!("Unrecognised args: {}", s)
            }
        }
        res
    }
}

pub struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
}

impl SimpleLogger {
    pub fn init(args:&Args) -> Result<(), SetLoggerError> {
        log::set_logger(|max_log_level| {
            if args.verbose {
                max_log_level.set(LogLevelFilter::Debug);
            } else {
                max_log_level.set(LogLevelFilter::Error);
            }
            Box::new(SimpleLogger)
        })
    }
}

pub fn init() {
    let args = Args::from_args();
    let r = SimpleLogger::init(&args);
    match r {
        Err(_) => panic!("Could not initialize logger"),
        Ok(_) => ()
    }
}
