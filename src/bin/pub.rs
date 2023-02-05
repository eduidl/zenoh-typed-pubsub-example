use std::{thread::sleep, time::Duration};

use zenoh::prelude::r#async::*;
use zenoh_typed_pubsub::{Nest, TestMessage};

#[tokio::main]
async fn main() {
    let config = Config::default();
    let session = zenoh::open(config).res().await.unwrap();

    let publisher = session
        .declare_publisher("test_message")
        .res()
        .await
        .unwrap();

    let mut count = 0;
    loop {
        let msg = TestMessage {
            a: count,
            b: "hello world".into(),
            c: vec![20, 34, 2],
            d: (-1, 25),
            e: Nest {
                f: "hoge".into(),
                g: 244,
            },
        };

        println!("[pub] {:?}", &msg);
        let serialized = bincode::serialize(&msg).unwrap();
        publisher.put(serialized).res().await.unwrap();

        count += 1;
        sleep(Duration::from_secs(1));
    }
}
