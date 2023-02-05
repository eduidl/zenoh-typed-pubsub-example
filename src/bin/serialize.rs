use zenoh_typed_pubsub::{Nest, TestMessage};

fn main() {
    let msg = TestMessage {
        a: 1,
        b: "hello world".into(),
        c: vec![20, 34, 2],
        d: (-1, 25),
        e: Nest {
            f: "hoge".into(),
            g: 244,
        },
    };

    let data = bincode::serialize(&msg).unwrap();
    println!("serialized: {:?}", &data);
}
