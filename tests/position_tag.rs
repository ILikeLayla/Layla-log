use layla_log::*;

#[cfg(not(feature = "async"))]
#[test]
fn write_in() {
    clean_log();
    log_init(LogSetting {
        display_path: true,
        display_scoop: true,
        ..Default::default()
    });
    
    error!("This is an error message");
    warn!("This is a warning message");
    info!("This is an info message");
    debug!("This is a debug message");
    trace!("This is a trace message");

    log_set(LogSetting {
        display_path: false,
        display_scoop: true,
        ..Default::default()
    });

    error!("This is an error message");
    warn!("This is a warning message");
    info!("This is an info message");
    debug!("This is a debug message");
    trace!("This is a trace message");

    log_set(LogSetting {
        display_path: true,
        display_scoop: false,
        ..Default::default()
    });

    error!("This is an error message");
    warn!("This is a warning message");
    info!("This is an info message");
    debug!("This is a debug message");
    trace!("This is a trace message");

    log_set(LogSetting {
        display_path: false,
        display_scoop: false,
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
