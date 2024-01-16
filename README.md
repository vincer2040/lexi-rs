# lexi-rs

a rust client for [lexidb](https://github.com/vincer2040/lexidb), an in memory data structure database.

## Getting started

### install this package

```console
$ cargo add lexi-rs
```

### Basic Usage

currently, values that are set must implement Into<LexiType>. Data types that implement
this trait include:

1. &str
2. String,
3. i8, u8, i16, u16, i32, u32, i64, f32, f64

```rs

use lexi;
use anyhow;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let mut client = Client::new(<address>)?;
  client.connect.await?;

  let set_res = client.set("key", "value").await?;
  assert_eq!(set_res, lexi::LexiType::Simple("OK".to_owned()));

  let get_res = client.get("key").await?;
  assert_eq!(get_res, lexi::LexiType::BulkString("value".to_owned()));

  let del_res = client.del("key").await?;
  assert_eq!(del_res, lexi::LexiType::Simple("OK".to_owned()));

  Ok(())
}
```

#### clusters

```rs
let new_cluster_res = client.cluster_new("name").await?;
assert_eq!(new_cluster_res, lexi::LexiType::Simple("OK".to_owned()));

let cluster_set_res = client.cluster_set("name", "key", "value").await?;
assert_eq!(cluster_set_res, lexi::LexiType::Simple("OK".to_owned()));

let cluster_get_res = client.cluster_get("name", "key").await?;
assert_eq!(cluster_get_res, lexi::LexiType::BulkString("value".to_owned()));

let cluster_del_res = client.cluster_del("name", "key").await?;
assert_eq!(cluster_del_res, lexi::LexiType::Simple("OK".to_owned()));

let cluster_drop_res = client.cluster_drop("name").await?;
assert_eq!(cluster_drop_res, lexi::LexiType::Simple("OK".to_owned()));
```


#### stack

```rs
let push_res = client.push("vince").await?;
assert_eq!(push_res, LexiType::Simple("OK".to_owned()));

let pop_res = client.pop().await?;
assert_eq!(pop_res, LexiType::BulkString("vince".to_owned());
```

#### queue
```rs
let enque_res = client.enque("vince").await?;
assert_eq!(enque_res, LexiType::Simple("OK".to_owned()));

let deque_res = client.deque().await?;
assert_eq!(deque_res, LexiType::BulkString("vince".to_owned());
```
