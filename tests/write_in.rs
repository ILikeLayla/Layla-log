use layla_log::*;

#[test]
fn write_in() {
    // optional
    // default_init("path/to/log");
    
    error!("This is an error message");
    warn!("This is a warning message");
    debug!("This is a debug message");
    info!("This is an info message");
    trace!("This is a trace message");
}
