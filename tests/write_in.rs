use layla_log::*;

#[cfg(not(feature = "async"))]
#[test]
fn write_in() {
    clean_log();

    error!("This is an error message");
    warn!("This is a warning message");
    debug!("This is a debug message");
    info!("This is an info message");
    trace!("This is a trace message");
}

#[cfg(feature = "async")]
#[tokio::test]
async fn write_in() {
    clean_log().await;

    error!("This is an error message");
    warn!("This is a warning message");
    debug!("This is a debug message");
    info!("This is an info message");
    trace!("This is a trace message");
}
