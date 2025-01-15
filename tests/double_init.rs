use layla_log::*;

#[cfg(not(feature = "async"))]
#[test]
fn double_init() {
    init(Setting {
        dir_path: "./logs".to_string(),
        ..Default::default()
    });

    init(Setting {
        dir_path: "./logs".to_string(),
        ..Default::default()
    });
    
    error!("error");
    warn!("warn");
    info!("info");
    debug!("debug");
    trace!("trace");
}

#[cfg(feature = "async")]
#[tokio::test]
async fn double_init() {
    init(Setting {
        dir_path: "./logs".to_string(),
        ..Default::default()
    }).await;

    init(Setting {
        dir_path: "./logs".to_string(),
        ..Default::default()
    }).await;
    
    error!("error");
    warn!("warn");
    info!("info");
    debug!("debug");
    trace!("trace");
}