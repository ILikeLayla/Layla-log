use layla_log::*;

#[cfg(not(feature = "async"))]
#[test]
fn write_in() {
    clean_log();
    init(Setting {
        time_zone: 1,
        ..Default::default()
    });
    
    error!("This is an error message");
    warn!("This is a warning message");
    info!("This is an info message");
    debug!("This is a debug message");
    trace!("This is a trace message");
}

#[cfg(feature = "async")]
#[tokio::test]
async fn write_in() {
    clean_log().await;
    init(Setting {
        time_zone: 1,
        ..Default::default()
    }).await;

    error!("This is an error message");
    warn!("This is a warning message");
    info!("This is an info message");
    debug!("This is a debug message");
    trace!("This is a trace message");
}
