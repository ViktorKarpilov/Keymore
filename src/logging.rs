use std::fs::File;
use log;
use log::trace;
use simplelog;
use simplelog::*;

pub fn add_logging() -> Result<(), std::io::Error>{
    let logs_path = "logs.log";

    let logs_file = match File::open(logs_path) {
        Ok(file) => file,
        Err(_) => File::create(logs_path)?
    };
    
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Debug, Config::default(), logs_file),
        ]
    ).unwrap(); 
    
    trace!("Logging initialized");

    Ok(())
}