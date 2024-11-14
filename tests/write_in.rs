use layla_log::*;

#[test]
fn write_in() {
    // default_init("path/to/log");
    init("./logs/", 1219, LogLevel::Trace, LogLevel::Debug, 0, true, true);
    error!("This is an error message");
    warn!("This is a warning message");
    debug!("This is a debug message");
    info!("This is an info message");
    trace!("This is a trace message");
    // clean_log();
    // error!("error");
    // warn!("warn");
    // info!("info");
    // debug!("debug");
    // trace!("trace");
}