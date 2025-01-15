use layla_log::*;

#[cfg(not(feature = "async"))]
#[test]
fn test_disable() {

    clean_log();
    
    // Disable all logging
    disable_log();

    // Test that logging is disabled
    info!("This should not be logged");
    warn!("This should not be logged");
    error!("This should not be logged");
    debug!("This should not be logged");
    trace!("This should not be logged");

    // Enable logging for a specific module
    enable_log();
    info!("This should be logged");
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_disable() {
    clean_log().await;
    
    // Disable all logging
    disable_log().await;

    // Test that logging is disabled
    info!("This should not be logged");
    warn!("This should not be logged");
    error!("This should not be logged");
    debug!("This should not be logged");
    trace!("This should not be logged");

    // Enable logging for a specific module
    enable_log().await;
    info!("This should be logged");
}