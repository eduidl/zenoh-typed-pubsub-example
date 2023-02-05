use zenoh::{buffers::reader::HasReader, prelude::r#async::*};
use zenoh_typed_pubsub::TestMessage;

#[tokio::main]
async fn main() {
    let config = Config::default();
    let session = zenoh::open(config).res().await.unwrap();

    let subscriber = session
        .declare_subscriber("test_message")
        .res()
        .await
        .unwrap();

    loop {
        let msg = subscriber.recv_async().await.unwrap();
        let deserialized: TestMessage = bincode::deserialize_from(msg.payload.reader()).unwrap();
        println!("[sub] {:?}", &deserialized);
    }
}
