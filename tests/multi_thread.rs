#![feature(test)]

extern crate test;
use layla_log::*;

#[cfg(not(feature = "async"))]
#[test]
fn write_by_multi_threads() {
    clean_log();
    let mut handles = vec![];
    for i in 0..1219 {
        let handle = std::thread::spawn(move || {
            info!("thread {} started", i);
            std::thread::sleep(std::time::Duration::from_millis(1));
            info!("thread {} finished", i);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

#[cfg(feature = "async")]
#[tokio::test]
async fn write_by_multi_threads() {
    clean_log().await;
    let mut handles = vec![];
    for i in 0..1219 {
        let handle = tokio::spawn(async move {
            info!("thread {} started", i);
            tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
            info!("thread {} finished", i);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
}
