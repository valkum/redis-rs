use super::comment::Comment;
use super::CodeGenerator;

pub(crate) fn append_constant_docs(doc: &str, generator: &mut CodeGenerator) {
    let doc_comment = Comment(doc.lines().map(ToOwned::to_owned).collect::<Vec<_>>());
    doc_comment.append_with_indent(generator.depth, generator.buf);
}

pub const COMMAND_TRAIT_DOCS: &str = r#"Implements common redis commands for connection like objects.  This
allows you to send commands straight to a connection or client.  It
is also implemented for redis results of clients which makes for
very convenient access in some basic cases.

This allows you to use nicer syntax for some common operations.
For instance this code:

```rust,no_run
# fn do_something() -> redis::RedisResult<()> {
let client = redis::Client::open("redis://127.0.0.1/")?;
let mut con = client.get_connection()?;
redis::cmd("SET").arg("my_key").arg(42).execute(&mut con);
assert_eq!(redis::cmd("GET").arg("my_key").query(&mut con), Ok(42));
# Ok(()) }
```

Will become this:

```rust,no_run
# fn do_something() -> redis::RedisResult<()> {
use redis::Commands;
let client = redis::Client::open("redis://127.0.0.1/")?;
let mut con = client.get_connection()?;
con.set("my_key", 42)?;
assert_eq!(con.get("my_key"), Ok(42));
# Ok(()) }
```"#;

pub const ASYNC_COMMAND_TRAIT_DOCS: &str = r#"Implements common redis commands over asynchronous connections. This
allows you to send commands straight to a connection or client.

This allows you to use nicer syntax for some common operations.
For instance this code:

```rust,no_run
use redis::AsyncCommands;
# async fn do_something() -> redis::RedisResult<()> {
let client = redis::Client::open("redis://127.0.0.1/")?;
let mut con = client.get_async_connection().await?;
redis::cmd("SET").arg("my_key").arg(42i32).query_async(&mut con).await?;
assert_eq!(redis::cmd("GET").arg("my_key").query_async(&mut con).await, Ok(42i32));
# Ok(()) }
```

Will become this:

```rust,no_run
use redis::AsyncCommands;
# async fn do_something() -> redis::RedisResult<()> {
use redis::Commands;
let client = redis::Client::open("redis://127.0.0.1/")?;
let mut con = client.get_async_connection().await?;
con.set("my_key", 42i32).await?;
assert_eq!(con.get("my_key").await, Ok(42i32));
# Ok(()) }
```"#;

pub const PIPELINE_DOCS: &str = r#"Implements common redis commands for pipelines.  Unlike the regular
commands trait, this returns the pipeline rather than a result
directly.  Other than that it works the same however."#;

pub const CLUSTER_PIPELINE_DOCS: &str = r#"Implements common redis commands for cluster pipelines.  Unlike the regular
commands trait, this returns the cluster pipeline rather than a result
directly.  Other than that it works the same however."#;
