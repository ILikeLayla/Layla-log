use layla_log::*;

#[cfg(not(feature = "async"))]
#[test]
fn f() {
    clean_log();

    error!("This is an error message");
}

#[cfg(feature = "async")]
#[tokio::test]
async fn f() {
    clean_log().await;

    error!("This is an error message");
}
