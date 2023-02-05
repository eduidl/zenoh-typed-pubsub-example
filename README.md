# zenoh-typed-pubsub

## Subscribe

```
$ cargo run --bin sub
[pub] TestMessage { a: 0, b: "hello world", c: [20, 34, 2], d: (-1, 25), e: Nest { f: "hoge", g: 244 } }
[pub] TestMessage { a: 1, b: "hello world", c: [20, 34, 2], d: (-1, 25), e: Nest { f: "hoge", g: 244 } }
[pub] TestMessage { a: 2, b: "hello world", c: [20, 34, 2], d: (-1, 25), e: Nest { f: "hoge", g: 244 } }
[pub] TestMessage { a: 3, b: "hello world", c: [20, 34, 2], d: (-1, 25), e: Nest { f: "hoge", g: 244 } }
[pub] TestMessage { a: 4, b: "hello world", c: [20, 34, 2], d: (-1, 25), e: Nest { f: "hoge", g: 244 } }
[pub] TestMessage { a: 5, b: "hello world", c: [20, 34, 2], d: (-1, 25), e: Nest { f: "hoge", g: 244 } }
:
```

## Publish

Another terminal

```
$ cargo run --bin pub
[sub] TestMessage { a: 0, b: "hello world", c: [20, 34, 2], d: (-1, 25), e: Nest { f: "hoge", g: 244 } }
[sub] TestMessage { a: 1, b: "hello world", c: [20, 34, 2], d: (-1, 25), e: Nest { f: "hoge", g: 244 } }
[sub] TestMessage { a: 2, b: "hello world", c: [20, 34, 2], d: (-1, 25), e: Nest { f: "hoge", g: 244 } }
[sub] TestMessage { a: 3, b: "hello world", c: [20, 34, 2], d: (-1, 25), e: Nest { f: "hoge", g: 244 } }
[sub] TestMessage { a: 4, b: "hello world", c: [20, 34, 2], d: (-1, 25), e: Nest { f: "hoge", g: 244 } }
:
```
