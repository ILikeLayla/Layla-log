use layla_log::*;

#[cfg(not(feature = "async"))]
#[test]
fn universal_log() {
    clean_log();

    log!(LogLevel::Trace, "Hello, {}!", "world");
}

#[cfg(feature = "async")]
#[tokio::test]
async fn universal_log() {
    clean_log().await;

    log!(LogLevel::Trace, "Hello, {}!", "world");
}
