use layla_log::*;

#[test]
fn double_init() {
    default_init("./logs/");
    default_init("./logs/");
    error!("error");
    warn!("warn");
    info!("info");
    debug!("debug");
    trace!("trace");
}