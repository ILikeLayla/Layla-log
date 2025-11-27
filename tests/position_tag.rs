use layla_log::*;

#[cfg(not(feature = "async"))]
#[test]
fn write_in() {
    clean_log!();
    log_set! {
        display_path: true,
        display_scope: true
    };

    error!("This is an error message");
    warn!("This is a warning message");
    info!("This is an info message");
    debug!("This is a debug message");
    trace!("This is a trace message");

    log_set! {
        display_path: false,
        display_scope: true
    };

    error!("This is an error message");
    warn!("This is a warning message");
    info!("This is an info message");
    debug!("This is a debug message");
    trace!("This is a trace message");

    log_set! {
        display_path: true,
        display_scope: false
    };

    error!("This is an error message");
    warn!("This is a warning message");
    info!("This is an info message");
    debug!("This is a debug message");
    trace!("This is a trace message");

    log_set! {
        display_path: false,
        display_scope: false
    };

    error!("This is an error message");
    warn!("This is a warning message");
    info!("This is an info message");
    debug!("This is a debug message");
    trace!("This is a trace message");
}

#[cfg(feature = "async")]
#[tokio::test]
async fn write_in() {
    clean_log!();

    log_set! {
        display_path: true,
        display_scope: true
    };

    error!("This is an error message");
    warn!("This is a warning message");
    info!("This is an info message");
    debug!("This is a debug message");
    trace!("This is a trace message");

    log_set! {
        display_path: false,
        display_scope: true
    };

    error!("This is an error message");
    warn!("This is a warning message");
    info!("This is an info message");
    debug!("This is a debug message");
    trace!("This is a trace message");

    log_set! {
        display_path: true,
        display_scope: false
    };

    error!("This is an error message");
    warn!("This is a warning message");
    info!("This is an info message");
    debug!("This is a debug message");
    trace!("This is a trace message");

    log_set! {
        display_path: false,
        display_scope: false
    };

    error!("This is an error message");
    warn!("This is a warning message");
    info!("This is an info message");
    debug!("This is a debug message");
    trace!("This is a trace message");
}
