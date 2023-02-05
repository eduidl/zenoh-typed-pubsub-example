use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestMessage {
    pub a: u32,
    pub b: String,
    pub c: Vec<u8>,
    pub d: (i8, i64),
    pub e: Nest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Nest {
    pub f: String,
    pub g: u8,
}

