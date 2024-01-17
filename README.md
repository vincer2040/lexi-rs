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
  assert_eq!(set_res, lexi::LexiType::Simple(SimpleString::Ok));

  let get_res = client.get("key").await?;
  assert_eq!(get_res, lexi::LexiType::BulkString("value".to_owned()));

  let del_res = client.del("key").await?;
  assert_eq!(del_res, lexi::LexiType::Simple(SimpleString::Ok));

  Ok(())
}
```

#### stack

```rs
let push_res = client.push("vince").await?;
assert_eq!(push_res, LexiType::Simple(SimpleString::Ok));

let pop_res = client.pop().await?;
assert_eq!(pop_res, LexiType::BulkString("vince".to_owned());
```

#### queue
```rs
let enque_res = client.enque("vince").await?;
assert_eq!(enque_res, LexiType::Simple(SimpleString::Ok));

let deque_res = client.deque().await?;
assert_eq!(deque_res, LexiType::BulkString("vince".to_owned());
```
