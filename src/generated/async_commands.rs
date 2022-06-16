use crate::types::{FromRedisValue, NumericBehavior, RedisResult, ToRedisArgs, RedisWrite, Expiry};
use crate::connection::{Connection, ConnectionLike, Msg};
use crate::cmd::{Cmd, Iter};

/// Implements common redis commands over asynchronous connections. This
/// allows you to send commands straight to a connection or client.
/// 
/// This allows you to use nicer syntax for some common operations.
/// For instance this code:
/// 
/// ```rust,no_run
/// use redis::AsyncCommands;
/// # async fn do_something() -> redis::RedisResult<()> {
/// let client = redis::Client::open("redis://127.0.0.1/")?;
/// let mut con = client.get_async_connection().await?;
/// redis::cmd("SET").arg("my_key").arg(42i32).query_async(&mut con).await?;
/// assert_eq!(redis::cmd("GET").arg("my_key").query_async(&mut con).await, Ok(42i32));
/// # Ok(()) }
/// ```
/// 
/// Will become this:
/// 
/// ```rust,no_run
/// use redis::AsyncCommands;
/// # async fn do_something() -> redis::RedisResult<()> {
/// use redis::Commands;
/// let client = redis::Client::open("redis://127.0.0.1/")?;
/// let mut con = client.get_async_connection().await?;
/// con.set("my_key", 42i32).await?;
/// assert_eq!(con.get("my_key").await, Ok(42i32));
/// # Ok(()) }
/// ```
#[cfg(feature = "aio")]
pub trait AsyncCommands : crate::aio::ConnectionLike + Send + Sized {
    /// COPY
    /// 
    /// Copy a key
    /// 
    /// Since: Redis 6.2.0
    /// Group: Generic
    /// Complexity: O(N) worst case for collections, where N is the number of nested items. O(1) for string values.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// ACL Categories:
    /// * @keyspace
    /// * @write
    /// * @slow
    fn copy<'a, K0: ToRedisArgs + Send + Sync + 'a, K1: ToRedisArgs + Send + Sync + 'a>(source: K0, destination: K1) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("COPY");
            rv.arg(source);
            rv.arg(destination);
            rv.query_async(self).await
        })
    }

    /// DEL
    /// 
    /// Delete a key
    /// 
    /// Since: Redis 1.0.0
    /// Group: Generic
    /// Complexity: O(N) where N is the number of keys that will be removed. When a key to remove holds a value other than a string, the individual complexity for this key is O(M) where M is the number of elements in the list, set, sorted set or hash. Removing a single key that holds a string value is O(1).
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// ACL Categories:
    /// * @keyspace
    /// * @write
    /// * @slow
    fn del<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: &'a [K0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("DEL");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// DUMP
    /// 
    /// Return a serialized version of the value stored at the specified key.
    /// 
    /// Since: Redis 2.6.0
    /// Group: Generic
    /// Complexity: O(1) to access the key and additional O(N*M) to serialize it, where N is the number of Redis objects composing the value and M their average size. For small string values the time complexity is thus O(1)+O(1*M) where M is small, so simply O(1).
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @keyspace
    /// * @read
    /// * @slow
    fn dump<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("DUMP");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// EXISTS
    /// 
    /// Determine if a key exists
    /// 
    /// Since: Redis 1.0.0
    /// Group: Generic
    /// Complexity: O(N) where N is the number of keys to check.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @keyspace
    /// * @read
    /// * @fast
    fn exists<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: &'a [K0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("EXISTS");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// EXPIRE
    /// 
    /// Set a key's time to live in seconds
    /// 
    /// Since: Redis 1.0.0
    /// Group: Generic
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @keyspace
    /// * @write
    /// * @fast
    fn expire<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0, seconds: i64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("EXPIRE");
            rv.arg(key);
            rv.arg(seconds);
            rv.query_async(self).await
        })
    }

    /// EXPIREAT
    /// 
    /// Set the expiration for a key as a UNIX timestamp
    /// 
    /// Since: Redis 1.2.0
    /// Group: Generic
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @keyspace
    /// * @write
    /// * @fast
    fn expireat<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("EXPIREAT");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// EXPIRETIME
    /// 
    /// Get the expiration Unix timestamp for a key
    /// 
    /// Since: Redis 7.0.0
    /// Group: Generic
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @keyspace
    /// * @read
    /// * @fast
    fn expiretime<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("EXPIRETIME");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// KEYS
    /// 
    /// Find all keys matching the given pattern
    /// 
    /// Since: Redis 1.0.0
    /// Group: Generic
    /// Complexity: O(N) with N being the number of keys in the database, under the assumption that the key names in the database and the given pattern have limited length.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @keyspace
    /// * @read
    /// * @slow
    /// * @dangerous
    fn keys<'a, K0: ToRedisArgs + Send + Sync + 'a>(pattern: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("KEYS");
            rv.arg(pattern);
            rv.query_async(self).await
        })
    }

    /// MIGRATE
    /// 
    /// Atomically transfer a key from a Redis instance to another one.
    /// 
    /// Since: Redis 2.6.0
    /// Group: Generic
    /// Complexity: This command actually executes a DUMP+DEL in the source instance, and a RESTORE in the target instance. See the pages of these commands for time complexity. Also an O(N) data transfer between the two instances is performed.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Movablekeys: This first key, last key, and step values don't determine all key positions. Clients need to use COMMAND GETKEYS or key specifications in this case. See below for more details.
    /// ACL Categories:
    /// * @keyspace
    /// * @write
    /// * @slow
    /// * @dangerous
    fn migrate<'a, T0: ToRedisArgs + Send + Sync + 'a>(host: T0, port: i64, destination_db: i64, timeout: i64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("MIGRATE");
            rv.arg(host);
            rv.arg(port);
            rv.arg(destination_db);
            rv.arg(timeout);
            rv.query_async(self).await
        })
    }

    /// MOVE
    /// 
    /// Move a key to another database
    /// 
    /// Since: Redis 1.0.0
    /// Group: Generic
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @keyspace
    /// * @write
    /// * @fast
    fn move_key<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0, db: i64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("MOVE");
            rv.arg(key);
            rv.arg(db);
            rv.query_async(self).await
        })
    }

    /// OBJECT ENCODING
    /// 
    /// Inspect the internal encoding of a Redis object
    /// 
    /// Since: Redis 2.2.3
    /// Group: Generic
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @keyspace
    /// * @read
    /// * @slow
    fn object_encoding<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("OBJECT ENCODING");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// OBJECT FREQ
    /// 
    /// Get the logarithmic access frequency counter of a Redis object
    /// 
    /// Since: Redis 4.0.0
    /// Group: Generic
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @keyspace
    /// * @read
    /// * @slow
    fn object_freq<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("OBJECT FREQ");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// OBJECT HELP
    /// 
    /// Show helpful text about the different subcommands
    /// 
    /// Since: Redis 6.2.0
    /// Group: Generic
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @keyspace
    /// * @slow
    fn object_help<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("OBJECT HELP");
            rv.query_async(self).await
        })
    }

    /// OBJECT IDLETIME
    /// 
    /// Get the time since a Redis object was last accessed
    /// 
    /// Since: Redis 2.2.3
    /// Group: Generic
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @keyspace
    /// * @read
    /// * @slow
    fn object_idletime<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("OBJECT IDLETIME");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// OBJECT REFCOUNT
    /// 
    /// Get the number of references to the value of the key
    /// 
    /// Since: Redis 2.2.3
    /// Group: Generic
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @keyspace
    /// * @read
    /// * @slow
    fn object_refcount<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("OBJECT REFCOUNT");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// PERSIST
    /// 
    /// Remove the expiration from a key
    /// 
    /// Since: Redis 2.2.0
    /// Group: Generic
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @keyspace
    /// * @write
    /// * @fast
    fn persist<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("PERSIST");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// PEXPIRE
    /// 
    /// Set a key's time to live in milliseconds
    /// 
    /// Since: Redis 2.6.0
    /// Group: Generic
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @keyspace
    /// * @write
    /// * @fast
    fn pexpire<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0, milliseconds: i64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("PEXPIRE");
            rv.arg(key);
            rv.arg(milliseconds);
            rv.query_async(self).await
        })
    }

    /// PEXPIREAT
    /// 
    /// Set the expiration for a key as a UNIX timestamp specified in milliseconds
    /// 
    /// Since: Redis 2.6.0
    /// Group: Generic
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @keyspace
    /// * @write
    /// * @fast
    fn pexpireat<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("PEXPIREAT");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// PEXPIRETIME
    /// 
    /// Get the expiration Unix timestamp for a key in milliseconds
    /// 
    /// Since: Redis 7.0.0
    /// Group: Generic
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @keyspace
    /// * @read
    /// * @fast
    fn pexpiretime<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("PEXPIRETIME");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// PTTL
    /// 
    /// Get the time to live for a key in milliseconds
    /// 
    /// Since: Redis 2.6.0
    /// Group: Generic
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @keyspace
    /// * @read
    /// * @fast
    fn pttl<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("PTTL");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// RANDOMKEY
    /// 
    /// Return a random key from the keyspace
    /// 
    /// Since: Redis 1.0.0
    /// Group: Generic
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @keyspace
    /// * @read
    /// * @slow
    fn randomkey<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("RANDOMKEY");
            rv.query_async(self).await
        })
    }

    /// RENAME
    /// 
    /// Rename a key
    /// 
    /// Since: Redis 1.0.0
    /// Group: Generic
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// ACL Categories:
    /// * @keyspace
    /// * @write
    /// * @slow
    fn rename<'a, K0: ToRedisArgs + Send + Sync + 'a, K1: ToRedisArgs + Send + Sync + 'a>(key: K0, newkey: K1) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("RENAME");
            rv.arg(key);
            rv.arg(newkey);
            rv.query_async(self).await
        })
    }

    /// RENAMENX
    /// 
    /// Rename a key, only if the new key does not exist
    /// 
    /// Since: Redis 1.0.0
    /// Group: Generic
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @keyspace
    /// * @write
    /// * @fast
    fn renamenx<'a, K0: ToRedisArgs + Send + Sync + 'a, K1: ToRedisArgs + Send + Sync + 'a>(key: K0, newkey: K1) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("RENAMENX");
            rv.arg(key);
            rv.arg(newkey);
            rv.query_async(self).await
        })
    }

    /// RESTORE
    /// 
    /// Create a key using the provided serialized value, previously obtained using DUMP.
    /// 
    /// Since: Redis 2.6.0
    /// Group: Generic
    /// Complexity: O(1) to create the new key and additional O(N*M) to reconstruct the serialized value, where N is the number of Redis objects composing the value and M their average size. For small string values the time complexity is thus O(1)+O(1*M) where M is small, so simply O(1). However for sorted set values the complexity is O(N*M*log(N)) because inserting values into sorted sets is O(log(N)).
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// ACL Categories:
    /// * @keyspace
    /// * @write
    /// * @slow
    /// * @dangerous
    fn restore<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, ttl: i64, serialized_value: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("RESTORE");
            rv.arg(key);
            rv.arg(ttl);
            rv.arg(serialized_value);
            rv.query_async(self).await
        })
    }

    /// SORT
    /// 
    /// Sort the elements in a list, set or sorted set
    /// 
    /// Since: Redis 1.0.0
    /// Group: Generic
    /// Complexity: O(N+M*log(M)) where N is the number of elements in the list or set to sort, and M the number of returned elements. When the elements are not sorted, complexity is O(N).
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Movablekeys: This first key, last key, and step values don't determine all key positions. Clients need to use COMMAND GETKEYS or key specifications in this case. See below for more details.
    /// ACL Categories:
    /// * @write
    /// * @set
    /// * @sortedset
    /// * @list
    /// * @slow
    /// * @dangerous
    fn sort<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SORT");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// SORT_RO
    /// 
    /// Sort the elements in a list, set or sorted set. Read-only variant of SORT.
    /// 
    /// Since: Redis 7.0.0
    /// Group: Generic
    /// Complexity: O(N+M*log(M)) where N is the number of elements in the list or set to sort, and M the number of returned elements. When the elements are not sorted, complexity is O(N).
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Movablekeys: This first key, last key, and step values don't determine all key positions. Clients need to use COMMAND GETKEYS or key specifications in this case. See below for more details.
    /// ACL Categories:
    /// * @read
    /// * @set
    /// * @sortedset
    /// * @list
    /// * @slow
    /// * @dangerous
    fn sort_ro<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SORT_RO");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// TOUCH
    /// 
    /// Alters the last access time of a key(s). Returns the number of existing keys specified.
    /// 
    /// Since: Redis 3.2.1
    /// Group: Generic
    /// Complexity: O(N) where N is the number of keys that will be touched.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @keyspace
    /// * @read
    /// * @fast
    fn touch<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: &'a [K0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("TOUCH");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// TTL
    /// 
    /// Get the time to live for a key in seconds
    /// 
    /// Since: Redis 1.0.0
    /// Group: Generic
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @keyspace
    /// * @read
    /// * @fast
    fn ttl<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("TTL");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// TYPE
    /// 
    /// Determine the type stored at key
    /// 
    /// Since: Redis 1.0.0
    /// Group: Generic
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @keyspace
    /// * @read
    /// * @fast
    fn r#type<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("TYPE");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// UNLINK
    /// 
    /// Delete a key asynchronously in another thread. Otherwise it is just as DEL, but non blocking.
    /// 
    /// Since: Redis 4.0.0
    /// Group: Generic
    /// Complexity: O(1) for each key removed regardless of its size. Then the command does O(N) work in a different thread in order to reclaim memory, where N is the number of allocations the deleted objects where composed of.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @keyspace
    /// * @write
    /// * @fast
    fn unlink<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: &'a [K0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("UNLINK");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// WAIT
    /// 
    /// Wait for the synchronous replication of all the write commands sent in the context of the current connection
    /// 
    /// Since: Redis 3.0.0
    /// Group: Generic
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// ACL Categories:
    /// * @slow
    /// * @connection
    fn wait<'a>(numreplicas: i64, timeout: i64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("WAIT");
            rv.arg(numreplicas);
            rv.arg(timeout);
            rv.query_async(self).await
        })
    }

    /// APPEND
    /// 
    /// Append a value to a key
    /// 
    /// Since: Redis 2.0.0
    /// Group: String
    /// Complexity: O(1). The amortized time complexity is O(1) assuming the appended value is small and the already present value is of any size, since the dynamic string library used by Redis will double the free space available on every reallocation.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @string
    /// * @fast
    fn append<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, value: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("APPEND");
            rv.arg(key);
            rv.arg(value);
            rv.query_async(self).await
        })
    }

    /// DECR
    /// 
    /// Decrement the integer value of a key by one
    /// 
    /// Since: Redis 1.0.0
    /// Group: String
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @string
    /// * @fast
    fn decr<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("DECR");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// DECRBY
    /// 
    /// Decrement the integer value of a key by the given number
    /// 
    /// Since: Redis 1.0.0
    /// Group: String
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @string
    /// * @fast
    fn decrby<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0, decrement: i64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("DECRBY");
            rv.arg(key);
            rv.arg(decrement);
            rv.query_async(self).await
        })
    }

    /// GET
    /// 
    /// Get the value of a key
    /// 
    /// Since: Redis 1.0.0
    /// Group: String
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @read
    /// * @string
    /// * @fast
    fn get<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("GET");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// GETDEL
    /// 
    /// Get the value of a key and delete the key
    /// 
    /// Since: Redis 6.2.0
    /// Group: String
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @string
    /// * @fast
    fn getdel<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("GETDEL");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// GETDEL
    /// 
    /// Get the value of a key and delete the key
    /// 
    /// Since: Redis 6.2.0
    /// Group: String
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @string
    /// * @fast
    fn get_del<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("GETDEL");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// GETEX
    /// 
    /// Get the value of a key and optionally set its expiration
    /// 
    /// Since: Redis 6.2.0
    /// Group: String
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @string
    /// * @fast
    fn getex<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("GETEX");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// GETRANGE
    /// 
    /// Get a substring of the string stored at a key
    /// 
    /// Since: Redis 2.4.0
    /// Group: String
    /// Complexity: O(N) where N is the length of the returned string. The complexity is ultimately determined by the returned length, but because creating a substring from an existing string is very cheap, it can be considered O(1) for small strings.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @string
    /// * @slow
    fn getrange<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0, start: i64, end: i64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("GETRANGE");
            rv.arg(key);
            rv.arg(start);
            rv.arg(end);
            rv.query_async(self).await
        })
    }

    /// GETSET
    /// 
    /// Set the string value of a key and return its old value
    /// 
    /// Since: Redis 1.0.0
    /// Group: String
    /// Replaced By: `SET` with the `!GET` argument
    /// Complexity: O(1)
    /// Replaced By: `SET` with the `!GET` argument
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @string
    /// * @fast
    #[deprecated]
    fn getset<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, value: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("GETSET");
            rv.arg(key);
            rv.arg(value);
            rv.query_async(self).await
        })
    }

    /// INCR
    /// 
    /// Increment the integer value of a key by one
    /// 
    /// Since: Redis 1.0.0
    /// Group: String
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @string
    /// * @fast
    fn incr<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("INCR");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// INCRBY
    /// 
    /// Increment the integer value of a key by the given amount
    /// 
    /// Since: Redis 1.0.0
    /// Group: String
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @string
    /// * @fast
    fn incrby<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0, increment: i64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("INCRBY");
            rv.arg(key);
            rv.arg(increment);
            rv.query_async(self).await
        })
    }

    /// INCRBYFLOAT
    /// 
    /// Increment the float value of a key by the given amount
    /// 
    /// Since: Redis 2.6.0
    /// Group: String
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @string
    /// * @fast
    fn incrbyfloat<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0, increment: f64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("INCRBYFLOAT");
            rv.arg(key);
            rv.arg(increment);
            rv.query_async(self).await
        })
    }

    /// LCS
    /// 
    /// Find longest common substring
    /// 
    /// Since: Redis 7.0.0
    /// Group: String
    /// Complexity: O(N*M) where N and M are the lengths of s1 and s2, respectively
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @string
    /// * @slow
    fn lcs<'a, K0: ToRedisArgs + Send + Sync + 'a, K1: ToRedisArgs + Send + Sync + 'a>(key1: K0, key2: K1) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("LCS");
            rv.arg(key1);
            rv.arg(key2);
            rv.query_async(self).await
        })
    }

    /// MGET
    /// 
    /// Get the values of all the given keys
    /// 
    /// Since: Redis 1.0.0
    /// Group: String
    /// Complexity: O(N) where N is the number of keys to retrieve.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @read
    /// * @string
    /// * @fast
    fn mget<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: &'a [K0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("MGET");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// MSET
    /// 
    /// Set multiple keys to multiple values
    /// 
    /// Since: Redis 1.0.1
    /// Group: String
    /// Complexity: O(N) where N is the number of keys to set.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// ACL Categories:
    /// * @write
    /// * @string
    /// * @slow
    fn mset<'a, T0: ToRedisArgs + Send + Sync + 'a>(key_value: &'a [T0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("MSET");
            rv.arg(key_value);
            rv.query_async(self).await
        })
    }

    /// MSETNX
    /// 
    /// Set multiple keys to multiple values, only if none of the keys exist
    /// 
    /// Since: Redis 1.0.1
    /// Group: String
    /// Complexity: O(N) where N is the number of keys to set.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// ACL Categories:
    /// * @write
    /// * @string
    /// * @slow
    fn msetnx<'a, T0: ToRedisArgs + Send + Sync + 'a>(key_value: &'a [T0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("MSETNX");
            rv.arg(key_value);
            rv.query_async(self).await
        })
    }

    /// PSETEX
    /// 
    /// Set the value and expiration in milliseconds of a key
    /// 
    /// Since: Redis 2.6.0
    /// Group: String
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// ACL Categories:
    /// * @write
    /// * @string
    /// * @slow
    fn psetex<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, milliseconds: i64, value: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("PSETEX");
            rv.arg(key);
            rv.arg(milliseconds);
            rv.arg(value);
            rv.query_async(self).await
        })
    }

    /// SET
    /// 
    /// Set the string value of a key
    /// 
    /// Since: Redis 1.0.0
    /// Group: String
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Movablekeys: This first key, last key, and step values don't determine all key positions. Clients need to use COMMAND GETKEYS or key specifications in this case. See below for more details.
    /// ACL Categories:
    /// * @write
    /// * @string
    /// * @slow
    fn set<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, value: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SET");
            rv.arg(key);
            rv.arg(value);
            rv.query_async(self).await
        })
    }

    /// SETEX
    /// 
    /// Set the value and expiration of a key
    /// 
    /// Since: Redis 2.0.0
    /// Group: String
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// ACL Categories:
    /// * @write
    /// * @string
    /// * @slow
    fn setex<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, seconds: i64, value: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SETEX");
            rv.arg(key);
            rv.arg(seconds);
            rv.arg(value);
            rv.query_async(self).await
        })
    }

    /// SETNX
    /// 
    /// Set the value of a key, only if the key does not exist
    /// 
    /// Since: Redis 1.0.0
    /// Group: String
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @string
    /// * @fast
    fn setnx<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, value: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SETNX");
            rv.arg(key);
            rv.arg(value);
            rv.query_async(self).await
        })
    }

    /// SETRANGE
    /// 
    /// Overwrite part of a string at key starting at the specified offset
    /// 
    /// Since: Redis 2.2.0
    /// Group: String
    /// Complexity: O(1), not counting the time taken to copy the new string in place. Usually, this string is very small so the amortized complexity is O(1). Otherwise, complexity is O(M) with M being the length of the value argument.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// ACL Categories:
    /// * @write
    /// * @string
    /// * @slow
    fn setrange<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, offset: i64, value: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SETRANGE");
            rv.arg(key);
            rv.arg(offset);
            rv.arg(value);
            rv.query_async(self).await
        })
    }

    /// STRLEN
    /// 
    /// Get the length of the value stored in a key
    /// 
    /// Since: Redis 2.2.0
    /// Group: String
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @read
    /// * @string
    /// * @fast
    fn strlen<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("STRLEN");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// SUBSTR
    /// 
    /// Get a substring of the string stored at a key
    /// 
    /// Since: Redis 1.0.0
    /// Group: String
    /// Replaced By: `GETRANGE`
    /// Complexity: O(N) where N is the length of the returned string. The complexity is ultimately determined by the returned length, but because creating a substring from an existing string is very cheap, it can be considered O(1) for small strings.
    /// Replaced By: `GETRANGE`
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @string
    /// * @slow
    #[deprecated]
    fn substr<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0, start: i64, end: i64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SUBSTR");
            rv.arg(key);
            rv.arg(start);
            rv.arg(end);
            rv.query_async(self).await
        })
    }

    /// BLMOVE
    /// 
    /// Pop an element from a list, push it to another list and return it; or block until one is available
    /// 
    /// Since: Redis 6.2.0
    /// Group: List
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Blocking: This command may block the requesting client.
    /// ACL Categories:
    /// * @write
    /// * @list
    /// * @slow
    /// * @blocking
    fn blmove<'a, K0: ToRedisArgs + Send + Sync + 'a, K1: ToRedisArgs + Send + Sync + 'a>(source: K0, destination: K1, timeout: f64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("BLMOVE");
            rv.arg(source);
            rv.arg(destination);
            rv.arg(timeout);
            rv.query_async(self).await
        })
    }

    /// BLMPOP
    /// 
    /// Pop elements from a list, or block until one is available
    /// 
    /// Since: Redis 7.0.0
    /// Group: List
    /// Complexity: O(N+M) where N is the number of provided keys and M is the number of elements returned.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Blocking: This command may block the requesting client.
    /// * Movablekeys: This first key, last key, and step values don't determine all key positions. Clients need to use COMMAND GETKEYS or key specifications in this case. See below for more details.
    /// ACL Categories:
    /// * @write
    /// * @list
    /// * @slow
    /// * @blocking
    fn blmpop<'a, K0: ToRedisArgs + Send + Sync + 'a>(timeout: f64, numkeys: i64, key: &'a [K0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("BLMPOP");
            rv.arg(timeout);
            rv.arg(numkeys);
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// BLPOP
    /// 
    /// Remove and get the first element in a list, or block until one is available
    /// 
    /// Since: Redis 2.0.0
    /// Group: List
    /// Complexity: O(N) where N is the number of provided keys.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Blocking: This command may block the requesting client.
    /// ACL Categories:
    /// * @write
    /// * @list
    /// * @slow
    /// * @blocking
    fn blpop<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: &'a [K0], timeout: f64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("BLPOP");
            rv.arg(key);
            rv.arg(timeout);
            rv.query_async(self).await
        })
    }

    /// BRPOP
    /// 
    /// Remove and get the last element in a list, or block until one is available
    /// 
    /// Since: Redis 2.0.0
    /// Group: List
    /// Complexity: O(N) where N is the number of provided keys.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Blocking: This command may block the requesting client.
    /// ACL Categories:
    /// * @write
    /// * @list
    /// * @slow
    /// * @blocking
    fn brpop<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: &'a [K0], timeout: f64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("BRPOP");
            rv.arg(key);
            rv.arg(timeout);
            rv.query_async(self).await
        })
    }

    /// BRPOPLPUSH
    /// 
    /// Pop an element from a list, push it to another list and return it; or block until one is available
    /// 
    /// Since: Redis 2.2.0
    /// Group: List
    /// Replaced By: `BLMOVE` with the `RIGHT` and `LEFT` arguments
    /// Complexity: O(1)
    /// Replaced By: `BLMOVE` with the `RIGHT` and `LEFT` arguments
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Blocking: This command may block the requesting client.
    /// ACL Categories:
    /// * @write
    /// * @list
    /// * @slow
    /// * @blocking
    #[deprecated]
    fn brpoplpush<'a, K0: ToRedisArgs + Send + Sync + 'a, K1: ToRedisArgs + Send + Sync + 'a>(source: K0, destination: K1, timeout: f64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("BRPOPLPUSH");
            rv.arg(source);
            rv.arg(destination);
            rv.arg(timeout);
            rv.query_async(self).await
        })
    }

    /// LINDEX
    /// 
    /// Get an element from a list by its index
    /// 
    /// Since: Redis 1.0.0
    /// Group: List
    /// Complexity: O(N) where N is the number of elements to traverse to get to the element at index. This makes asking for the first or the last element of the list O(1).
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @list
    /// * @slow
    fn lindex<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0, index: i64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("LINDEX");
            rv.arg(key);
            rv.arg(index);
            rv.query_async(self).await
        })
    }

    /// LINSERT
    /// 
    /// Insert an element before or after another element in a list
    /// 
    /// Since: Redis 2.2.0
    /// Group: List
    /// Complexity: O(N) where N is the number of elements to traverse before seeing the value pivot. This means that inserting somewhere on the left end on the list (head) can be considered O(1) and inserting somewhere on the right end (tail) is O(N).
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// ACL Categories:
    /// * @write
    /// * @list
    /// * @slow
    fn linsert<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(key: K0, pivot: T0, element: T1) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("LINSERT");
            rv.arg(key);
            rv.arg(pivot);
            rv.arg(element);
            rv.query_async(self).await
        })
    }

    /// LLEN
    /// 
    /// Get the length of a list
    /// 
    /// Since: Redis 1.0.0
    /// Group: List
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @read
    /// * @list
    /// * @fast
    fn llen<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("LLEN");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// LMOVE
    /// 
    /// Pop an element from a list, push it to another list and return it
    /// 
    /// Since: Redis 6.2.0
    /// Group: List
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// ACL Categories:
    /// * @write
    /// * @list
    /// * @slow
    fn lmove<'a, K0: ToRedisArgs + Send + Sync + 'a, K1: ToRedisArgs + Send + Sync + 'a>(source: K0, destination: K1) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("LMOVE");
            rv.arg(source);
            rv.arg(destination);
            rv.query_async(self).await
        })
    }

    /// LMPOP
    /// 
    /// Pop elements from a list
    /// 
    /// Since: Redis 7.0.0
    /// Group: List
    /// Complexity: O(N+M) where N is the number of provided keys and M is the number of elements returned.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Movablekeys: This first key, last key, and step values don't determine all key positions. Clients need to use COMMAND GETKEYS or key specifications in this case. See below for more details.
    /// ACL Categories:
    /// * @write
    /// * @list
    /// * @slow
    fn lmpop<'a, K0: ToRedisArgs + Send + Sync + 'a>(numkeys: i64, key: &'a [K0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("LMPOP");
            rv.arg(numkeys);
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// LPOP
    /// 
    /// Remove and get the first elements in a list
    /// 
    /// Since: Redis 1.0.0
    /// Group: List
    /// Complexity: O(N) where N is the number of elements returned
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @list
    /// * @fast
    fn lpop<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0, count: Option<i64>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("LPOP");
            rv.arg(key);
            rv.arg(count);
            rv.query_async(self).await
        })
    }

    /// LPOS
    /// 
    /// Return the index of matching elements on a list
    /// 
    /// Since: Redis 6.0.6
    /// Group: List
    /// Complexity: O(N) where N is the number of elements in the list, for the average case. When searching for elements near the head or the tail of the list, or when the MAXLEN option is provided, the command may run in constant time.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @list
    /// * @slow
    fn lpos<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, element: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("LPOS");
            rv.arg(key);
            rv.arg(element);
            rv.query_async(self).await
        })
    }

    /// LPUSH
    /// 
    /// Prepend one or multiple elements to a list
    /// 
    /// Since: Redis 1.0.0
    /// Group: List
    /// Complexity: O(1) for each element added, so O(N) to add N elements when the command is called with multiple arguments.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @list
    /// * @fast
    fn lpush<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, element: &'a [T0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("LPUSH");
            rv.arg(key);
            rv.arg(element);
            rv.query_async(self).await
        })
    }

    /// LPUSHX
    /// 
    /// Prepend an element to a list, only if the list exists
    /// 
    /// Since: Redis 2.2.0
    /// Group: List
    /// Complexity: O(1) for each element added, so O(N) to add N elements when the command is called with multiple arguments.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @list
    /// * @fast
    fn lpushx<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, element: &'a [T0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("LPUSHX");
            rv.arg(key);
            rv.arg(element);
            rv.query_async(self).await
        })
    }

    /// LRANGE
    /// 
    /// Get a range of elements from a list
    /// 
    /// Since: Redis 1.0.0
    /// Group: List
    /// Complexity: O(S+N) where S is the distance of start offset from HEAD for small lists, from nearest end (HEAD or TAIL) for large lists; and N is the number of elements in the specified range.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @list
    /// * @slow
    fn lrange<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0, start: i64, stop: i64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("LRANGE");
            rv.arg(key);
            rv.arg(start);
            rv.arg(stop);
            rv.query_async(self).await
        })
    }

    /// LREM
    /// 
    /// Remove elements from a list
    /// 
    /// Since: Redis 1.0.0
    /// Group: List
    /// Complexity: O(N+M) where N is the length of the list and M is the number of elements removed.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// ACL Categories:
    /// * @write
    /// * @list
    /// * @slow
    fn lrem<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, count: i64, element: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("LREM");
            rv.arg(key);
            rv.arg(count);
            rv.arg(element);
            rv.query_async(self).await
        })
    }

    /// LSET
    /// 
    /// Set the value of an element in a list by its index
    /// 
    /// Since: Redis 1.0.0
    /// Group: List
    /// Complexity: O(N) where N is the length of the list. Setting either the first or the last element of the list is O(1).
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// ACL Categories:
    /// * @write
    /// * @list
    /// * @slow
    fn lset<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, index: i64, element: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("LSET");
            rv.arg(key);
            rv.arg(index);
            rv.arg(element);
            rv.query_async(self).await
        })
    }

    /// LTRIM
    /// 
    /// Trim a list to the specified range
    /// 
    /// Since: Redis 1.0.0
    /// Group: List
    /// Complexity: O(N) where N is the number of elements to be removed by the operation.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// ACL Categories:
    /// * @write
    /// * @list
    /// * @slow
    fn ltrim<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0, start: i64, stop: i64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("LTRIM");
            rv.arg(key);
            rv.arg(start);
            rv.arg(stop);
            rv.query_async(self).await
        })
    }

    /// RPOP
    /// 
    /// Remove and get the last elements in a list
    /// 
    /// Since: Redis 1.0.0
    /// Group: List
    /// Complexity: O(N) where N is the number of elements returned
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @list
    /// * @fast
    fn rpop<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0, count: Option<i64>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("RPOP");
            rv.arg(key);
            rv.arg(count);
            rv.query_async(self).await
        })
    }

    /// RPOPLPUSH
    /// 
    /// Remove the last element in a list, prepend it to another list and return it
    /// 
    /// Since: Redis 1.2.0
    /// Group: List
    /// Replaced By: `LMOVE` with the `RIGHT` and `LEFT` arguments
    /// Complexity: O(1)
    /// Replaced By: `LMOVE` with the `RIGHT` and `LEFT` arguments
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// ACL Categories:
    /// * @write
    /// * @list
    /// * @slow
    #[deprecated]
    fn rpoplpush<'a, K0: ToRedisArgs + Send + Sync + 'a, K1: ToRedisArgs + Send + Sync + 'a>(source: K0, destination: K1) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("RPOPLPUSH");
            rv.arg(source);
            rv.arg(destination);
            rv.query_async(self).await
        })
    }

    /// RPUSH
    /// 
    /// Append one or multiple elements to a list
    /// 
    /// Since: Redis 1.0.0
    /// Group: List
    /// Complexity: O(1) for each element added, so O(N) to add N elements when the command is called with multiple arguments.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @list
    /// * @fast
    fn rpush<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, element: &'a [T0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("RPUSH");
            rv.arg(key);
            rv.arg(element);
            rv.query_async(self).await
        })
    }

    /// RPUSHX
    /// 
    /// Append an element to a list, only if the list exists
    /// 
    /// Since: Redis 2.2.0
    /// Group: List
    /// Complexity: O(1) for each element added, so O(N) to add N elements when the command is called with multiple arguments.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @list
    /// * @fast
    fn rpushx<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, element: &'a [T0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("RPUSHX");
            rv.arg(key);
            rv.arg(element);
            rv.query_async(self).await
        })
    }

    /// SADD
    /// 
    /// Add one or more members to a set
    /// 
    /// Since: Redis 1.0.0
    /// Group: Set
    /// Complexity: O(1) for each element added, so O(N) to add N elements when the command is called with multiple arguments.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @set
    /// * @fast
    fn sadd<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, member: &'a [T0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SADD");
            rv.arg(key);
            rv.arg(member);
            rv.query_async(self).await
        })
    }

    /// SCARD
    /// 
    /// Get the number of members in a set
    /// 
    /// Since: Redis 1.0.0
    /// Group: Set
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @read
    /// * @set
    /// * @fast
    fn scard<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SCARD");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// SDIFF
    /// 
    /// Subtract multiple sets
    /// 
    /// Since: Redis 1.0.0
    /// Group: Set
    /// Complexity: O(N) where N is the total number of elements in all given sets.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @set
    /// * @slow
    fn sdiff<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: &'a [K0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SDIFF");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// SDIFFSTORE
    /// 
    /// Subtract multiple sets and store the resulting set in a key
    /// 
    /// Since: Redis 1.0.0
    /// Group: Set
    /// Complexity: O(N) where N is the total number of elements in all given sets.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// ACL Categories:
    /// * @write
    /// * @set
    /// * @slow
    fn sdiffstore<'a, K0: ToRedisArgs + Send + Sync + 'a, K1: ToRedisArgs + Send + Sync + 'a>(destination: K0, key: &'a [K1]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SDIFFSTORE");
            rv.arg(destination);
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// SINTER
    /// 
    /// Intersect multiple sets
    /// 
    /// Since: Redis 1.0.0
    /// Group: Set
    /// Complexity: O(N*M) worst case where N is the cardinality of the smallest set and M is the number of sets.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @set
    /// * @slow
    fn sinter<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: &'a [K0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SINTER");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// SINTERCARD
    /// 
    /// Intersect multiple sets and return the cardinality of the result
    /// 
    /// Since: Redis 7.0.0
    /// Group: Set
    /// Complexity: O(N*M) worst case where N is the cardinality of the smallest set and M is the number of sets.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Movablekeys: This first key, last key, and step values don't determine all key positions. Clients need to use COMMAND GETKEYS or key specifications in this case. See below for more details.
    /// ACL Categories:
    /// * @read
    /// * @set
    /// * @slow
    fn sintercard<'a, K0: ToRedisArgs + Send + Sync + 'a>(numkeys: i64, key: &'a [K0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SINTERCARD");
            rv.arg(numkeys);
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// SINTERSTORE
    /// 
    /// Intersect multiple sets and store the resulting set in a key
    /// 
    /// Since: Redis 1.0.0
    /// Group: Set
    /// Complexity: O(N*M) worst case where N is the cardinality of the smallest set and M is the number of sets.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// ACL Categories:
    /// * @write
    /// * @set
    /// * @slow
    fn sinterstore<'a, K0: ToRedisArgs + Send + Sync + 'a, K1: ToRedisArgs + Send + Sync + 'a>(destination: K0, key: &'a [K1]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SINTERSTORE");
            rv.arg(destination);
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// SISMEMBER
    /// 
    /// Determine if a given value is a member of a set
    /// 
    /// Since: Redis 1.0.0
    /// Group: Set
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @read
    /// * @set
    /// * @fast
    fn sismember<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, member: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SISMEMBER");
            rv.arg(key);
            rv.arg(member);
            rv.query_async(self).await
        })
    }

    /// SMEMBERS
    /// 
    /// Get all the members in a set
    /// 
    /// Since: Redis 1.0.0
    /// Group: Set
    /// Complexity: O(N) where N is the set cardinality.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @set
    /// * @slow
    fn smembers<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SMEMBERS");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// SMISMEMBER
    /// 
    /// Returns the membership associated with the given elements for a set
    /// 
    /// Since: Redis 6.2.0
    /// Group: Set
    /// Complexity: O(N) where N is the number of elements being checked for membership
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @read
    /// * @set
    /// * @fast
    fn smismember<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, member: &'a [T0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SMISMEMBER");
            rv.arg(key);
            rv.arg(member);
            rv.query_async(self).await
        })
    }

    /// SMOVE
    /// 
    /// Move a member from one set to another
    /// 
    /// Since: Redis 1.0.0
    /// Group: Set
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @set
    /// * @fast
    fn smove<'a, K0: ToRedisArgs + Send + Sync + 'a, K1: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(source: K0, destination: K1, member: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SMOVE");
            rv.arg(source);
            rv.arg(destination);
            rv.arg(member);
            rv.query_async(self).await
        })
    }

    /// SPOP
    /// 
    /// Remove and return one or multiple random members from a set
    /// 
    /// Since: Redis 1.0.0
    /// Group: Set
    /// Complexity: Without the count argument O(1), otherwise O(N) where N is the value of the passed count.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @set
    /// * @fast
    fn spop<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0, count: Option<i64>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SPOP");
            rv.arg(key);
            rv.arg(count);
            rv.query_async(self).await
        })
    }

    /// SRANDMEMBER
    /// 
    /// Get one or multiple random members from a set
    /// 
    /// Since: Redis 1.0.0
    /// Group: Set
    /// Complexity: Without the count argument O(1), otherwise O(N) where N is the absolute value of the passed count.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @set
    /// * @slow
    fn srandmember<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0, count: Option<i64>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SRANDMEMBER");
            rv.arg(key);
            rv.arg(count);
            rv.query_async(self).await
        })
    }

    /// SREM
    /// 
    /// Remove one or more members from a set
    /// 
    /// Since: Redis 1.0.0
    /// Group: Set
    /// Complexity: O(N) where N is the number of members to be removed.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @set
    /// * @fast
    fn srem<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, member: &'a [T0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SREM");
            rv.arg(key);
            rv.arg(member);
            rv.query_async(self).await
        })
    }

    /// SUNION
    /// 
    /// Add multiple sets
    /// 
    /// Since: Redis 1.0.0
    /// Group: Set
    /// Complexity: O(N) where N is the total number of elements in all given sets.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @set
    /// * @slow
    fn sunion<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: &'a [K0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SUNION");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// SUNIONSTORE
    /// 
    /// Add multiple sets and store the resulting set in a key
    /// 
    /// Since: Redis 1.0.0
    /// Group: Set
    /// Complexity: O(N) where N is the total number of elements in all given sets.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// ACL Categories:
    /// * @write
    /// * @set
    /// * @slow
    fn sunionstore<'a, K0: ToRedisArgs + Send + Sync + 'a, K1: ToRedisArgs + Send + Sync + 'a>(destination: K0, key: &'a [K1]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SUNIONSTORE");
            rv.arg(destination);
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// BZMPOP
    /// 
    /// Remove and return members with scores in a sorted set or block until one is available
    /// 
    /// Since: Redis 7.0.0
    /// Group: SortedSet
    /// Complexity: O(K) + O(N*log(M)) where K is the number of provided keys, N being the number of elements in the sorted set, and M being the number of elements popped.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Blocking: This command may block the requesting client.
    /// * Movablekeys: This first key, last key, and step values don't determine all key positions. Clients need to use COMMAND GETKEYS or key specifications in this case. See below for more details.
    /// ACL Categories:
    /// * @write
    /// * @sortedset
    /// * @slow
    /// * @blocking
    fn bzmpop<'a, K0: ToRedisArgs + Send + Sync + 'a>(timeout: f64, numkeys: i64, key: &'a [K0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("BZMPOP");
            rv.arg(timeout);
            rv.arg(numkeys);
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// BZPOPMAX
    /// 
    /// Remove and return the member with the highest score from one or more sorted sets, or block until one is available
    /// 
    /// Since: Redis 5.0.0
    /// Group: SortedSet
    /// Complexity: O(log(N)) with N being the number of elements in the sorted set.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Blocking: This command may block the requesting client.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @sortedset
    /// * @fast
    /// * @blocking
    fn bzpopmax<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: &'a [K0], timeout: f64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("BZPOPMAX");
            rv.arg(key);
            rv.arg(timeout);
            rv.query_async(self).await
        })
    }

    /// BZPOPMIN
    /// 
    /// Remove and return the member with the lowest score from one or more sorted sets, or block until one is available
    /// 
    /// Since: Redis 5.0.0
    /// Group: SortedSet
    /// Complexity: O(log(N)) with N being the number of elements in the sorted set.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Blocking: This command may block the requesting client.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @sortedset
    /// * @fast
    /// * @blocking
    fn bzpopmin<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: &'a [K0], timeout: f64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("BZPOPMIN");
            rv.arg(key);
            rv.arg(timeout);
            rv.query_async(self).await
        })
    }

    /// ZADD
    /// 
    /// Add one or more members to a sorted set, or update its score if it already exists
    /// 
    /// Since: Redis 1.2.0
    /// Group: SortedSet
    /// Complexity: O(log(N)) for each item added, where N is the number of elements in the sorted set.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @sortedset
    /// * @fast
    fn zadd<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, score_member: &'a [T0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZADD");
            rv.arg(key);
            rv.arg(score_member);
            rv.query_async(self).await
        })
    }

    /// ZCARD
    /// 
    /// Get the number of members in a sorted set
    /// 
    /// Since: Redis 1.2.0
    /// Group: SortedSet
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @read
    /// * @sortedset
    /// * @fast
    fn zcard<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZCARD");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// ZCOUNT
    /// 
    /// Count the members in a sorted set with scores within the given values
    /// 
    /// Since: Redis 2.0.0
    /// Group: SortedSet
    /// Complexity: O(log(N)) with N being the number of elements in the sorted set.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @read
    /// * @sortedset
    /// * @fast
    fn zcount<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0, min: f64, max: f64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZCOUNT");
            rv.arg(key);
            rv.arg(min);
            rv.arg(max);
            rv.query_async(self).await
        })
    }

    /// ZDIFF
    /// 
    /// Subtract multiple sorted sets
    /// 
    /// Since: Redis 6.2.0
    /// Group: SortedSet
    /// Complexity: O(L + (N-K)log(N)) worst case where L is the total number of elements in all the sets, N is the size of the first set, and K is the size of the result set.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Movablekeys: This first key, last key, and step values don't determine all key positions. Clients need to use COMMAND GETKEYS or key specifications in this case. See below for more details.
    /// ACL Categories:
    /// * @read
    /// * @sortedset
    /// * @slow
    fn zdiff<'a, K0: ToRedisArgs + Send + Sync + 'a>(numkeys: i64, key: &'a [K0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZDIFF");
            rv.arg(numkeys);
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// ZDIFFSTORE
    /// 
    /// Subtract multiple sorted sets and store the resulting sorted set in a new key
    /// 
    /// Since: Redis 6.2.0
    /// Group: SortedSet
    /// Complexity: O(L + (N-K)log(N)) worst case where L is the total number of elements in all the sets, N is the size of the first set, and K is the size of the result set.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Movablekeys: This first key, last key, and step values don't determine all key positions. Clients need to use COMMAND GETKEYS or key specifications in this case. See below for more details.
    /// ACL Categories:
    /// * @write
    /// * @sortedset
    /// * @slow
    fn zdiffstore<'a, K0: ToRedisArgs + Send + Sync + 'a, K1: ToRedisArgs + Send + Sync + 'a>(destination: K0, numkeys: i64, key: &'a [K1]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZDIFFSTORE");
            rv.arg(destination);
            rv.arg(numkeys);
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// ZINCRBY
    /// 
    /// Increment the score of a member in a sorted set
    /// 
    /// Since: Redis 1.2.0
    /// Group: SortedSet
    /// Complexity: O(log(N)) where N is the number of elements in the sorted set.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @sortedset
    /// * @fast
    fn zincrby<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, increment: i64, member: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZINCRBY");
            rv.arg(key);
            rv.arg(increment);
            rv.arg(member);
            rv.query_async(self).await
        })
    }

    /// ZINTER
    /// 
    /// Intersect multiple sorted sets
    /// 
    /// Since: Redis 6.2.0
    /// Group: SortedSet
    /// Complexity: O(N*K)+O(M*log(M)) worst case with N being the smallest input sorted set, K being the number of input sorted sets and M being the number of elements in the resulting sorted set.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Movablekeys: This first key, last key, and step values don't determine all key positions. Clients need to use COMMAND GETKEYS or key specifications in this case. See below for more details.
    /// ACL Categories:
    /// * @read
    /// * @sortedset
    /// * @slow
    fn zinter<'a, K0: ToRedisArgs + Send + Sync + 'a>(numkeys: i64, key: &'a [K0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZINTER");
            rv.arg(numkeys);
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// ZINTERCARD
    /// 
    /// Intersect multiple sorted sets and return the cardinality of the result
    /// 
    /// Since: Redis 7.0.0
    /// Group: SortedSet
    /// Complexity: O(N*K) worst case with N being the smallest input sorted set, K being the number of input sorted sets.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Movablekeys: This first key, last key, and step values don't determine all key positions. Clients need to use COMMAND GETKEYS or key specifications in this case. See below for more details.
    /// ACL Categories:
    /// * @read
    /// * @sortedset
    /// * @slow
    fn zintercard<'a, K0: ToRedisArgs + Send + Sync + 'a>(numkeys: i64, key: &'a [K0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZINTERCARD");
            rv.arg(numkeys);
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// ZINTERSTORE
    /// 
    /// Intersect multiple sorted sets and store the resulting sorted set in a new key
    /// 
    /// Since: Redis 2.0.0
    /// Group: SortedSet
    /// Complexity: O(N*K)+O(M*log(M)) worst case with N being the smallest input sorted set, K being the number of input sorted sets and M being the number of elements in the resulting sorted set.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Movablekeys: This first key, last key, and step values don't determine all key positions. Clients need to use COMMAND GETKEYS or key specifications in this case. See below for more details.
    /// ACL Categories:
    /// * @write
    /// * @sortedset
    /// * @slow
    fn zinterstore<'a, K0: ToRedisArgs + Send + Sync + 'a, K1: ToRedisArgs + Send + Sync + 'a>(destination: K0, numkeys: i64, key: &'a [K1]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZINTERSTORE");
            rv.arg(destination);
            rv.arg(numkeys);
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// ZLEXCOUNT
    /// 
    /// Count the number of members in a sorted set between a given lexicographical range
    /// 
    /// Since: Redis 2.8.9
    /// Group: SortedSet
    /// Complexity: O(log(N)) with N being the number of elements in the sorted set.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @read
    /// * @sortedset
    /// * @fast
    fn zlexcount<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(key: K0, min: T0, max: T1) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZLEXCOUNT");
            rv.arg(key);
            rv.arg(min);
            rv.arg(max);
            rv.query_async(self).await
        })
    }

    /// ZMPOP
    /// 
    /// Remove and return members with scores in a sorted set
    /// 
    /// Since: Redis 7.0.0
    /// Group: SortedSet
    /// Complexity: O(K) + O(N*log(M)) where K is the number of provided keys, N being the number of elements in the sorted set, and M being the number of elements popped.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Movablekeys: This first key, last key, and step values don't determine all key positions. Clients need to use COMMAND GETKEYS or key specifications in this case. See below for more details.
    /// ACL Categories:
    /// * @write
    /// * @sortedset
    /// * @slow
    fn zmpop<'a, K0: ToRedisArgs + Send + Sync + 'a>(numkeys: i64, key: &'a [K0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZMPOP");
            rv.arg(numkeys);
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// ZMSCORE
    /// 
    /// Get the score associated with the given members in a sorted set
    /// 
    /// Since: Redis 6.2.0
    /// Group: SortedSet
    /// Complexity: O(N) where N is the number of members being requested.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @read
    /// * @sortedset
    /// * @fast
    fn zmscore<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, member: &'a [T0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZMSCORE");
            rv.arg(key);
            rv.arg(member);
            rv.query_async(self).await
        })
    }

    /// ZPOPMAX
    /// 
    /// Remove and return members with the highest scores in a sorted set
    /// 
    /// Since: Redis 5.0.0
    /// Group: SortedSet
    /// Complexity: O(log(N)*M) with N being the number of elements in the sorted set, and M being the number of elements popped.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @sortedset
    /// * @fast
    fn zpopmax<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0, count: Option<i64>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZPOPMAX");
            rv.arg(key);
            rv.arg(count);
            rv.query_async(self).await
        })
    }

    /// ZPOPMIN
    /// 
    /// Remove and return members with the lowest scores in a sorted set
    /// 
    /// Since: Redis 5.0.0
    /// Group: SortedSet
    /// Complexity: O(log(N)*M) with N being the number of elements in the sorted set, and M being the number of elements popped.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @sortedset
    /// * @fast
    fn zpopmin<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0, count: Option<i64>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZPOPMIN");
            rv.arg(key);
            rv.arg(count);
            rv.query_async(self).await
        })
    }

    /// ZRANDMEMBER
    /// 
    /// Get one or multiple random elements from a sorted set
    /// 
    /// Since: Redis 6.2.0
    /// Group: SortedSet
    /// Complexity: O(N) where N is the number of elements returned
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @sortedset
    /// * @slow
    fn zrandmember<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, options: Option<T0>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZRANDMEMBER");
            rv.arg(key);
            rv.arg(options);
            rv.query_async(self).await
        })
    }

    /// ZRANGE
    /// 
    /// Return a range of members in a sorted set
    /// 
    /// Since: Redis 1.2.0
    /// Group: SortedSet
    /// Complexity: O(log(N)+M) with N being the number of elements in the sorted set and M the number of elements returned.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @sortedset
    /// * @slow
    fn zrange<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(key: K0, min: T0, max: T1) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZRANGE");
            rv.arg(key);
            rv.arg(min);
            rv.arg(max);
            rv.query_async(self).await
        })
    }

    /// ZRANGEBYLEX
    /// 
    /// Return a range of members in a sorted set, by lexicographical range
    /// 
    /// Since: Redis 2.8.9
    /// Group: SortedSet
    /// Replaced By: `ZRANGE` with the `BYLEX` argument
    /// Complexity: O(log(N)+M) with N being the number of elements in the sorted set and M the number of elements being returned. If M is constant (e.g. always asking for the first 10 elements with LIMIT), you can consider it O(log(N)).
    /// Replaced By: `ZRANGE` with the `BYLEX` argument
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @sortedset
    /// * @slow
    #[deprecated]
    fn zrangebylex<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(key: K0, min: T0, max: T1) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZRANGEBYLEX");
            rv.arg(key);
            rv.arg(min);
            rv.arg(max);
            rv.query_async(self).await
        })
    }

    /// ZRANGEBYSCORE
    /// 
    /// Return a range of members in a sorted set, by score
    /// 
    /// Since: Redis 1.0.5
    /// Group: SortedSet
    /// Replaced By: `ZRANGE` with the `BYSCORE` argument
    /// Complexity: O(log(N)+M) with N being the number of elements in the sorted set and M the number of elements being returned. If M is constant (e.g. always asking for the first 10 elements with LIMIT), you can consider it O(log(N)).
    /// Replaced By: `ZRANGE` with the `BYSCORE` argument
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @sortedset
    /// * @slow
    #[deprecated]
    fn zrangebyscore<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0, min: f64, max: f64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZRANGEBYSCORE");
            rv.arg(key);
            rv.arg(min);
            rv.arg(max);
            rv.query_async(self).await
        })
    }

    /// ZRANGESTORE
    /// 
    /// Store a range of members from sorted set into another key
    /// 
    /// Since: Redis 6.2.0
    /// Group: SortedSet
    /// Complexity: O(log(N)+M) with N being the number of elements in the sorted set and M the number of elements stored into the destination key.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// ACL Categories:
    /// * @write
    /// * @sortedset
    /// * @slow
    fn zrangestore<'a, K0: ToRedisArgs + Send + Sync + 'a, K1: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(dst: K0, src: K1, min: T0, max: T1) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZRANGESTORE");
            rv.arg(dst);
            rv.arg(src);
            rv.arg(min);
            rv.arg(max);
            rv.query_async(self).await
        })
    }

    /// ZRANK
    /// 
    /// Determine the index of a member in a sorted set
    /// 
    /// Since: Redis 2.0.0
    /// Group: SortedSet
    /// Complexity: O(log(N))
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @read
    /// * @sortedset
    /// * @fast
    fn zrank<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, member: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZRANK");
            rv.arg(key);
            rv.arg(member);
            rv.query_async(self).await
        })
    }

    /// ZREM
    /// 
    /// Remove one or more members from a sorted set
    /// 
    /// Since: Redis 1.2.0
    /// Group: SortedSet
    /// Complexity: O(M*log(N)) with N being the number of elements in the sorted set and M the number of elements to be removed.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @sortedset
    /// * @fast
    fn zrem<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, member: &'a [T0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZREM");
            rv.arg(key);
            rv.arg(member);
            rv.query_async(self).await
        })
    }

    /// ZREMRANGEBYLEX
    /// 
    /// Remove all members in a sorted set between the given lexicographical range
    /// 
    /// Since: Redis 2.8.9
    /// Group: SortedSet
    /// Complexity: O(log(N)+M) with N being the number of elements in the sorted set and M the number of elements removed by the operation.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// ACL Categories:
    /// * @write
    /// * @sortedset
    /// * @slow
    fn zremrangebylex<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(key: K0, min: T0, max: T1) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZREMRANGEBYLEX");
            rv.arg(key);
            rv.arg(min);
            rv.arg(max);
            rv.query_async(self).await
        })
    }

    /// ZREMRANGEBYLEX
    /// 
    /// Remove all members in a sorted set between the given lexicographical range
    /// 
    /// Since: Redis 2.8.9
    /// Group: SortedSet
    /// Complexity: O(log(N)+M) with N being the number of elements in the sorted set and M the number of elements removed by the operation.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// ACL Categories:
    /// * @write
    /// * @sortedset
    /// * @slow
    fn zrembylex<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(key: K0, min: T0, max: T1) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZREMRANGEBYLEX");
            rv.arg(key);
            rv.arg(min);
            rv.arg(max);
            rv.query_async(self).await
        })
    }

    /// ZREMRANGEBYRANK
    /// 
    /// Remove all members in a sorted set within the given indexes
    /// 
    /// Since: Redis 2.0.0
    /// Group: SortedSet
    /// Complexity: O(log(N)+M) with N being the number of elements in the sorted set and M the number of elements removed by the operation.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// ACL Categories:
    /// * @write
    /// * @sortedset
    /// * @slow
    fn zremrangebyrank<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0, start: i64, stop: i64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZREMRANGEBYRANK");
            rv.arg(key);
            rv.arg(start);
            rv.arg(stop);
            rv.query_async(self).await
        })
    }

    /// ZREMRANGEBYSCORE
    /// 
    /// Remove all members in a sorted set within the given scores
    /// 
    /// Since: Redis 1.2.0
    /// Group: SortedSet
    /// Complexity: O(log(N)+M) with N being the number of elements in the sorted set and M the number of elements removed by the operation.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// ACL Categories:
    /// * @write
    /// * @sortedset
    /// * @slow
    fn zremrangebyscore<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0, min: f64, max: f64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZREMRANGEBYSCORE");
            rv.arg(key);
            rv.arg(min);
            rv.arg(max);
            rv.query_async(self).await
        })
    }

    /// ZREVRANGE
    /// 
    /// Return a range of members in a sorted set, by index, with scores ordered from high to low
    /// 
    /// Since: Redis 1.2.0
    /// Group: SortedSet
    /// Replaced By: `ZRANGE` with the `REV` argument
    /// Complexity: O(log(N)+M) with N being the number of elements in the sorted set and M the number of elements returned.
    /// Replaced By: `ZRANGE` with the `REV` argument
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @sortedset
    /// * @slow
    #[deprecated]
    fn zrevrange<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0, start: i64, stop: i64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZREVRANGE");
            rv.arg(key);
            rv.arg(start);
            rv.arg(stop);
            rv.query_async(self).await
        })
    }

    /// ZREVRANGEBYLEX
    /// 
    /// Return a range of members in a sorted set, by lexicographical range, ordered from higher to lower strings.
    /// 
    /// Since: Redis 2.8.9
    /// Group: SortedSet
    /// Replaced By: `ZRANGE` with the `REV` and `BYLEX` arguments
    /// Complexity: O(log(N)+M) with N being the number of elements in the sorted set and M the number of elements being returned. If M is constant (e.g. always asking for the first 10 elements with LIMIT), you can consider it O(log(N)).
    /// Replaced By: `ZRANGE` with the `REV` and `BYLEX` arguments
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @sortedset
    /// * @slow
    #[deprecated]
    fn zrevrangebylex<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(key: K0, max: T0, min: T1) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZREVRANGEBYLEX");
            rv.arg(key);
            rv.arg(max);
            rv.arg(min);
            rv.query_async(self).await
        })
    }

    /// ZREVRANGEBYSCORE
    /// 
    /// Return a range of members in a sorted set, by score, with scores ordered from high to low
    /// 
    /// Since: Redis 2.2.0
    /// Group: SortedSet
    /// Replaced By: `ZRANGE` with the `REV` and `BYSCORE` arguments
    /// Complexity: O(log(N)+M) with N being the number of elements in the sorted set and M the number of elements being returned. If M is constant (e.g. always asking for the first 10 elements with LIMIT), you can consider it O(log(N)).
    /// Replaced By: `ZRANGE` with the `REV` and `BYSCORE` arguments
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @sortedset
    /// * @slow
    #[deprecated]
    fn zrevrangebyscore<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0, max: f64, min: f64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZREVRANGEBYSCORE");
            rv.arg(key);
            rv.arg(max);
            rv.arg(min);
            rv.query_async(self).await
        })
    }

    /// ZREVRANK
    /// 
    /// Determine the index of a member in a sorted set, with scores ordered from high to low
    /// 
    /// Since: Redis 2.0.0
    /// Group: SortedSet
    /// Complexity: O(log(N))
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @read
    /// * @sortedset
    /// * @fast
    fn zrevrank<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, member: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZREVRANK");
            rv.arg(key);
            rv.arg(member);
            rv.query_async(self).await
        })
    }

    /// ZSCORE
    /// 
    /// Get the score associated with the given member in a sorted set
    /// 
    /// Since: Redis 1.2.0
    /// Group: SortedSet
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @read
    /// * @sortedset
    /// * @fast
    fn zscore<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, member: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZSCORE");
            rv.arg(key);
            rv.arg(member);
            rv.query_async(self).await
        })
    }

    /// ZUNION
    /// 
    /// Add multiple sorted sets
    /// 
    /// Since: Redis 6.2.0
    /// Group: SortedSet
    /// Complexity: O(N)+O(M*log(M)) with N being the sum of the sizes of the input sorted sets, and M being the number of elements in the resulting sorted set.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Movablekeys: This first key, last key, and step values don't determine all key positions. Clients need to use COMMAND GETKEYS or key specifications in this case. See below for more details.
    /// ACL Categories:
    /// * @read
    /// * @sortedset
    /// * @slow
    fn zunion<'a, K0: ToRedisArgs + Send + Sync + 'a>(numkeys: i64, key: &'a [K0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZUNION");
            rv.arg(numkeys);
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// ZUNIONSTORE
    /// 
    /// Add multiple sorted sets and store the resulting sorted set in a new key
    /// 
    /// Since: Redis 2.0.0
    /// Group: SortedSet
    /// Complexity: O(N)+O(M log(M)) with N being the sum of the sizes of the input sorted sets, and M being the number of elements in the resulting sorted set.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Movablekeys: This first key, last key, and step values don't determine all key positions. Clients need to use COMMAND GETKEYS or key specifications in this case. See below for more details.
    /// ACL Categories:
    /// * @write
    /// * @sortedset
    /// * @slow
    fn zunionstore<'a, K0: ToRedisArgs + Send + Sync + 'a, K1: ToRedisArgs + Send + Sync + 'a>(destination: K0, numkeys: i64, key: &'a [K1]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ZUNIONSTORE");
            rv.arg(destination);
            rv.arg(numkeys);
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// HDEL
    /// 
    /// Delete one or more hash fields
    /// 
    /// Since: Redis 2.0.0
    /// Group: Hash
    /// Complexity: O(N) where N is the number of fields to be removed.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @hash
    /// * @fast
    fn hdel<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, field: &'a [T0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("HDEL");
            rv.arg(key);
            rv.arg(field);
            rv.query_async(self).await
        })
    }

    /// HEXISTS
    /// 
    /// Determine if a hash field exists
    /// 
    /// Since: Redis 2.0.0
    /// Group: Hash
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @read
    /// * @hash
    /// * @fast
    fn hexists<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, field: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("HEXISTS");
            rv.arg(key);
            rv.arg(field);
            rv.query_async(self).await
        })
    }

    /// HGET
    /// 
    /// Get the value of a hash field
    /// 
    /// Since: Redis 2.0.0
    /// Group: Hash
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @read
    /// * @hash
    /// * @fast
    fn hget<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, field: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("HGET");
            rv.arg(key);
            rv.arg(field);
            rv.query_async(self).await
        })
    }

    /// HGETALL
    /// 
    /// Get all the fields and values in a hash
    /// 
    /// Since: Redis 2.0.0
    /// Group: Hash
    /// Complexity: O(N) where N is the size of the hash.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @hash
    /// * @slow
    fn hgetall<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("HGETALL");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// HINCRBY
    /// 
    /// Increment the integer value of a hash field by the given number
    /// 
    /// Since: Redis 2.0.0
    /// Group: Hash
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @hash
    /// * @fast
    fn hincrby<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, field: T0, increment: i64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("HINCRBY");
            rv.arg(key);
            rv.arg(field);
            rv.arg(increment);
            rv.query_async(self).await
        })
    }

    /// HINCRBYFLOAT
    /// 
    /// Increment the float value of a hash field by the given amount
    /// 
    /// Since: Redis 2.6.0
    /// Group: Hash
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @hash
    /// * @fast
    fn hincrbyfloat<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, field: T0, increment: f64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("HINCRBYFLOAT");
            rv.arg(key);
            rv.arg(field);
            rv.arg(increment);
            rv.query_async(self).await
        })
    }

    /// HKEYS
    /// 
    /// Get all the fields in a hash
    /// 
    /// Since: Redis 2.0.0
    /// Group: Hash
    /// Complexity: O(N) where N is the size of the hash.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @hash
    /// * @slow
    fn hkeys<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("HKEYS");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// HLEN
    /// 
    /// Get the number of fields in a hash
    /// 
    /// Since: Redis 2.0.0
    /// Group: Hash
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @read
    /// * @hash
    /// * @fast
    fn hlen<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("HLEN");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// HMGET
    /// 
    /// Get the values of all the given hash fields
    /// 
    /// Since: Redis 2.0.0
    /// Group: Hash
    /// Complexity: O(N) where N is the number of fields being requested.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @read
    /// * @hash
    /// * @fast
    fn hmget<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, field: &'a [T0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("HMGET");
            rv.arg(key);
            rv.arg(field);
            rv.query_async(self).await
        })
    }

    /// HMSET
    /// 
    /// Set multiple hash fields to multiple values
    /// 
    /// Since: Redis 2.0.0
    /// Group: Hash
    /// Replaced By: `HSET` with multiple field-value pairs
    /// Complexity: O(N) where N is the number of fields being set.
    /// Replaced By: `HSET` with multiple field-value pairs
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @hash
    /// * @fast
    #[deprecated]
    fn hmset<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, field_value: &'a [T0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("HMSET");
            rv.arg(key);
            rv.arg(field_value);
            rv.query_async(self).await
        })
    }

    /// HRANDFIELD
    /// 
    /// Get one or multiple random fields from a hash
    /// 
    /// Since: Redis 6.2.0
    /// Group: Hash
    /// Complexity: O(N) where N is the number of fields returned
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @hash
    /// * @slow
    fn hrandfield<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, options: Option<T0>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("HRANDFIELD");
            rv.arg(key);
            rv.arg(options);
            rv.query_async(self).await
        })
    }

    /// HSET
    /// 
    /// Set the string value of a hash field
    /// 
    /// Since: Redis 2.0.0
    /// Group: Hash
    /// Complexity: O(1) for each field/value pair added, so O(N) to add N field/value pairs when the command is called with multiple field/value pairs.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @hash
    /// * @fast
    fn hset<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, field_value: &'a [T0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("HSET");
            rv.arg(key);
            rv.arg(field_value);
            rv.query_async(self).await
        })
    }

    /// HSETNX
    /// 
    /// Set the value of a hash field, only if the field does not exist
    /// 
    /// Since: Redis 2.0.0
    /// Group: Hash
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @hash
    /// * @fast
    fn hsetnx<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(key: K0, field: T0, value: T1) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("HSETNX");
            rv.arg(key);
            rv.arg(field);
            rv.arg(value);
            rv.query_async(self).await
        })
    }

    /// HSTRLEN
    /// 
    /// Get the length of the value of a hash field
    /// 
    /// Since: Redis 3.2.0
    /// Group: Hash
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @read
    /// * @hash
    /// * @fast
    fn hstrlen<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, field: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("HSTRLEN");
            rv.arg(key);
            rv.arg(field);
            rv.query_async(self).await
        })
    }

    /// HVALS
    /// 
    /// Get all the values in a hash
    /// 
    /// Since: Redis 2.0.0
    /// Group: Hash
    /// Complexity: O(N) where N is the size of the hash.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @hash
    /// * @slow
    fn hvals<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("HVALS");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// PSUBSCRIBE
    /// 
    /// Listen for messages published to channels matching the given patterns
    /// 
    /// Since: Redis 2.0.0
    /// Group: Pubsub
    /// Complexity: O(N) where N is the number of patterns the client is already subscribed to.
    /// CommandFlags:
    /// * Pubsub: This command is related to Redis Pub/Sub.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @pubsub
    /// * @slow
    fn psubscribe<'a, T0: ToRedisArgs + Send + Sync + 'a>(pattern: &'a [T0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("PSUBSCRIBE");
            rv.arg(pattern);
            rv.query_async(self).await
        })
    }

    /// PUBLISH
    /// 
    /// Post a message to a channel
    /// 
    /// Since: Redis 2.0.0
    /// Group: Pubsub
    /// Complexity: O(N+M) where N is the number of clients subscribed to the receiving channel and M is the total number of subscribed patterns (by any client).
    /// CommandFlags:
    /// * Pubsub: This command is related to Redis Pub/Sub.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @pubsub
    /// * @fast
    fn publish<'a, T0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(channel: T0, message: T1) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("PUBLISH");
            rv.arg(channel);
            rv.arg(message);
            rv.query_async(self).await
        })
    }

    /// PUBSUB
    /// 
    /// A container for Pub/Sub commands
    /// 
    /// Since: Redis 2.8.0
    /// Group: Pubsub
    /// Complexity: Depends on subcommand.
    /// ACL Categories:
    /// * @slow
    fn pubsub<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("PUBSUB");
            rv.query_async(self).await
        })
    }

    /// PUBSUB CHANNELS
    /// 
    /// List active channels
    /// 
    /// Since: Redis 2.8.0
    /// Group: Pubsub
    /// Complexity: O(N) where N is the number of active channels, and assuming constant time pattern matching (relatively short channels and patterns)
    /// CommandFlags:
    /// * Pubsub: This command is related to Redis Pub/Sub.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @pubsub
    /// * @slow
    fn pubsub_channels<'a, K0: ToRedisArgs + Send + Sync + 'a>(pattern: Option<K0>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("PUBSUB CHANNELS");
            rv.arg(pattern);
            rv.query_async(self).await
        })
    }

    /// PUBSUB HELP
    /// 
    /// Show helpful text about the different subcommands
    /// 
    /// Since: Redis 6.2.0
    /// Group: Pubsub
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    fn pubsub_help<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("PUBSUB HELP");
            rv.query_async(self).await
        })
    }

    /// PUBSUB NUMPAT
    /// 
    /// Get the count of unique patterns pattern subscriptions
    /// 
    /// Since: Redis 2.8.0
    /// Group: Pubsub
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Pubsub: This command is related to Redis Pub/Sub.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @pubsub
    /// * @slow
    fn pubsub_numpat<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("PUBSUB NUMPAT");
            rv.query_async(self).await
        })
    }

    /// PUBSUB NUMSUB
    /// 
    /// Get the count of subscribers for channels
    /// 
    /// Since: Redis 2.8.0
    /// Group: Pubsub
    /// Complexity: O(N) for the NUMSUB subcommand, where N is the number of requested channels
    /// CommandFlags:
    /// * Pubsub: This command is related to Redis Pub/Sub.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @pubsub
    /// * @slow
    fn pubsub_numsub<'a, T0: ToRedisArgs + Send + Sync + 'a>(channel: Option<&'a [T0]>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("PUBSUB NUMSUB");
            rv.arg(channel);
            rv.query_async(self).await
        })
    }

    /// PUBSUB SHARDCHANNELS
    /// 
    /// List active shard channels
    /// 
    /// Since: Redis 7.0.0
    /// Group: Pubsub
    /// Complexity: O(N) where N is the number of active shard channels, and assuming constant time pattern matching (relatively short shard channels).
    /// CommandFlags:
    /// * Pubsub: This command is related to Redis Pub/Sub.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @pubsub
    /// * @slow
    fn pubsub_shardchannels<'a, K0: ToRedisArgs + Send + Sync + 'a>(pattern: Option<K0>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("PUBSUB SHARDCHANNELS");
            rv.arg(pattern);
            rv.query_async(self).await
        })
    }

    /// PUBSUB SHARDNUMSUB
    /// 
    /// Get the count of subscribers for shard channels
    /// 
    /// Since: Redis 7.0.0
    /// Group: Pubsub
    /// Complexity: O(N) for the SHARDNUMSUB subcommand, where N is the number of requested shard channels
    /// CommandFlags:
    /// * Pubsub: This command is related to Redis Pub/Sub.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @pubsub
    /// * @slow
    fn pubsub_shardnumsub<'a, T0: ToRedisArgs + Send + Sync + 'a>(shardchannel: Option<&'a [T0]>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("PUBSUB SHARDNUMSUB");
            rv.arg(shardchannel);
            rv.query_async(self).await
        })
    }

    /// PUNSUBSCRIBE
    /// 
    /// Stop listening for messages posted to channels matching the given patterns
    /// 
    /// Since: Redis 2.0.0
    /// Group: Pubsub
    /// Complexity: O(N+M) where N is the number of patterns the client is already subscribed and M is the number of total patterns subscribed in the system (by any client).
    /// CommandFlags:
    /// * Pubsub: This command is related to Redis Pub/Sub.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @pubsub
    /// * @slow
    fn punsubscribe<'a, K0: ToRedisArgs + Send + Sync + 'a>(pattern: Option<&'a [K0]>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("PUNSUBSCRIBE");
            rv.arg(pattern);
            rv.query_async(self).await
        })
    }

    /// SPUBLISH
    /// 
    /// Post a message to a shard channel
    /// 
    /// Since: Redis 7.0.0
    /// Group: Pubsub
    /// Complexity: O(N) where N is the number of clients subscribed to the receiving shard channel.
    /// CommandFlags:
    /// * Pubsub: This command is related to Redis Pub/Sub.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @pubsub
    /// * @fast
    fn spublish<'a, T0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(shardchannel: T0, message: T1) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SPUBLISH");
            rv.arg(shardchannel);
            rv.arg(message);
            rv.query_async(self).await
        })
    }

    /// SSUBSCRIBE
    /// 
    /// Listen for messages published to the given shard channels
    /// 
    /// Since: Redis 7.0.0
    /// Group: Pubsub
    /// Complexity: O(N) where N is the number of shard channels to subscribe to.
    /// CommandFlags:
    /// * Pubsub: This command is related to Redis Pub/Sub.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @pubsub
    /// * @slow
    fn ssubscribe<'a, T0: ToRedisArgs + Send + Sync + 'a>(shardchannel: &'a [T0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SSUBSCRIBE");
            rv.arg(shardchannel);
            rv.query_async(self).await
        })
    }

    /// SUBSCRIBE
    /// 
    /// Listen for messages published to the given channels
    /// 
    /// Since: Redis 2.0.0
    /// Group: Pubsub
    /// Complexity: O(N) where N is the number of channels to subscribe to.
    /// CommandFlags:
    /// * Pubsub: This command is related to Redis Pub/Sub.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @pubsub
    /// * @slow
    fn subscribe<'a, T0: ToRedisArgs + Send + Sync + 'a>(channel: &'a [T0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SUBSCRIBE");
            rv.arg(channel);
            rv.query_async(self).await
        })
    }

    /// SUNSUBSCRIBE
    /// 
    /// Stop listening for messages posted to the given shard channels
    /// 
    /// Since: Redis 7.0.0
    /// Group: Pubsub
    /// Complexity: O(N) where N is the number of clients already subscribed to a shard channel.
    /// CommandFlags:
    /// * Pubsub: This command is related to Redis Pub/Sub.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @pubsub
    /// * @slow
    fn sunsubscribe<'a, T0: ToRedisArgs + Send + Sync + 'a>(shardchannel: Option<&'a [T0]>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SUNSUBSCRIBE");
            rv.arg(shardchannel);
            rv.query_async(self).await
        })
    }

    /// UNSUBSCRIBE
    /// 
    /// Stop listening for messages posted to the given channels
    /// 
    /// Since: Redis 2.0.0
    /// Group: Pubsub
    /// Complexity: O(N) where N is the number of clients already subscribed to a channel.
    /// CommandFlags:
    /// * Pubsub: This command is related to Redis Pub/Sub.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @pubsub
    /// * @slow
    fn unsubscribe<'a, T0: ToRedisArgs + Send + Sync + 'a>(channel: Option<&'a [T0]>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("UNSUBSCRIBE");
            rv.arg(channel);
            rv.query_async(self).await
        })
    }

    /// DISCARD
    /// 
    /// Discard all commands issued after MULTI
    /// 
    /// Since: Redis 2.0.0
    /// Group: Transactions
    /// Complexity: O(N), when N is the number of queued commands
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// * AllowBusy: From https://redis.io/docs/reference/modules/modules-api-ref/: Permit the command while the server is blocked either by a script or by a slow module command, see RM_Yield.
    /// ACL Categories:
    /// * @fast
    /// * @transaction
    fn discard<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("DISCARD");
            rv.query_async(self).await
        })
    }

    /// EXEC
    /// 
    /// Execute all commands issued after MULTI
    /// 
    /// Since: Redis 1.2.0
    /// Group: Transactions
    /// Complexity: Depends on commands in the transaction
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * SkipSlowlog: This command is not shown in SLOWLOG's output. As of Redis 7.0, this flag is a command tip.
    /// ACL Categories:
    /// * @slow
    /// * @transaction
    fn exec<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("EXEC");
            rv.query_async(self).await
        })
    }

    /// MULTI
    /// 
    /// Mark the start of a transaction block
    /// 
    /// Since: Redis 1.2.0
    /// Group: Transactions
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// * AllowBusy: From https://redis.io/docs/reference/modules/modules-api-ref/: Permit the command while the server is blocked either by a script or by a slow module command, see RM_Yield.
    /// ACL Categories:
    /// * @fast
    /// * @transaction
    fn multi<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("MULTI");
            rv.query_async(self).await
        })
    }

    /// UNWATCH
    /// 
    /// Forget about all watched keys
    /// 
    /// Since: Redis 2.2.0
    /// Group: Transactions
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// * AllowBusy: From https://redis.io/docs/reference/modules/modules-api-ref/: Permit the command while the server is blocked either by a script or by a slow module command, see RM_Yield.
    /// ACL Categories:
    /// * @fast
    /// * @transaction
    fn unwatch<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("UNWATCH");
            rv.query_async(self).await
        })
    }

    /// WATCH
    /// 
    /// Watch the given keys to determine execution of the MULTI/EXEC block
    /// 
    /// Since: Redis 2.2.0
    /// Group: Transactions
    /// Complexity: O(1) for every key.
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// * AllowBusy: From https://redis.io/docs/reference/modules/modules-api-ref/: Permit the command while the server is blocked either by a script or by a slow module command, see RM_Yield.
    /// ACL Categories:
    /// * @fast
    /// * @transaction
    fn watch<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: &'a [K0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("WATCH");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// AUTH
    /// 
    /// Authenticate to the server
    /// 
    /// Since: Redis 1.0.0
    /// Group: Connection
    /// Complexity: O(N) where N is the number of passwords defined for the user
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// * NoAuth: Thiscuting the command doesn't require authentication.
    /// * AllowBusy: From https://redis.io/docs/reference/modules/modules-api-ref/: Permit the command while the server is blocked either by a script or by a slow module command, see RM_Yield.
    /// ACL Categories:
    /// * @fast
    /// * @connection
    fn auth<'a, T0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(username: Option<T0>, password: T1) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("AUTH");
            rv.arg(username);
            rv.arg(password);
            rv.query_async(self).await
        })
    }

    /// CLIENT
    /// 
    /// A container for client connection commands
    /// 
    /// Since: Redis 2.4.0
    /// Group: Connection
    /// Complexity: Depends on subcommand.
    /// ACL Categories:
    /// * @slow
    fn client<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLIENT");
            rv.query_async(self).await
        })
    }

    /// CLIENT CACHING
    /// 
    /// Instruct the server about tracking or not keys in the next request
    /// 
    /// Since: Redis 6.0.0
    /// Group: Connection
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    /// * @connection
    fn client_caching<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLIENT CACHING");
            rv.query_async(self).await
        })
    }

    /// CLIENT GETNAME
    /// 
    /// Get the current connection name
    /// 
    /// Since: Redis 2.6.9
    /// Group: Connection
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    /// * @connection
    fn client_getname<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLIENT GETNAME");
            rv.query_async(self).await
        })
    }

    /// CLIENT GETREDIR
    /// 
    /// Get tracking notifications redirection client ID if any
    /// 
    /// Since: Redis 6.0.0
    /// Group: Connection
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    /// * @connection
    fn client_getredir<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLIENT GETREDIR");
            rv.query_async(self).await
        })
    }

    /// CLIENT HELP
    /// 
    /// Show helpful text about the different subcommands
    /// 
    /// Since: Redis 5.0.0
    /// Group: Connection
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    /// * @connection
    fn client_help<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLIENT HELP");
            rv.query_async(self).await
        })
    }

    /// CLIENT ID
    /// 
    /// Returns the client ID for the current connection
    /// 
    /// Since: Redis 5.0.0
    /// Group: Connection
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    /// * @connection
    fn client_id<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLIENT ID");
            rv.query_async(self).await
        })
    }

    /// CLIENT INFO
    /// 
    /// Returns information about the current client connection.
    /// 
    /// Since: Redis 6.2.0
    /// Group: Connection
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    /// * @connection
    fn client_info<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLIENT INFO");
            rv.query_async(self).await
        })
    }

    /// CLIENT LIST
    /// 
    /// Get the list of client connections
    /// 
    /// Since: Redis 2.4.0
    /// Group: Connection
    /// Complexity: O(N) where N is the number of client connections
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    /// * @connection
    fn client_list<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLIENT LIST");
            rv.query_async(self).await
        })
    }

    /// CLIENT NO-EVICT
    /// 
    /// Set client eviction mode for the current connection
    /// 
    /// Since: Redis 7.0.0
    /// Group: Connection
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    /// * @connection
    fn client_no_evict<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLIENT NO-EVICT");
            rv.query_async(self).await
        })
    }

    /// CLIENT PAUSE
    /// 
    /// Stop processing commands from clients for some time
    /// 
    /// Since: Redis 2.9.50
    /// Group: Connection
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    /// * @connection
    fn client_pause<'a>(timeout: i64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLIENT PAUSE");
            rv.arg(timeout);
            rv.query_async(self).await
        })
    }

    /// CLIENT REPLY
    /// 
    /// Instruct the server whether to reply to commands
    /// 
    /// Since: Redis 3.2.0
    /// Group: Connection
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    /// * @connection
    fn client_reply<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLIENT REPLY");
            rv.query_async(self).await
        })
    }

    /// CLIENT SETNAME
    /// 
    /// Set the current connection name
    /// 
    /// Since: Redis 2.6.9
    /// Group: Connection
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    /// * @connection
    fn client_setname<'a, T0: ToRedisArgs + Send + Sync + 'a>(connection_name: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLIENT SETNAME");
            rv.arg(connection_name);
            rv.query_async(self).await
        })
    }

    /// CLIENT TRACKING
    /// 
    /// Enable or disable server assisted client side caching support
    /// 
    /// Since: Redis 6.0.0
    /// Group: Connection
    /// Complexity: O(1). Some options may introduce additional complexity.
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    /// * @connection
    fn client_tracking<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLIENT TRACKING");
            rv.query_async(self).await
        })
    }

    /// CLIENT TRACKINGINFO
    /// 
    /// Return information about server assisted client side caching for the current connection
    /// 
    /// Since: Redis 6.2.0
    /// Group: Connection
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    /// * @connection
    fn client_trackinginfo<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLIENT TRACKINGINFO");
            rv.query_async(self).await
        })
    }

    /// CLIENT UNBLOCK
    /// 
    /// Unblock a client blocked in a blocking command from a different connection
    /// 
    /// Since: Redis 5.0.0
    /// Group: Connection
    /// Complexity: O(log N) where N is the number of client connections
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    /// * @connection
    fn client_unblock<'a>(client_id: i64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLIENT UNBLOCK");
            rv.arg(client_id);
            rv.query_async(self).await
        })
    }

    /// CLIENT UNPAUSE
    /// 
    /// Resume processing of clients that were paused
    /// 
    /// Since: Redis 6.2.0
    /// Group: Connection
    /// Complexity: O(N) Where N is the number of paused clients
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    /// * @connection
    fn client_unpause<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLIENT UNPAUSE");
            rv.query_async(self).await
        })
    }

    /// ECHO
    /// 
    /// Echo the given string
    /// 
    /// Since: Redis 1.0.0
    /// Group: Connection
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @fast
    /// * @connection
    fn echo<'a, T0: ToRedisArgs + Send + Sync + 'a>(message: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ECHO");
            rv.arg(message);
            rv.query_async(self).await
        })
    }

    /// HELLO
    /// 
    /// Handshake with Redis
    /// 
    /// Since: Redis 6.0.0
    /// Group: Connection
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// * NoAuth: Thiscuting the command doesn't require authentication.
    /// * AllowBusy: From https://redis.io/docs/reference/modules/modules-api-ref/: Permit the command while the server is blocked either by a script or by a slow module command, see RM_Yield.
    /// ACL Categories:
    /// * @fast
    /// * @connection
    fn hello<'a, T0: ToRedisArgs + Send + Sync + 'a>(arguments: Option<T0>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("HELLO");
            rv.arg(arguments);
            rv.query_async(self).await
        })
    }

    /// PING
    /// 
    /// Ping the server
    /// 
    /// Since: Redis 1.0.0
    /// Group: Connection
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @fast
    /// * @connection
    fn ping<'a, T0: ToRedisArgs + Send + Sync + 'a>(message: Option<T0>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("PING");
            rv.arg(message);
            rv.query_async(self).await
        })
    }

    /// QUIT
    /// 
    /// Close the connection
    /// 
    /// Since: Redis 1.0.0
    /// Group: Connection
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// * NoAuth: Thiscuting the command doesn't require authentication.
    /// * AllowBusy: From https://redis.io/docs/reference/modules/modules-api-ref/: Permit the command while the server is blocked either by a script or by a slow module command, see RM_Yield.
    /// ACL Categories:
    /// * @fast
    /// * @connection
    fn quit<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("QUIT");
            rv.query_async(self).await
        })
    }

    /// RESET
    /// 
    /// Reset the connection
    /// 
    /// Since: Redis 6.2.0
    /// Group: Connection
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// * NoAuth: Thiscuting the command doesn't require authentication.
    /// * AllowBusy: From https://redis.io/docs/reference/modules/modules-api-ref/: Permit the command while the server is blocked either by a script or by a slow module command, see RM_Yield.
    /// ACL Categories:
    /// * @fast
    /// * @connection
    fn reset<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("RESET");
            rv.query_async(self).await
        })
    }

    /// SELECT
    /// 
    /// Change the selected database for the current connection
    /// 
    /// Since: Redis 1.0.0
    /// Group: Connection
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @fast
    /// * @connection
    fn select<'a>(index: i64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SELECT");
            rv.arg(index);
            rv.query_async(self).await
        })
    }

    /// ACL
    /// 
    /// A container for Access List Control commands 
    /// 
    /// Since: Redis 6.0.0
    /// Group: Server
    /// Complexity: Depends on subcommand.
    /// ACL Categories:
    /// * @slow
    #[cfg(feature = "acl")]
    #[cfg_attr(docsrs, doc(cfg(feature = "acl")))]
    fn acl<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ACL");
            rv.query_async(self).await
        })
    }

    /// ACL CAT
    /// 
    /// List the ACL categories or the commands inside a category
    /// 
    /// Since: Redis 6.0.0
    /// Group: Server
    /// Complexity: O(1) since the categories and commands are a fixed set.
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    #[cfg(feature = "acl")]
    #[cfg_attr(docsrs, doc(cfg(feature = "acl")))]
    fn acl_cat<'a, T0: ToRedisArgs + Send + Sync + 'a>(categoryname: Option<T0>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ACL CAT");
            rv.arg(categoryname);
            rv.query_async(self).await
        })
    }

    /// ACL DELUSER
    /// 
    /// Remove the specified ACL users and the associated rules
    /// 
    /// Since: Redis 6.0.0
    /// Group: Server
    /// Complexity: O(1) amortized time considering the typical user.
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    #[cfg(feature = "acl")]
    #[cfg_attr(docsrs, doc(cfg(feature = "acl")))]
    fn acl_deluser<'a, T0: ToRedisArgs + Send + Sync + 'a>(username: &'a [T0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ACL DELUSER");
            rv.arg(username);
            rv.query_async(self).await
        })
    }

    /// ACL DRYRUN
    /// 
    /// Returns whether the user can execute the given command without executing the command.
    /// 
    /// Since: Redis 7.0.0
    /// Group: Server
    /// Complexity: O(1).
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    #[cfg(feature = "acl")]
    #[cfg_attr(docsrs, doc(cfg(feature = "acl")))]
    fn acl_dryrun<'a, T0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a, T2: ToRedisArgs + Send + Sync + 'a>(username: T0, command: T1, arg: Option<&'a [T2]>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ACL DRYRUN");
            rv.arg(username);
            rv.arg(command);
            rv.arg(arg);
            rv.query_async(self).await
        })
    }

    /// ACL GENPASS
    /// 
    /// Generate a pseudorandom secure password to use for ACL users
    /// 
    /// Since: Redis 6.0.0
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    #[cfg(feature = "acl")]
    #[cfg_attr(docsrs, doc(cfg(feature = "acl")))]
    fn acl_genpass<'a>(bits: Option<i64>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ACL GENPASS");
            rv.arg(bits);
            rv.query_async(self).await
        })
    }

    /// ACL GETUSER
    /// 
    /// Get the rules for a specific ACL user
    /// 
    /// Since: Redis 6.0.0
    /// Group: Server
    /// Complexity: O(N). Where N is the number of password, command and pattern rules that the user has.
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    #[cfg(feature = "acl")]
    #[cfg_attr(docsrs, doc(cfg(feature = "acl")))]
    fn acl_getuser<'a, T0: ToRedisArgs + Send + Sync + 'a>(username: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ACL GETUSER");
            rv.arg(username);
            rv.query_async(self).await
        })
    }

    /// ACL HELP
    /// 
    /// Show helpful text about the different subcommands
    /// 
    /// Since: Redis 6.0.0
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    #[cfg(feature = "acl")]
    #[cfg_attr(docsrs, doc(cfg(feature = "acl")))]
    fn acl_help<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ACL HELP");
            rv.query_async(self).await
        })
    }

    /// ACL LIST
    /// 
    /// List the current ACL rules in ACL config file format
    /// 
    /// Since: Redis 6.0.0
    /// Group: Server
    /// Complexity: O(N). Where N is the number of configured users.
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    #[cfg(feature = "acl")]
    #[cfg_attr(docsrs, doc(cfg(feature = "acl")))]
    fn acl_list<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ACL LIST");
            rv.query_async(self).await
        })
    }

    /// ACL LOAD
    /// 
    /// Reload the ACLs from the configured ACL file
    /// 
    /// Since: Redis 6.0.0
    /// Group: Server
    /// Complexity: O(N). Where N is the number of configured users.
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    #[cfg(feature = "acl")]
    #[cfg_attr(docsrs, doc(cfg(feature = "acl")))]
    fn acl_load<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ACL LOAD");
            rv.query_async(self).await
        })
    }

    /// ACL LOG
    /// 
    /// List latest events denied because of ACLs in place
    /// 
    /// Since: Redis 6.0.0
    /// Group: Server
    /// Complexity: O(N) with N being the number of entries shown.
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    #[cfg(feature = "acl")]
    #[cfg_attr(docsrs, doc(cfg(feature = "acl")))]
    fn acl_log<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ACL LOG");
            rv.query_async(self).await
        })
    }

    /// ACL SAVE
    /// 
    /// Save the current ACL rules in the configured ACL file
    /// 
    /// Since: Redis 6.0.0
    /// Group: Server
    /// Complexity: O(N). Where N is the number of configured users.
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    #[cfg(feature = "acl")]
    #[cfg_attr(docsrs, doc(cfg(feature = "acl")))]
    fn acl_save<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ACL SAVE");
            rv.query_async(self).await
        })
    }

    /// ACL SETUSER
    /// 
    /// Modify or create the rules for a specific ACL user
    /// 
    /// Since: Redis 6.0.0
    /// Group: Server
    /// Complexity: O(N). Where N is the number of rules provided.
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    #[cfg(feature = "acl")]
    #[cfg_attr(docsrs, doc(cfg(feature = "acl")))]
    fn acl_setuser<'a, T0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(username: T0, rule: Option<&'a [T1]>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ACL SETUSER");
            rv.arg(username);
            rv.arg(rule);
            rv.query_async(self).await
        })
    }

    /// ACL USERS
    /// 
    /// List the username of all the configured ACL rules
    /// 
    /// Since: Redis 6.0.0
    /// Group: Server
    /// Complexity: O(N). Where N is the number of configured users.
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    #[cfg(feature = "acl")]
    #[cfg_attr(docsrs, doc(cfg(feature = "acl")))]
    fn acl_users<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ACL USERS");
            rv.query_async(self).await
        })
    }

    /// ACL WHOAMI
    /// 
    /// Return the name of the user associated to the current connection
    /// 
    /// Since: Redis 6.0.0
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    #[cfg(feature = "acl")]
    #[cfg_attr(docsrs, doc(cfg(feature = "acl")))]
    fn acl_whoami<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ACL WHOAMI");
            rv.query_async(self).await
        })
    }

    /// BGREWRITEAOF
    /// 
    /// Asynchronously rewrite the append-only file
    /// 
    /// Since: Redis 1.0.0
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * NoAsyncLoading: This command is denied during asynchronous loading (that is when a replica uses disk-less SWAPDB SYNC, and allows access to the old dataset).
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn bgrewriteaof<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("BGREWRITEAOF");
            rv.query_async(self).await
        })
    }

    /// BGSAVE
    /// 
    /// Asynchronously save the dataset to disk
    /// 
    /// Since: Redis 1.0.0
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * NoAsyncLoading: This command is denied during asynchronous loading (that is when a replica uses disk-less SWAPDB SYNC, and allows access to the old dataset).
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn bgsave<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("BGSAVE");
            rv.query_async(self).await
        })
    }

    /// COMMAND
    /// 
    /// Get array of Redis command details
    /// 
    /// Since: Redis 2.8.13
    /// Group: Server
    /// Complexity: O(N) where N is the total number of Redis commands
    /// CommandFlags:
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    /// * @connection
    fn command<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("COMMAND");
            rv.query_async(self).await
        })
    }

    /// COMMAND COUNT
    /// 
    /// Get total number of Redis commands
    /// 
    /// Since: Redis 2.8.13
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    /// * @connection
    fn command_count<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("COMMAND COUNT");
            rv.query_async(self).await
        })
    }

    /// COMMAND DOCS
    /// 
    /// Get array of specific Redis command documentation
    /// 
    /// Since: Redis 7.0.0
    /// Group: Server
    /// Complexity: O(N) where N is the number of commands to look up
    /// CommandFlags:
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    /// * @connection
    fn command_docs<'a, T0: ToRedisArgs + Send + Sync + 'a>(command_name: Option<&'a [T0]>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("COMMAND DOCS");
            rv.arg(command_name);
            rv.query_async(self).await
        })
    }

    /// COMMAND GETKEYS
    /// 
    /// Extract keys given a full Redis command
    /// 
    /// Since: Redis 2.8.13
    /// Group: Server
    /// Complexity: O(N) where N is the number of arguments to the command
    /// CommandFlags:
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    /// * @connection
    fn command_getkeys<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("COMMAND GETKEYS");
            rv.query_async(self).await
        })
    }

    /// COMMAND GETKEYSANDFLAGS
    /// 
    /// Extract keys and access flags given a full Redis command
    /// 
    /// Since: Redis 7.0.0
    /// Group: Server
    /// Complexity: O(N) where N is the number of arguments to the command
    /// CommandFlags:
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    /// * @connection
    fn command_getkeysandflags<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("COMMAND GETKEYSANDFLAGS");
            rv.query_async(self).await
        })
    }

    /// COMMAND HELP
    /// 
    /// Show helpful text about the different subcommands
    /// 
    /// Since: Redis 5.0.0
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    /// * @connection
    fn command_help<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("COMMAND HELP");
            rv.query_async(self).await
        })
    }

    /// COMMAND INFO
    /// 
    /// Get array of specific Redis command details, or all when no argument is given.
    /// 
    /// Since: Redis 2.8.13
    /// Group: Server
    /// Complexity: O(N) where N is the number of commands to look up
    /// CommandFlags:
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    /// * @connection
    fn command_info<'a, T0: ToRedisArgs + Send + Sync + 'a>(command_name: Option<&'a [T0]>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("COMMAND INFO");
            rv.arg(command_name);
            rv.query_async(self).await
        })
    }

    /// COMMAND LIST
    /// 
    /// Get an array of Redis command names
    /// 
    /// Since: Redis 7.0.0
    /// Group: Server
    /// Complexity: O(N) where N is the total number of Redis commands
    /// CommandFlags:
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    /// * @connection
    fn command_list<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("COMMAND LIST");
            rv.query_async(self).await
        })
    }

    /// CONFIG
    /// 
    /// A container for server configuration commands
    /// 
    /// Since: Redis 2.0.0
    /// Group: Server
    /// Complexity: Depends on subcommand.
    /// ACL Categories:
    /// * @slow
    fn config<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CONFIG");
            rv.query_async(self).await
        })
    }

    /// CONFIG GET
    /// 
    /// Get the values of configuration parameters
    /// 
    /// Since: Redis 2.0.0
    /// Group: Server
    /// Complexity: O(N) when N is the number of configuration parameters provided
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn config_get<'a, T0: ToRedisArgs + Send + Sync + 'a>(parameter: &'a [T0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CONFIG GET");
            rv.arg(parameter);
            rv.query_async(self).await
        })
    }

    /// CONFIG HELP
    /// 
    /// Show helpful text about the different subcommands
    /// 
    /// Since: Redis 5.0.0
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    fn config_help<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CONFIG HELP");
            rv.query_async(self).await
        })
    }

    /// CONFIG RESETSTAT
    /// 
    /// Reset the stats returned by INFO
    /// 
    /// Since: Redis 2.0.0
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn config_resetstat<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CONFIG RESETSTAT");
            rv.query_async(self).await
        })
    }

    /// CONFIG REWRITE
    /// 
    /// Rewrite the configuration file with the in memory configuration
    /// 
    /// Since: Redis 2.8.0
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn config_rewrite<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CONFIG REWRITE");
            rv.query_async(self).await
        })
    }

    /// CONFIG SET
    /// 
    /// Set configuration parameters to the given values
    /// 
    /// Since: Redis 2.0.0
    /// Group: Server
    /// Complexity: O(N) when N is the number of configuration parameters provided
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn config_set<'a, T0: ToRedisArgs + Send + Sync + 'a>(parameter_value: &'a [T0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CONFIG SET");
            rv.arg(parameter_value);
            rv.query_async(self).await
        })
    }

    /// DBSIZE
    /// 
    /// Return the number of keys in the selected database
    /// 
    /// Since: Redis 1.0.0
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @keyspace
    /// * @read
    /// * @fast
    fn dbsize<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("DBSIZE");
            rv.query_async(self).await
        })
    }

    /// DEBUG
    /// 
    /// A container for debugging commands
    /// 
    /// Since: Redis 1.0.0
    /// Group: Server
    /// Complexity: Depends on subcommand.
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn debug<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("DEBUG");
            rv.query_async(self).await
        })
    }

    /// FAILOVER
    /// 
    /// Start a coordinated failover between this server and one of its replicas.
    /// 
    /// Since: Redis 6.2.0
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn failover<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("FAILOVER");
            rv.query_async(self).await
        })
    }

    /// FLUSHALL
    /// 
    /// Remove all keys from all databases
    /// 
    /// Since: Redis 1.0.0
    /// Group: Server
    /// Complexity: O(N) where N is the total number of keys in all databases
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// ACL Categories:
    /// * @keyspace
    /// * @write
    /// * @slow
    /// * @dangerous
    fn flushall<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("FLUSHALL");
            rv.query_async(self).await
        })
    }

    /// FLUSHDB
    /// 
    /// Remove all keys from the current database
    /// 
    /// Since: Redis 1.0.0
    /// Group: Server
    /// Complexity: O(N) where N is the number of keys in the selected database
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// ACL Categories:
    /// * @keyspace
    /// * @write
    /// * @slow
    /// * @dangerous
    fn flushdb<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("FLUSHDB");
            rv.query_async(self).await
        })
    }

    /// INFO
    /// 
    /// Get information and statistics about the server
    /// 
    /// Since: Redis 1.0.0
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    /// * @dangerous
    fn info<'a, T0: ToRedisArgs + Send + Sync + 'a>(section: Option<&'a [T0]>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("INFO");
            rv.arg(section);
            rv.query_async(self).await
        })
    }

    /// LASTSAVE
    /// 
    /// Get the UNIX time stamp of the last successful save to disk
    /// 
    /// Since: Redis 1.0.0
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @admin
    /// * @fast
    /// * @dangerous
    fn lastsave<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("LASTSAVE");
            rv.query_async(self).await
        })
    }

    /// LATENCY
    /// 
    /// A container for latency diagnostics commands
    /// 
    /// Since: Redis 2.8.13
    /// Group: Server
    /// Complexity: Depends on subcommand.
    /// ACL Categories:
    /// * @slow
    fn latency<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("LATENCY");
            rv.query_async(self).await
        })
    }

    /// LATENCY DOCTOR
    /// 
    /// Return a human readable latency analysis report.
    /// 
    /// Since: Redis 2.8.13
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn latency_doctor<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("LATENCY DOCTOR");
            rv.query_async(self).await
        })
    }

    /// LATENCY GRAPH
    /// 
    /// Return a latency graph for the event.
    /// 
    /// Since: Redis 2.8.13
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn latency_graph<'a, T0: ToRedisArgs + Send + Sync + 'a>(event: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("LATENCY GRAPH");
            rv.arg(event);
            rv.query_async(self).await
        })
    }

    /// LATENCY HELP
    /// 
    /// Show helpful text about the different subcommands.
    /// 
    /// Since: Redis 2.8.13
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    fn latency_help<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("LATENCY HELP");
            rv.query_async(self).await
        })
    }

    /// LATENCY HISTOGRAM
    /// 
    /// Return the cumulative distribution of latencies of a subset of commands or all.
    /// 
    /// Since: Redis 7.0.0
    /// Group: Server
    /// Complexity: O(N) where N is the number of commands with latency information being retrieved.
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn latency_histogram<'a, T0: ToRedisArgs + Send + Sync + 'a>(command: Option<&'a [T0]>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("LATENCY HISTOGRAM");
            rv.arg(command);
            rv.query_async(self).await
        })
    }

    /// LATENCY HISTORY
    /// 
    /// Return timestamp-latency samples for the event.
    /// 
    /// Since: Redis 2.8.13
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn latency_history<'a, T0: ToRedisArgs + Send + Sync + 'a>(event: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("LATENCY HISTORY");
            rv.arg(event);
            rv.query_async(self).await
        })
    }

    /// LATENCY LATEST
    /// 
    /// Return the latest latency samples for all events.
    /// 
    /// Since: Redis 2.8.13
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn latency_latest<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("LATENCY LATEST");
            rv.query_async(self).await
        })
    }

    /// LATENCY RESET
    /// 
    /// Reset latency data for one or more events.
    /// 
    /// Since: Redis 2.8.13
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn latency_reset<'a, T0: ToRedisArgs + Send + Sync + 'a>(event: Option<&'a [T0]>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("LATENCY RESET");
            rv.arg(event);
            rv.query_async(self).await
        })
    }

    /// LOLWUT
    /// 
    /// Display some computer art and the Redis version
    /// 
    /// Since: Redis 5.0.0
    /// Group: Server
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @read
    /// * @fast
    fn lolwut<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("LOLWUT");
            rv.query_async(self).await
        })
    }

    /// MEMORY
    /// 
    /// A container for memory diagnostics commands
    /// 
    /// Since: Redis 4.0.0
    /// Group: Server
    /// Complexity: Depends on subcommand.
    /// ACL Categories:
    /// * @slow
    fn memory<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("MEMORY");
            rv.query_async(self).await
        })
    }

    /// MEMORY DOCTOR
    /// 
    /// Outputs memory problems report
    /// 
    /// Since: Redis 4.0.0
    /// Group: Server
    /// Complexity: O(1)
    /// ACL Categories:
    /// * @slow
    fn memory_doctor<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("MEMORY DOCTOR");
            rv.query_async(self).await
        })
    }

    /// MEMORY HELP
    /// 
    /// Show helpful text about the different subcommands
    /// 
    /// Since: Redis 4.0.0
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    fn memory_help<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("MEMORY HELP");
            rv.query_async(self).await
        })
    }

    /// MEMORY MALLOC-STATS
    /// 
    /// Show allocator internal stats
    /// 
    /// Since: Redis 4.0.0
    /// Group: Server
    /// Complexity: Depends on how much memory is allocated, could be slow
    /// ACL Categories:
    /// * @slow
    fn memory_malloc_stats<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("MEMORY MALLOC-STATS");
            rv.query_async(self).await
        })
    }

    /// MEMORY PURGE
    /// 
    /// Ask the allocator to release memory
    /// 
    /// Since: Redis 4.0.0
    /// Group: Server
    /// Complexity: Depends on how much memory is allocated, could be slow
    /// ACL Categories:
    /// * @slow
    fn memory_purge<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("MEMORY PURGE");
            rv.query_async(self).await
        })
    }

    /// MEMORY STATS
    /// 
    /// Show memory usage details
    /// 
    /// Since: Redis 4.0.0
    /// Group: Server
    /// Complexity: O(1)
    /// ACL Categories:
    /// * @slow
    fn memory_stats<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("MEMORY STATS");
            rv.query_async(self).await
        })
    }

    /// MEMORY USAGE
    /// 
    /// Estimate the memory usage of a key
    /// 
    /// Since: Redis 4.0.0
    /// Group: Server
    /// Complexity: O(N) where N is the number of samples.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @slow
    fn memory_usage<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("MEMORY USAGE");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// MODULE
    /// 
    /// A container for module commands
    /// 
    /// Since: Redis 4.0.0
    /// Group: Server
    /// Complexity: Depends on subcommand.
    /// ACL Categories:
    /// * @slow
    fn module<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("MODULE");
            rv.query_async(self).await
        })
    }

    /// MODULE HELP
    /// 
    /// Show helpful text about the different subcommands
    /// 
    /// Since: Redis 5.0.0
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    fn module_help<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("MODULE HELP");
            rv.query_async(self).await
        })
    }

    /// MODULE LIST
    /// 
    /// List all modules loaded by the server
    /// 
    /// Since: Redis 4.0.0
    /// Group: Server
    /// Complexity: O(N) where N is the number of loaded modules.
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn module_list<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("MODULE LIST");
            rv.query_async(self).await
        })
    }

    /// MODULE LOAD
    /// 
    /// Load a module
    /// 
    /// Since: Redis 4.0.0
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * NoAsyncLoading: This command is denied during asynchronous loading (that is when a replica uses disk-less SWAPDB SYNC, and allows access to the old dataset).
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn module_load<'a, T0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(path: T0, arg: Option<&'a [T1]>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("MODULE LOAD");
            rv.arg(path);
            rv.arg(arg);
            rv.query_async(self).await
        })
    }

    /// MODULE LOADEX
    /// 
    /// Load a module with extended parameters
    /// 
    /// Since: Redis 7.0.0
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * NoAsyncLoading: This command is denied during asynchronous loading (that is when a replica uses disk-less SWAPDB SYNC, and allows access to the old dataset).
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn module_loadex<'a, T0: ToRedisArgs + Send + Sync + 'a>(path: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("MODULE LOADEX");
            rv.arg(path);
            rv.query_async(self).await
        })
    }

    /// MODULE UNLOAD
    /// 
    /// Unload a module
    /// 
    /// Since: Redis 4.0.0
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * NoAsyncLoading: This command is denied during asynchronous loading (that is when a replica uses disk-less SWAPDB SYNC, and allows access to the old dataset).
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn module_unload<'a, T0: ToRedisArgs + Send + Sync + 'a>(name: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("MODULE UNLOAD");
            rv.arg(name);
            rv.query_async(self).await
        })
    }

    /// MONITOR
    /// 
    /// Listen for all requests received by the server in real time
    /// 
    /// Since: Redis 1.0.0
    /// Group: Server
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn monitor<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("MONITOR");
            rv.query_async(self).await
        })
    }

    /// PSYNC
    /// 
    /// Internal command used for replication
    /// 
    /// Since: Redis 2.8.0
    /// Group: Server
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * NoAsyncLoading: This command is denied during asynchronous loading (that is when a replica uses disk-less SWAPDB SYNC, and allows access to the old dataset).
    /// * NoMulti: This command isn't allowed inside the context of a transaction.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn psync<'a, T0: ToRedisArgs + Send + Sync + 'a>(replicationid: T0, offset: i64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("PSYNC");
            rv.arg(replicationid);
            rv.arg(offset);
            rv.query_async(self).await
        })
    }

    /// REPLCONF
    /// 
    /// An internal command for configuring the replication stream
    /// 
    /// Since: Redis 3.0.0
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * AllowBusy: From https://redis.io/docs/reference/modules/modules-api-ref/: Permit the command while the server is blocked either by a script or by a slow module command, see RM_Yield.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn replconf<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("REPLCONF");
            rv.query_async(self).await
        })
    }

    /// REPLICAOF
    /// 
    /// Make the server a replica of another instance, or promote it as master.
    /// 
    /// Since: Redis 5.0.0
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * NoAsyncLoading: This command is denied during asynchronous loading (that is when a replica uses disk-less SWAPDB SYNC, and allows access to the old dataset).
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn replicaof<'a, T0: ToRedisArgs + Send + Sync + 'a>(host: T0, port: i64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("REPLICAOF");
            rv.arg(host);
            rv.arg(port);
            rv.query_async(self).await
        })
    }

    /// RESTORE-ASKING
    /// 
    /// An internal command for migrating keys in a cluster
    /// 
    /// Since: Redis 3.0.0
    /// Group: Server
    /// Complexity: O(1) to create the new key and additional O(N*M) to reconstruct the serialized value, where N is the number of Redis objects composing the value and M their average size. For small string values the time complexity is thus O(1)+O(1*M) where M is small, so simply O(1). However for sorted set values the complexity is O(N*M*log(N)) because inserting values into sorted sets is O(log(N)).
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Asking: This command is allowed even during hash slot migration. This flag is relevant in Redis Cluster deployments.
    /// ACL Categories:
    /// * @keyspace
    /// * @write
    /// * @slow
    /// * @dangerous
    fn restore_asking<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, ttl: i64, serialized_value: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("RESTORE-ASKING");
            rv.arg(key);
            rv.arg(ttl);
            rv.arg(serialized_value);
            rv.query_async(self).await
        })
    }

    /// ROLE
    /// 
    /// Return the role of the instance in the context of replication
    /// 
    /// Since: Redis 2.8.12
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @admin
    /// * @fast
    /// * @dangerous
    fn role<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ROLE");
            rv.query_async(self).await
        })
    }

    /// SAVE
    /// 
    /// Synchronously save the dataset to disk
    /// 
    /// Since: Redis 1.0.0
    /// Group: Server
    /// Complexity: O(N) where N is the total number of keys in all databases
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * NoAsyncLoading: This command is denied during asynchronous loading (that is when a replica uses disk-less SWAPDB SYNC, and allows access to the old dataset).
    /// * NoMulti: This command isn't allowed inside the context of a transaction.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn save<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SAVE");
            rv.query_async(self).await
        })
    }

    /// SHUTDOWN
    /// 
    /// Synchronously save the dataset to disk and then shut down the server
    /// 
    /// Since: Redis 1.0.0
    /// Group: Server
    /// Complexity: O(N) when saving, where N is the total number of keys in all databases when saving data, otherwise O(1)
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * NoMulti: This command isn't allowed inside the context of a transaction.
    /// * AllowBusy: From https://redis.io/docs/reference/modules/modules-api-ref/: Permit the command while the server is blocked either by a script or by a slow module command, see RM_Yield.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn shutdown<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SHUTDOWN");
            rv.query_async(self).await
        })
    }

    /// SLAVEOF
    /// 
    /// Make the server a replica of another instance, or promote it as master.
    /// 
    /// Since: Redis 1.0.0
    /// Group: Server
    /// Replaced By: `REPLICAOF`
    /// Complexity: O(1)
    /// Replaced By: `REPLICAOF`
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * NoAsyncLoading: This command is denied during asynchronous loading (that is when a replica uses disk-less SWAPDB SYNC, and allows access to the old dataset).
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    #[deprecated]
    fn slaveof<'a, T0: ToRedisArgs + Send + Sync + 'a>(host: T0, port: i64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SLAVEOF");
            rv.arg(host);
            rv.arg(port);
            rv.query_async(self).await
        })
    }

    /// SLOWLOG
    /// 
    /// A container for slow log commands
    /// 
    /// Since: Redis 2.2.12
    /// Group: Server
    /// Complexity: Depends on subcommand.
    /// ACL Categories:
    /// * @slow
    fn slowlog<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SLOWLOG");
            rv.query_async(self).await
        })
    }

    /// SLOWLOG GET
    /// 
    /// Get the slow log's entries
    /// 
    /// Since: Redis 2.2.12
    /// Group: Server
    /// Complexity: O(N) where N is the number of entries returned
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn slowlog_get<'a>(count: Option<i64>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SLOWLOG GET");
            rv.arg(count);
            rv.query_async(self).await
        })
    }

    /// SLOWLOG HELP
    /// 
    /// Show helpful text about the different subcommands
    /// 
    /// Since: Redis 6.2.0
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    fn slowlog_help<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SLOWLOG HELP");
            rv.query_async(self).await
        })
    }

    /// SLOWLOG LEN
    /// 
    /// Get the slow log's length
    /// 
    /// Since: Redis 2.2.12
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn slowlog_len<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SLOWLOG LEN");
            rv.query_async(self).await
        })
    }

    /// SLOWLOG RESET
    /// 
    /// Clear all entries from the slow log
    /// 
    /// Since: Redis 2.2.12
    /// Group: Server
    /// Complexity: O(N) where N is the number of entries in the slowlog
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn slowlog_reset<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SLOWLOG RESET");
            rv.query_async(self).await
        })
    }

    /// SWAPDB
    /// 
    /// Swaps two Redis databases
    /// 
    /// Since: Redis 4.0.0
    /// Group: Server
    /// Complexity: O(N) where N is the count of clients watching or blocking on keys from both databases.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @keyspace
    /// * @write
    /// * @fast
    /// * @dangerous
    fn swapdb<'a>(index1: i64, index2: i64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SWAPDB");
            rv.arg(index1);
            rv.arg(index2);
            rv.query_async(self).await
        })
    }

    /// SYNC
    /// 
    /// Internal command used for replication
    /// 
    /// Since: Redis 1.0.0
    /// Group: Server
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * NoAsyncLoading: This command is denied during asynchronous loading (that is when a replica uses disk-less SWAPDB SYNC, and allows access to the old dataset).
    /// * NoMulti: This command isn't allowed inside the context of a transaction.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn sync<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SYNC");
            rv.query_async(self).await
        })
    }

    /// TIME
    /// 
    /// Return the current server time
    /// 
    /// Since: Redis 2.6.0
    /// Group: Server
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @fast
    fn time<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("TIME");
            rv.query_async(self).await
        })
    }

    /// EVAL
    /// 
    /// Execute a Lua script server side
    /// 
    /// Since: Redis 2.6.0
    /// Group: Scripting
    /// Complexity: Depends on the script that is executed.
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * SkipMonitor: This command is not shown in MONITOR's output.
    /// * NoMandatoryKeys: This command may accept key name arguments, but these aren't mandatory.
    /// * Movablekeys: This first key, last key, and step values don't determine all key positions. Clients need to use COMMAND GETKEYS or key specifications in this case. See below for more details.
    /// ACL Categories:
    /// * @slow
    /// * @scripting
    fn eval<'a, T0: ToRedisArgs + Send + Sync + 'a, K0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(script: T0, numkeys: i64, key: Option<&'a [K0]>, arg: Option<&'a [T1]>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("EVAL");
            rv.arg(script);
            rv.arg(numkeys);
            rv.arg(key);
            rv.arg(arg);
            rv.query_async(self).await
        })
    }

    /// EVALSHA
    /// 
    /// Execute a Lua script server side
    /// 
    /// Since: Redis 2.6.0
    /// Group: Scripting
    /// Complexity: Depends on the script that is executed.
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * SkipMonitor: This command is not shown in MONITOR's output.
    /// * NoMandatoryKeys: This command may accept key name arguments, but these aren't mandatory.
    /// * Movablekeys: This first key, last key, and step values don't determine all key positions. Clients need to use COMMAND GETKEYS or key specifications in this case. See below for more details.
    /// ACL Categories:
    /// * @slow
    /// * @scripting
    fn evalsha<'a, T0: ToRedisArgs + Send + Sync + 'a, K0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(sha1: T0, numkeys: i64, key: Option<&'a [K0]>, arg: Option<&'a [T1]>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("EVALSHA");
            rv.arg(sha1);
            rv.arg(numkeys);
            rv.arg(key);
            rv.arg(arg);
            rv.query_async(self).await
        })
    }

    /// EVALSHA_RO
    /// 
    /// Execute a read-only Lua script server side
    /// 
    /// Since: Redis 7.0.0
    /// Group: Scripting
    /// Complexity: Depends on the script that is executed.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * SkipMonitor: This command is not shown in MONITOR's output.
    /// * NoMandatoryKeys: This command may accept key name arguments, but these aren't mandatory.
    /// * Movablekeys: This first key, last key, and step values don't determine all key positions. Clients need to use COMMAND GETKEYS or key specifications in this case. See below for more details.
    /// ACL Categories:
    /// * @slow
    /// * @scripting
    fn evalsha_ro<'a, T0: ToRedisArgs + Send + Sync + 'a, K0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(sha1: T0, numkeys: i64, key: &'a [K0], arg: &'a [T1]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("EVALSHA_RO");
            rv.arg(sha1);
            rv.arg(numkeys);
            rv.arg(key);
            rv.arg(arg);
            rv.query_async(self).await
        })
    }

    /// EVAL_RO
    /// 
    /// Execute a read-only Lua script server side
    /// 
    /// Since: Redis 7.0.0
    /// Group: Scripting
    /// Complexity: Depends on the script that is executed.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * SkipMonitor: This command is not shown in MONITOR's output.
    /// * NoMandatoryKeys: This command may accept key name arguments, but these aren't mandatory.
    /// * Movablekeys: This first key, last key, and step values don't determine all key positions. Clients need to use COMMAND GETKEYS or key specifications in this case. See below for more details.
    /// ACL Categories:
    /// * @slow
    /// * @scripting
    fn eval_ro<'a, T0: ToRedisArgs + Send + Sync + 'a, K0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(script: T0, numkeys: i64, key: &'a [K0], arg: &'a [T1]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("EVAL_RO");
            rv.arg(script);
            rv.arg(numkeys);
            rv.arg(key);
            rv.arg(arg);
            rv.query_async(self).await
        })
    }

    /// FCALL
    /// 
    /// Invoke a function
    /// 
    /// Since: Redis 7.0.0
    /// Group: Scripting
    /// Complexity: Depends on the function that is executed.
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * SkipMonitor: This command is not shown in MONITOR's output.
    /// * NoMandatoryKeys: This command may accept key name arguments, but these aren't mandatory.
    /// * Movablekeys: This first key, last key, and step values don't determine all key positions. Clients need to use COMMAND GETKEYS or key specifications in this case. See below for more details.
    /// ACL Categories:
    /// * @slow
    /// * @scripting
    fn fcall<'a, T0: ToRedisArgs + Send + Sync + 'a, K0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(function: T0, numkeys: i64, key: &'a [K0], arg: &'a [T1]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("FCALL");
            rv.arg(function);
            rv.arg(numkeys);
            rv.arg(key);
            rv.arg(arg);
            rv.query_async(self).await
        })
    }

    /// FCALL_RO
    /// 
    /// Invoke a read-only function
    /// 
    /// Since: Redis 7.0.0
    /// Group: Scripting
    /// Complexity: Depends on the function that is executed.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * SkipMonitor: This command is not shown in MONITOR's output.
    /// * NoMandatoryKeys: This command may accept key name arguments, but these aren't mandatory.
    /// * Movablekeys: This first key, last key, and step values don't determine all key positions. Clients need to use COMMAND GETKEYS or key specifications in this case. See below for more details.
    /// ACL Categories:
    /// * @slow
    /// * @scripting
    fn fcall_ro<'a, T0: ToRedisArgs + Send + Sync + 'a, K0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(function: T0, numkeys: i64, key: &'a [K0], arg: &'a [T1]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("FCALL_RO");
            rv.arg(function);
            rv.arg(numkeys);
            rv.arg(key);
            rv.arg(arg);
            rv.query_async(self).await
        })
    }

    /// FUNCTION
    /// 
    /// A container for function commands
    /// 
    /// Since: Redis 7.0.0
    /// Group: Scripting
    /// Complexity: Depends on subcommand.
    /// ACL Categories:
    /// * @slow
    fn function<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("FUNCTION");
            rv.query_async(self).await
        })
    }

    /// FUNCTION DELETE
    /// 
    /// Delete a function by name
    /// 
    /// Since: Redis 7.0.0
    /// Group: Scripting
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Noscript: This command can't be called from scripts or functions.
    /// ACL Categories:
    /// * @write
    /// * @slow
    /// * @scripting
    fn function_delete<'a, T0: ToRedisArgs + Send + Sync + 'a>(library_name: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("FUNCTION DELETE");
            rv.arg(library_name);
            rv.query_async(self).await
        })
    }

    /// FUNCTION DUMP
    /// 
    /// Dump all functions into a serialized binary payload
    /// 
    /// Since: Redis 7.0.0
    /// Group: Scripting
    /// Complexity: O(N) where N is the number of functions
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// ACL Categories:
    /// * @slow
    /// * @scripting
    fn function_dump<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("FUNCTION DUMP");
            rv.query_async(self).await
        })
    }

    /// FUNCTION FLUSH
    /// 
    /// Deleting all functions
    /// 
    /// Since: Redis 7.0.0
    /// Group: Scripting
    /// Complexity: O(N) where N is the number of functions deleted
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Noscript: This command can't be called from scripts or functions.
    /// ACL Categories:
    /// * @write
    /// * @slow
    /// * @scripting
    fn function_flush<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("FUNCTION FLUSH");
            rv.query_async(self).await
        })
    }

    /// FUNCTION HELP
    /// 
    /// Show helpful text about the different subcommands
    /// 
    /// Since: Redis 7.0.0
    /// Group: Scripting
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    /// * @scripting
    fn function_help<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("FUNCTION HELP");
            rv.query_async(self).await
        })
    }

    /// FUNCTION KILL
    /// 
    /// Kill the function currently in execution.
    /// 
    /// Since: Redis 7.0.0
    /// Group: Scripting
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// * AllowBusy: From https://redis.io/docs/reference/modules/modules-api-ref/: Permit the command while the server is blocked either by a script or by a slow module command, see RM_Yield.
    /// ACL Categories:
    /// * @slow
    /// * @scripting
    fn function_kill<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("FUNCTION KILL");
            rv.query_async(self).await
        })
    }

    /// FUNCTION LIST
    /// 
    /// List information about all the functions
    /// 
    /// Since: Redis 7.0.0
    /// Group: Scripting
    /// Complexity: O(N) where N is the number of functions
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// ACL Categories:
    /// * @slow
    /// * @scripting
    fn function_list<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("FUNCTION LIST");
            rv.query_async(self).await
        })
    }

    /// FUNCTION LOAD
    /// 
    /// Create a function with the given arguments (name, code, description)
    /// 
    /// Since: Redis 7.0.0
    /// Group: Scripting
    /// Complexity: O(1) (considering compilation time is redundant)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Noscript: This command can't be called from scripts or functions.
    /// ACL Categories:
    /// * @write
    /// * @slow
    /// * @scripting
    fn function_load<'a, T0: ToRedisArgs + Send + Sync + 'a>(function_code: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("FUNCTION LOAD");
            rv.arg(function_code);
            rv.query_async(self).await
        })
    }

    /// FUNCTION RESTORE
    /// 
    /// Restore all the functions on the given payload
    /// 
    /// Since: Redis 7.0.0
    /// Group: Scripting
    /// Complexity: O(N) where N is the number of functions on the payload
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Noscript: This command can't be called from scripts or functions.
    /// ACL Categories:
    /// * @write
    /// * @slow
    /// * @scripting
    fn function_restore<'a, T0: ToRedisArgs + Send + Sync + 'a>(serialized_value: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("FUNCTION RESTORE");
            rv.arg(serialized_value);
            rv.query_async(self).await
        })
    }

    /// FUNCTION STATS
    /// 
    /// Return information about the function currently running (name, description, duration)
    /// 
    /// Since: Redis 7.0.0
    /// Group: Scripting
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// * AllowBusy: From https://redis.io/docs/reference/modules/modules-api-ref/: Permit the command while the server is blocked either by a script or by a slow module command, see RM_Yield.
    /// ACL Categories:
    /// * @slow
    /// * @scripting
    fn function_stats<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("FUNCTION STATS");
            rv.query_async(self).await
        })
    }

    /// SCRIPT
    /// 
    /// A container for Lua scripts management commands
    /// 
    /// Since: Redis 2.6.0
    /// Group: Scripting
    /// Complexity: Depends on subcommand.
    /// ACL Categories:
    /// * @slow
    fn script<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SCRIPT");
            rv.query_async(self).await
        })
    }

    /// SCRIPT DEBUG
    /// 
    /// Set the debug mode for executed scripts.
    /// 
    /// Since: Redis 3.2.0
    /// Group: Scripting
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// ACL Categories:
    /// * @slow
    /// * @scripting
    fn script_debug<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SCRIPT DEBUG");
            rv.query_async(self).await
        })
    }

    /// SCRIPT EXISTS
    /// 
    /// Check existence of scripts in the script cache.
    /// 
    /// Since: Redis 2.6.0
    /// Group: Scripting
    /// Complexity: O(N) with N being the number of scripts to check (so checking a single script is an O(1) operation).
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// ACL Categories:
    /// * @slow
    /// * @scripting
    fn script_exists<'a, T0: ToRedisArgs + Send + Sync + 'a>(sha1: &'a [T0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SCRIPT EXISTS");
            rv.arg(sha1);
            rv.query_async(self).await
        })
    }

    /// SCRIPT FLUSH
    /// 
    /// Remove all the scripts from the script cache.
    /// 
    /// Since: Redis 2.6.0
    /// Group: Scripting
    /// Complexity: O(N) with N being the number of scripts in cache
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// ACL Categories:
    /// * @slow
    /// * @scripting
    fn script_flush<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SCRIPT FLUSH");
            rv.query_async(self).await
        })
    }

    /// SCRIPT HELP
    /// 
    /// Show helpful text about the different subcommands
    /// 
    /// Since: Redis 5.0.0
    /// Group: Scripting
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    /// * @scripting
    fn script_help<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SCRIPT HELP");
            rv.query_async(self).await
        })
    }

    /// SCRIPT KILL
    /// 
    /// Kill the script currently in execution.
    /// 
    /// Since: Redis 2.6.0
    /// Group: Scripting
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// * AllowBusy: From https://redis.io/docs/reference/modules/modules-api-ref/: Permit the command while the server is blocked either by a script or by a slow module command, see RM_Yield.
    /// ACL Categories:
    /// * @slow
    /// * @scripting
    fn script_kill<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SCRIPT KILL");
            rv.query_async(self).await
        })
    }

    /// SCRIPT LOAD
    /// 
    /// Load the specified Lua script into the script cache.
    /// 
    /// Since: Redis 2.6.0
    /// Group: Scripting
    /// Complexity: O(N) with N being the length in bytes of the script body.
    /// CommandFlags:
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    /// * @scripting
    fn script_load<'a, T0: ToRedisArgs + Send + Sync + 'a>(script: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SCRIPT LOAD");
            rv.arg(script);
            rv.query_async(self).await
        })
    }

    /// PFADD
    /// 
    /// Adds the specified elements to the specified HyperLogLog.
    /// 
    /// Since: Redis 2.8.9
    /// Group: Hyperloglog
    /// Complexity: O(1) to add every element.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @hyperloglog
    /// * @fast
    fn pfadd<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, element: Option<&'a [T0]>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("PFADD");
            rv.arg(key);
            rv.arg(element);
            rv.query_async(self).await
        })
    }

    /// PFCOUNT
    /// 
    /// Return the approximated cardinality of the set(s) observed by the HyperLogLog at key(s).
    /// 
    /// Since: Redis 2.8.9
    /// Group: Hyperloglog
    /// Complexity: O(1) with a very small average constant time when called with a single key. O(N) with N being the number of keys, and much bigger constant times, when called with multiple keys.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @hyperloglog
    /// * @slow
    fn pfcount<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: &'a [K0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("PFCOUNT");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// PFDEBUG
    /// 
    /// Internal commands for debugging HyperLogLog values
    /// 
    /// Since: Redis 2.8.9
    /// Group: Hyperloglog
    /// Complexity: N/A
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Admin: This command is an administrative command.
    /// ACL Categories:
    /// * @write
    /// * @hyperloglog
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn pfdebug<'a, T0: ToRedisArgs + Send + Sync + 'a, K0: ToRedisArgs + Send + Sync + 'a>(subcommand: T0, key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("PFDEBUG");
            rv.arg(subcommand);
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// PFMERGE
    /// 
    /// Merge N different HyperLogLogs into a single one.
    /// 
    /// Since: Redis 2.8.9
    /// Group: Hyperloglog
    /// Complexity: O(N) to merge N HyperLogLogs, but with high constant times.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// ACL Categories:
    /// * @write
    /// * @hyperloglog
    /// * @slow
    fn pfmerge<'a, K0: ToRedisArgs + Send + Sync + 'a, K1: ToRedisArgs + Send + Sync + 'a>(destkey: K0, sourcekey: &'a [K1]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("PFMERGE");
            rv.arg(destkey);
            rv.arg(sourcekey);
            rv.query_async(self).await
        })
    }

    /// PFSELFTEST
    /// 
    /// An internal command for testing HyperLogLog values
    /// 
    /// Since: Redis 2.8.9
    /// Group: Hyperloglog
    /// Complexity: N/A
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// ACL Categories:
    /// * @hyperloglog
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn pfselftest<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("PFSELFTEST");
            rv.query_async(self).await
        })
    }

    /// ASKING
    /// 
    /// Sent by cluster clients after an -ASK redirect
    /// 
    /// Since: Redis 3.0.0
    /// Group: Cluster
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @fast
    /// * @connection
    fn asking<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("ASKING");
            rv.query_async(self).await
        })
    }

    /// CLUSTER
    /// 
    /// A container for cluster commands
    /// 
    /// Since: Redis 3.0.0
    /// Group: Cluster
    /// Complexity: Depends on subcommand.
    /// ACL Categories:
    /// * @slow
    fn cluster<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLUSTER");
            rv.query_async(self).await
        })
    }

    /// CLUSTER ADDSLOTS
    /// 
    /// Assign new hash slots to receiving node
    /// 
    /// Since: Redis 3.0.0
    /// Group: Cluster
    /// Complexity: O(N) where N is the total number of hash slot arguments
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * NoAsyncLoading: This command is denied during asynchronous loading (that is when a replica uses disk-less SWAPDB SYNC, and allows access to the old dataset).
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn cluster_addslots<'a>(slot: &'a [i64]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLUSTER ADDSLOTS");
            rv.arg(slot);
            rv.query_async(self).await
        })
    }

    /// CLUSTER ADDSLOTSRANGE
    /// 
    /// Assign new hash slots to receiving node
    /// 
    /// Since: Redis 7.0.0
    /// Group: Cluster
    /// Complexity: O(N) where N is the total number of the slots between the start slot and end slot arguments.
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * NoAsyncLoading: This command is denied during asynchronous loading (that is when a replica uses disk-less SWAPDB SYNC, and allows access to the old dataset).
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn cluster_addslotsrange<'a, T0: ToRedisArgs + Send + Sync + 'a>(start_slot_end_slot: &'a [T0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLUSTER ADDSLOTSRANGE");
            rv.arg(start_slot_end_slot);
            rv.query_async(self).await
        })
    }

    /// CLUSTER BUMPEPOCH
    /// 
    /// Advance the cluster config epoch
    /// 
    /// Since: Redis 3.0.0
    /// Group: Cluster
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * NoAsyncLoading: This command is denied during asynchronous loading (that is when a replica uses disk-less SWAPDB SYNC, and allows access to the old dataset).
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn cluster_bumpepoch<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLUSTER BUMPEPOCH");
            rv.query_async(self).await
        })
    }

    /// CLUSTER COUNT-FAILURE-REPORTS
    /// 
    /// Return the number of failure reports active for a given node
    /// 
    /// Since: Redis 3.0.0
    /// Group: Cluster
    /// Complexity: O(N) where N is the number of failure reports
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn cluster_count_failure_reports<'a, T0: ToRedisArgs + Send + Sync + 'a>(node_id: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLUSTER COUNT-FAILURE-REPORTS");
            rv.arg(node_id);
            rv.query_async(self).await
        })
    }

    /// CLUSTER COUNTKEYSINSLOT
    /// 
    /// Return the number of local keys in the specified hash slot
    /// 
    /// Since: Redis 3.0.0
    /// Group: Cluster
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    fn cluster_countkeysinslot<'a>(slot: i64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLUSTER COUNTKEYSINSLOT");
            rv.arg(slot);
            rv.query_async(self).await
        })
    }

    /// CLUSTER DELSLOTS
    /// 
    /// Set hash slots as unbound in receiving node
    /// 
    /// Since: Redis 3.0.0
    /// Group: Cluster
    /// Complexity: O(N) where N is the total number of hash slot arguments
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * NoAsyncLoading: This command is denied during asynchronous loading (that is when a replica uses disk-less SWAPDB SYNC, and allows access to the old dataset).
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn cluster_delslots<'a>(slot: &'a [i64]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLUSTER DELSLOTS");
            rv.arg(slot);
            rv.query_async(self).await
        })
    }

    /// CLUSTER DELSLOTSRANGE
    /// 
    /// Set hash slots as unbound in receiving node
    /// 
    /// Since: Redis 7.0.0
    /// Group: Cluster
    /// Complexity: O(N) where N is the total number of the slots between the start slot and end slot arguments.
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * NoAsyncLoading: This command is denied during asynchronous loading (that is when a replica uses disk-less SWAPDB SYNC, and allows access to the old dataset).
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn cluster_delslotsrange<'a, T0: ToRedisArgs + Send + Sync + 'a>(start_slot_end_slot: &'a [T0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLUSTER DELSLOTSRANGE");
            rv.arg(start_slot_end_slot);
            rv.query_async(self).await
        })
    }

    /// CLUSTER FAILOVER
    /// 
    /// Forces a replica to perform a manual failover of its master.
    /// 
    /// Since: Redis 3.0.0
    /// Group: Cluster
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * NoAsyncLoading: This command is denied during asynchronous loading (that is when a replica uses disk-less SWAPDB SYNC, and allows access to the old dataset).
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn cluster_failover<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLUSTER FAILOVER");
            rv.query_async(self).await
        })
    }

    /// CLUSTER FLUSHSLOTS
    /// 
    /// Delete a node's own slots information
    /// 
    /// Since: Redis 3.0.0
    /// Group: Cluster
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * NoAsyncLoading: This command is denied during asynchronous loading (that is when a replica uses disk-less SWAPDB SYNC, and allows access to the old dataset).
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn cluster_flushslots<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLUSTER FLUSHSLOTS");
            rv.query_async(self).await
        })
    }

    /// CLUSTER FORGET
    /// 
    /// Remove a node from the nodes table
    /// 
    /// Since: Redis 3.0.0
    /// Group: Cluster
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * NoAsyncLoading: This command is denied during asynchronous loading (that is when a replica uses disk-less SWAPDB SYNC, and allows access to the old dataset).
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn cluster_forget<'a, T0: ToRedisArgs + Send + Sync + 'a>(node_id: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLUSTER FORGET");
            rv.arg(node_id);
            rv.query_async(self).await
        })
    }

    /// CLUSTER GETKEYSINSLOT
    /// 
    /// Return local key names in the specified hash slot
    /// 
    /// Since: Redis 3.0.0
    /// Group: Cluster
    /// Complexity: O(log(N)) where N is the number of requested keys
    /// CommandFlags:
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    fn cluster_getkeysinslot<'a>(slot: i64, count: i64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLUSTER GETKEYSINSLOT");
            rv.arg(slot);
            rv.arg(count);
            rv.query_async(self).await
        })
    }

    /// CLUSTER HELP
    /// 
    /// Show helpful text about the different subcommands
    /// 
    /// Since: Redis 5.0.0
    /// Group: Cluster
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    fn cluster_help<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLUSTER HELP");
            rv.query_async(self).await
        })
    }

    /// CLUSTER INFO
    /// 
    /// Provides info about Redis Cluster node state
    /// 
    /// Since: Redis 3.0.0
    /// Group: Cluster
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    fn cluster_info<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLUSTER INFO");
            rv.query_async(self).await
        })
    }

    /// CLUSTER KEYSLOT
    /// 
    /// Returns the hash slot of the specified key
    /// 
    /// Since: Redis 3.0.0
    /// Group: Cluster
    /// Complexity: O(N) where N is the number of bytes in the key
    /// CommandFlags:
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    fn cluster_keyslot<'a, T0: ToRedisArgs + Send + Sync + 'a>(key: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLUSTER KEYSLOT");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// CLUSTER LINKS
    /// 
    /// Returns a list of all TCP links to and from peer nodes in cluster
    /// 
    /// Since: Redis 7.0.0
    /// Group: Cluster
    /// Complexity: O(N) where N is the total number of Cluster nodes
    /// CommandFlags:
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    fn cluster_links<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLUSTER LINKS");
            rv.query_async(self).await
        })
    }

    /// CLUSTER MEET
    /// 
    /// Force a node cluster to handshake with another node
    /// 
    /// Since: Redis 3.0.0
    /// Group: Cluster
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * NoAsyncLoading: This command is denied during asynchronous loading (that is when a replica uses disk-less SWAPDB SYNC, and allows access to the old dataset).
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn cluster_meet<'a, T0: ToRedisArgs + Send + Sync + 'a>(ip: T0, port: i64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLUSTER MEET");
            rv.arg(ip);
            rv.arg(port);
            rv.query_async(self).await
        })
    }

    /// CLUSTER MYID
    /// 
    /// Return the node id
    /// 
    /// Since: Redis 3.0.0
    /// Group: Cluster
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    fn cluster_myid<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLUSTER MYID");
            rv.query_async(self).await
        })
    }

    /// CLUSTER NODES
    /// 
    /// Get Cluster config for the node
    /// 
    /// Since: Redis 3.0.0
    /// Group: Cluster
    /// Complexity: O(N) where N is the total number of Cluster nodes
    /// CommandFlags:
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    fn cluster_nodes<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLUSTER NODES");
            rv.query_async(self).await
        })
    }

    /// CLUSTER REPLICAS
    /// 
    /// List replica nodes of the specified master node
    /// 
    /// Since: Redis 5.0.0
    /// Group: Cluster
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn cluster_replicas<'a, T0: ToRedisArgs + Send + Sync + 'a>(node_id: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLUSTER REPLICAS");
            rv.arg(node_id);
            rv.query_async(self).await
        })
    }

    /// CLUSTER REPLICATE
    /// 
    /// Reconfigure a node as a replica of the specified master node
    /// 
    /// Since: Redis 3.0.0
    /// Group: Cluster
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * NoAsyncLoading: This command is denied during asynchronous loading (that is when a replica uses disk-less SWAPDB SYNC, and allows access to the old dataset).
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn cluster_replicate<'a, T0: ToRedisArgs + Send + Sync + 'a>(node_id: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLUSTER REPLICATE");
            rv.arg(node_id);
            rv.query_async(self).await
        })
    }

    /// CLUSTER RESET
    /// 
    /// Reset a Redis Cluster node
    /// 
    /// Since: Redis 3.0.0
    /// Group: Cluster
    /// Complexity: O(N) where N is the number of known nodes. The command may execute a FLUSHALL as a side effect.
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Noscript: This command can't be called from scripts or functions.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn cluster_reset<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLUSTER RESET");
            rv.query_async(self).await
        })
    }

    /// CLUSTER SAVECONFIG
    /// 
    /// Forces the node to save cluster state on disk
    /// 
    /// Since: Redis 3.0.0
    /// Group: Cluster
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * NoAsyncLoading: This command is denied during asynchronous loading (that is when a replica uses disk-less SWAPDB SYNC, and allows access to the old dataset).
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn cluster_saveconfig<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLUSTER SAVECONFIG");
            rv.query_async(self).await
        })
    }

    /// CLUSTER SET-CONFIG-EPOCH
    /// 
    /// Set the configuration epoch in a new node
    /// 
    /// Since: Redis 3.0.0
    /// Group: Cluster
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * NoAsyncLoading: This command is denied during asynchronous loading (that is when a replica uses disk-less SWAPDB SYNC, and allows access to the old dataset).
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn cluster_set_config_epoch<'a>(config_epoch: i64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLUSTER SET-CONFIG-EPOCH");
            rv.arg(config_epoch);
            rv.query_async(self).await
        })
    }

    /// CLUSTER SETSLOT
    /// 
    /// Bind a hash slot to a specific node
    /// 
    /// Since: Redis 3.0.0
    /// Group: Cluster
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * NoAsyncLoading: This command is denied during asynchronous loading (that is when a replica uses disk-less SWAPDB SYNC, and allows access to the old dataset).
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    fn cluster_setslot<'a>(slot: i64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLUSTER SETSLOT");
            rv.arg(slot);
            rv.query_async(self).await
        })
    }

    /// CLUSTER SHARDS
    /// 
    /// Get array of cluster slots to node mappings
    /// 
    /// Since: Redis 7.0.0
    /// Group: Cluster
    /// Complexity: O(N) where N is the total number of cluster nodes
    /// CommandFlags:
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    fn cluster_shards<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLUSTER SHARDS");
            rv.query_async(self).await
        })
    }

    /// CLUSTER SLAVES
    /// 
    /// List replica nodes of the specified master node
    /// 
    /// Since: Redis 3.0.0
    /// Group: Cluster
    /// Replaced By: `CLUSTER REPLICAS`
    /// Complexity: O(1)
    /// Replaced By: `CLUSTER REPLICAS`
    /// CommandFlags:
    /// * Admin: This command is an administrative command.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @admin
    /// * @slow
    /// * @dangerous
    #[deprecated]
    fn cluster_slaves<'a, T0: ToRedisArgs + Send + Sync + 'a>(node_id: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLUSTER SLAVES");
            rv.arg(node_id);
            rv.query_async(self).await
        })
    }

    /// CLUSTER SLOTS
    /// 
    /// Get array of Cluster slot to node mappings
    /// 
    /// Since: Redis 3.0.0
    /// Group: Cluster
    /// Replaced By: `CLUSTER SHARDS`
    /// Complexity: O(N) where N is the total number of Cluster nodes
    /// Replaced By: `CLUSTER SHARDS`
    /// CommandFlags:
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @slow
    #[deprecated]
    fn cluster_slots<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("CLUSTER SLOTS");
            rv.query_async(self).await
        })
    }

    /// READONLY
    /// 
    /// Enables read queries for a connection to a cluster replica node
    /// 
    /// Since: Redis 3.0.0
    /// Group: Cluster
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @fast
    /// * @connection
    fn readonly<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("READONLY");
            rv.query_async(self).await
        })
    }

    /// READWRITE
    /// 
    /// Disables read queries for a connection to a cluster replica node
    /// 
    /// Since: Redis 3.0.0
    /// Group: Cluster
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @fast
    /// * @connection
    fn readwrite<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("READWRITE");
            rv.query_async(self).await
        })
    }

    /// GEOADD
    /// 
    /// Add one or more geospatial items in the geospatial index represented using a sorted set
    /// 
    /// Since: Redis 3.2.0
    /// Group: Geo
    /// Complexity: O(log(N)) for each item added, where N is the number of elements in the sorted set.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// ACL Categories:
    /// * @write
    /// * @geo
    /// * @slow
    #[cfg(feature = "geospatial")]
    #[cfg_attr(docsrs, doc(cfg(feature = "geospatial")))]
    fn geoadd<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, longitude_latitude_member: &'a [T0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("GEOADD");
            rv.arg(key);
            rv.arg(longitude_latitude_member);
            rv.query_async(self).await
        })
    }

    /// GEODIST
    /// 
    /// Returns the distance between two members of a geospatial index
    /// 
    /// Since: Redis 3.2.0
    /// Group: Geo
    /// Complexity: O(log(N))
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @geo
    /// * @slow
    #[cfg(feature = "geospatial")]
    #[cfg_attr(docsrs, doc(cfg(feature = "geospatial")))]
    fn geodist<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(key: K0, member1: T0, member2: T1) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("GEODIST");
            rv.arg(key);
            rv.arg(member1);
            rv.arg(member2);
            rv.query_async(self).await
        })
    }

    /// GEOHASH
    /// 
    /// Returns members of a geospatial index as standard geohash strings
    /// 
    /// Since: Redis 3.2.0
    /// Group: Geo
    /// Complexity: O(log(N)) for each member requested, where N is the number of elements in the sorted set.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @geo
    /// * @slow
    #[cfg(feature = "geospatial")]
    #[cfg_attr(docsrs, doc(cfg(feature = "geospatial")))]
    fn geohash<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, member: &'a [T0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("GEOHASH");
            rv.arg(key);
            rv.arg(member);
            rv.query_async(self).await
        })
    }

    /// GEOPOS
    /// 
    /// Returns longitude and latitude of members of a geospatial index
    /// 
    /// Since: Redis 3.2.0
    /// Group: Geo
    /// Complexity: O(N) where N is the number of members requested.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @geo
    /// * @slow
    #[cfg(feature = "geospatial")]
    #[cfg_attr(docsrs, doc(cfg(feature = "geospatial")))]
    fn geopos<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, member: &'a [T0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("GEOPOS");
            rv.arg(key);
            rv.arg(member);
            rv.query_async(self).await
        })
    }

    /// GEORADIUS
    /// 
    /// Query a sorted set representing a geospatial index to fetch members matching a given maximum distance from a point
    /// 
    /// Since: Redis 3.2.0
    /// Group: Geo
    /// Replaced By: `GEOSEARCH` and `GEOSEARCHSTORE` with the `BYRADIUS` argument
    /// Complexity: O(N+log(M)) where N is the number of elements inside the bounding box of the circular area delimited by center and radius and M is the number of items inside the index.
    /// Replaced By: `GEOSEARCH` and `GEOSEARCHSTORE` with the `BYRADIUS` argument
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Movablekeys: This first key, last key, and step values don't determine all key positions. Clients need to use COMMAND GETKEYS or key specifications in this case. See below for more details.
    /// ACL Categories:
    /// * @write
    /// * @geo
    /// * @slow
    #[cfg(feature = "geospatial")]
    #[cfg_attr(docsrs, doc(cfg(feature = "geospatial")))]
    #[deprecated]
    fn georadius<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, longitude: f64, latitude: f64, radius: f64, count: Option<T0>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("GEORADIUS");
            rv.arg(key);
            rv.arg(longitude);
            rv.arg(latitude);
            rv.arg(radius);
            rv.arg(count);
            rv.query_async(self).await
        })
    }

    /// GEORADIUSBYMEMBER
    /// 
    /// Query a sorted set representing a geospatial index to fetch members matching a given maximum distance from a member
    /// 
    /// Since: Redis 3.2.0
    /// Group: Geo
    /// Replaced By: `GEOSEARCH` and `GEOSEARCHSTORE` with the `BYRADIUS` and `FROMMEMBER` arguments
    /// Complexity: O(N+log(M)) where N is the number of elements inside the bounding box of the circular area delimited by center and radius and M is the number of items inside the index.
    /// Replaced By: `GEOSEARCH` and `GEOSEARCHSTORE` with the `BYRADIUS` and `FROMMEMBER` arguments
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Movablekeys: This first key, last key, and step values don't determine all key positions. Clients need to use COMMAND GETKEYS or key specifications in this case. See below for more details.
    /// ACL Categories:
    /// * @write
    /// * @geo
    /// * @slow
    #[cfg(feature = "geospatial")]
    #[cfg_attr(docsrs, doc(cfg(feature = "geospatial")))]
    #[deprecated]
    fn georadiusbymember<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(key: K0, member: T0, radius: f64, count: Option<T1>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("GEORADIUSBYMEMBER");
            rv.arg(key);
            rv.arg(member);
            rv.arg(radius);
            rv.arg(count);
            rv.query_async(self).await
        })
    }

    /// GEORADIUSBYMEMBER_RO
    /// 
    /// A read-only variant for GEORADIUSBYMEMBER
    /// 
    /// Since: Redis 3.2.10
    /// Group: Geo
    /// Replaced By: `GEOSEARCH` with the `BYRADIUS` and `FROMMEMBER` arguments
    /// Complexity: O(N+log(M)) where N is the number of elements inside the bounding box of the circular area delimited by center and radius and M is the number of items inside the index.
    /// Replaced By: `GEOSEARCH` with the `BYRADIUS` and `FROMMEMBER` arguments
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @geo
    /// * @slow
    #[cfg(feature = "geospatial")]
    #[cfg_attr(docsrs, doc(cfg(feature = "geospatial")))]
    #[deprecated]
    fn georadiusbymember_ro<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(key: K0, member: T0, radius: f64, count: Option<T1>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("GEORADIUSBYMEMBER_RO");
            rv.arg(key);
            rv.arg(member);
            rv.arg(radius);
            rv.arg(count);
            rv.query_async(self).await
        })
    }

    /// GEORADIUS_RO
    /// 
    /// A read-only variant for GEORADIUS
    /// 
    /// Since: Redis 3.2.10
    /// Group: Geo
    /// Replaced By: `GEOSEARCH` with the `BYRADIUS` argument
    /// Complexity: O(N+log(M)) where N is the number of elements inside the bounding box of the circular area delimited by center and radius and M is the number of items inside the index.
    /// Replaced By: `GEOSEARCH` with the `BYRADIUS` argument
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @geo
    /// * @slow
    #[cfg(feature = "geospatial")]
    #[cfg_attr(docsrs, doc(cfg(feature = "geospatial")))]
    #[deprecated]
    fn georadius_ro<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, longitude: f64, latitude: f64, radius: f64, count: Option<T0>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("GEORADIUS_RO");
            rv.arg(key);
            rv.arg(longitude);
            rv.arg(latitude);
            rv.arg(radius);
            rv.arg(count);
            rv.query_async(self).await
        })
    }

    /// GEOSEARCH
    /// 
    /// Query a sorted set representing a geospatial index to fetch members inside an area of a box or a circle.
    /// 
    /// Since: Redis 6.2.0
    /// Group: Geo
    /// Complexity: O(N+log(M)) where N is the number of elements in the grid-aligned bounding box area around the shape provided as the filter and M is the number of items inside the shape
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @geo
    /// * @slow
    #[cfg(feature = "geospatial")]
    #[cfg_attr(docsrs, doc(cfg(feature = "geospatial")))]
    fn geosearch<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, count: Option<T0>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("GEOSEARCH");
            rv.arg(key);
            rv.arg(count);
            rv.query_async(self).await
        })
    }

    /// GEOSEARCHSTORE
    /// 
    /// Query a sorted set representing a geospatial index to fetch members inside an area of a box or a circle, and store the result in another key.
    /// 
    /// Since: Redis 6.2.0
    /// Group: Geo
    /// Complexity: O(N+log(M)) where N is the number of elements in the grid-aligned bounding box area around the shape provided as the filter and M is the number of items inside the shape
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// ACL Categories:
    /// * @write
    /// * @geo
    /// * @slow
    #[cfg(feature = "geospatial")]
    #[cfg_attr(docsrs, doc(cfg(feature = "geospatial")))]
    fn geosearchstore<'a, K0: ToRedisArgs + Send + Sync + 'a, K1: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(destination: K0, source: K1, count: Option<T0>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("GEOSEARCHSTORE");
            rv.arg(destination);
            rv.arg(source);
            rv.arg(count);
            rv.query_async(self).await
        })
    }

    /// XACK
    /// 
    /// Marks a pending message as correctly processed, effectively removing it from the pending entries list of the consumer group. Return value of the command is the number of messages successfully acknowledged, that is, the IDs we were actually able to resolve in the PEL.
    /// 
    /// Since: Redis 5.0.0
    /// Group: Stream
    /// Complexity: O(1) for each message ID processed.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @stream
    /// * @fast
    #[cfg(feature = "streams")]
    #[cfg_attr(docsrs, doc(cfg(feature = "streams")))]
    fn xack<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(key: K0, group: T0, id: &'a [T1]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("XACK");
            rv.arg(key);
            rv.arg(group);
            rv.arg(id);
            rv.query_async(self).await
        })
    }

    /// XADD
    /// 
    /// Appends a new entry to a stream
    /// 
    /// Since: Redis 5.0.0
    /// Group: Stream
    /// Complexity: O(1) when adding a new entry, O(N) when trimming where N being the number of entries evicted.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @stream
    /// * @fast
    #[cfg(feature = "streams")]
    #[cfg_attr(docsrs, doc(cfg(feature = "streams")))]
    fn xadd<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(key: K0, trim: Option<T0>, field_value: &'a [T1]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("XADD");
            rv.arg(key);
            rv.arg(trim);
            rv.arg(field_value);
            rv.query_async(self).await
        })
    }

    /// XAUTOCLAIM
    /// 
    /// Changes (or acquires) ownership of messages in a consumer group, as if the messages were delivered to the specified consumer.
    /// 
    /// Since: Redis 6.2.0
    /// Group: Stream
    /// Complexity: O(1) if COUNT is small.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @stream
    /// * @fast
    #[cfg(feature = "streams")]
    #[cfg_attr(docsrs, doc(cfg(feature = "streams")))]
    fn xautoclaim<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a, T2: ToRedisArgs + Send + Sync + 'a, T3: ToRedisArgs + Send + Sync + 'a>(key: K0, group: T0, consumer: T1, min_idle_time: T2, start: T3) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("XAUTOCLAIM");
            rv.arg(key);
            rv.arg(group);
            rv.arg(consumer);
            rv.arg(min_idle_time);
            rv.arg(start);
            rv.query_async(self).await
        })
    }

    /// XCLAIM
    /// 
    /// Changes (or acquires) ownership of a message in a consumer group, as if the message was delivered to the specified consumer.
    /// 
    /// Since: Redis 5.0.0
    /// Group: Stream
    /// Complexity: O(log N) with N being the number of messages in the PEL of the consumer group.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @stream
    /// * @fast
    #[cfg(feature = "streams")]
    #[cfg_attr(docsrs, doc(cfg(feature = "streams")))]
    fn xclaim<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a, T2: ToRedisArgs + Send + Sync + 'a, T3: ToRedisArgs + Send + Sync + 'a>(key: K0, group: T0, consumer: T1, min_idle_time: T2, id: &'a [T3]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("XCLAIM");
            rv.arg(key);
            rv.arg(group);
            rv.arg(consumer);
            rv.arg(min_idle_time);
            rv.arg(id);
            rv.query_async(self).await
        })
    }

    /// XDEL
    /// 
    /// Removes the specified entries from the stream. Returns the number of items actually deleted, that may be different from the number of IDs passed in case certain IDs do not exist.
    /// 
    /// Since: Redis 5.0.0
    /// Group: Stream
    /// Complexity: O(1) for each single item to delete in the stream, regardless of the stream size.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @stream
    /// * @fast
    #[cfg(feature = "streams")]
    #[cfg_attr(docsrs, doc(cfg(feature = "streams")))]
    fn xdel<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, id: &'a [T0]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("XDEL");
            rv.arg(key);
            rv.arg(id);
            rv.query_async(self).await
        })
    }

    /// XGROUP
    /// 
    /// A container for consumer groups commands
    /// 
    /// Since: Redis 5.0.0
    /// Group: Stream
    /// Complexity: Depends on subcommand.
    /// ACL Categories:
    /// * @slow
    #[cfg(feature = "streams")]
    #[cfg_attr(docsrs, doc(cfg(feature = "streams")))]
    fn xgroup<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("XGROUP");
            rv.query_async(self).await
        })
    }

    /// XGROUP CREATE
    /// 
    /// Create a consumer group.
    /// 
    /// Since: Redis 5.0.0
    /// Group: Stream
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// ACL Categories:
    /// * @write
    /// * @stream
    /// * @slow
    #[cfg(feature = "streams")]
    #[cfg_attr(docsrs, doc(cfg(feature = "streams")))]
    fn xgroup_create<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, groupname: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("XGROUP CREATE");
            rv.arg(key);
            rv.arg(groupname);
            rv.query_async(self).await
        })
    }

    /// XGROUP CREATECONSUMER
    /// 
    /// Create a consumer in a consumer group.
    /// 
    /// Since: Redis 6.2.0
    /// Group: Stream
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// ACL Categories:
    /// * @write
    /// * @stream
    /// * @slow
    #[cfg(feature = "streams")]
    #[cfg_attr(docsrs, doc(cfg(feature = "streams")))]
    fn xgroup_createconsumer<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(key: K0, groupname: T0, consumername: T1) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("XGROUP CREATECONSUMER");
            rv.arg(key);
            rv.arg(groupname);
            rv.arg(consumername);
            rv.query_async(self).await
        })
    }

    /// XGROUP DELCONSUMER
    /// 
    /// Delete a consumer from a consumer group.
    /// 
    /// Since: Redis 5.0.0
    /// Group: Stream
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// ACL Categories:
    /// * @write
    /// * @stream
    /// * @slow
    #[cfg(feature = "streams")]
    #[cfg_attr(docsrs, doc(cfg(feature = "streams")))]
    fn xgroup_delconsumer<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(key: K0, groupname: T0, consumername: T1) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("XGROUP DELCONSUMER");
            rv.arg(key);
            rv.arg(groupname);
            rv.arg(consumername);
            rv.query_async(self).await
        })
    }

    /// XGROUP DESTROY
    /// 
    /// Destroy a consumer group.
    /// 
    /// Since: Redis 5.0.0
    /// Group: Stream
    /// Complexity: O(N) where N is the number of entries in the group's pending entries list (PEL).
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// ACL Categories:
    /// * @write
    /// * @stream
    /// * @slow
    #[cfg(feature = "streams")]
    #[cfg_attr(docsrs, doc(cfg(feature = "streams")))]
    fn xgroup_destroy<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, groupname: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("XGROUP DESTROY");
            rv.arg(key);
            rv.arg(groupname);
            rv.query_async(self).await
        })
    }

    /// XGROUP HELP
    /// 
    /// Show helpful text about the different subcommands
    /// 
    /// Since: Redis 5.0.0
    /// Group: Stream
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @stream
    /// * @slow
    #[cfg(feature = "streams")]
    #[cfg_attr(docsrs, doc(cfg(feature = "streams")))]
    fn xgroup_help<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("XGROUP HELP");
            rv.query_async(self).await
        })
    }

    /// XGROUP SETID
    /// 
    /// Set a consumer group to an arbitrary last delivered ID value.
    /// 
    /// Since: Redis 5.0.0
    /// Group: Stream
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// ACL Categories:
    /// * @write
    /// * @stream
    /// * @slow
    #[cfg(feature = "streams")]
    #[cfg_attr(docsrs, doc(cfg(feature = "streams")))]
    fn xgroup_setid<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, groupname: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("XGROUP SETID");
            rv.arg(key);
            rv.arg(groupname);
            rv.query_async(self).await
        })
    }

    /// XINFO
    /// 
    /// A container for stream introspection commands
    /// 
    /// Since: Redis 5.0.0
    /// Group: Stream
    /// Complexity: Depends on subcommand.
    /// ACL Categories:
    /// * @slow
    #[cfg(feature = "streams")]
    #[cfg_attr(docsrs, doc(cfg(feature = "streams")))]
    fn xinfo<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("XINFO");
            rv.query_async(self).await
        })
    }

    /// XINFO CONSUMERS
    /// 
    /// List the consumers in a consumer group
    /// 
    /// Since: Redis 5.0.0
    /// Group: Stream
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @stream
    /// * @slow
    #[cfg(feature = "streams")]
    #[cfg_attr(docsrs, doc(cfg(feature = "streams")))]
    fn xinfo_consumers<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, groupname: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("XINFO CONSUMERS");
            rv.arg(key);
            rv.arg(groupname);
            rv.query_async(self).await
        })
    }

    /// XINFO GROUPS
    /// 
    /// List the consumer groups of a stream
    /// 
    /// Since: Redis 5.0.0
    /// Group: Stream
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @stream
    /// * @slow
    #[cfg(feature = "streams")]
    #[cfg_attr(docsrs, doc(cfg(feature = "streams")))]
    fn xinfo_groups<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("XINFO GROUPS");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// XINFO HELP
    /// 
    /// Show helpful text about the different subcommands
    /// 
    /// Since: Redis 5.0.0
    /// Group: Stream
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Loading: This command is allowed while the database is loading.
    /// * Stale: This command is allowed while a replica has stale data.
    /// ACL Categories:
    /// * @stream
    /// * @slow
    #[cfg(feature = "streams")]
    #[cfg_attr(docsrs, doc(cfg(feature = "streams")))]
    fn xinfo_help<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("XINFO HELP");
            rv.query_async(self).await
        })
    }

    /// XINFO STREAM
    /// 
    /// Get information about a stream
    /// 
    /// Since: Redis 5.0.0
    /// Group: Stream
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @stream
    /// * @slow
    #[cfg(feature = "streams")]
    #[cfg_attr(docsrs, doc(cfg(feature = "streams")))]
    fn xinfo_stream<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("XINFO STREAM");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// XLEN
    /// 
    /// Return the number of entries in a stream
    /// 
    /// Since: Redis 5.0.0
    /// Group: Stream
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @read
    /// * @stream
    /// * @fast
    #[cfg(feature = "streams")]
    #[cfg_attr(docsrs, doc(cfg(feature = "streams")))]
    fn xlen<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("XLEN");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// XPENDING
    /// 
    /// Return information and entries from a stream consumer group pending entries list, that are messages fetched but never acknowledged.
    /// 
    /// Since: Redis 5.0.0
    /// Group: Stream
    /// Complexity: O(N) with N being the number of elements returned, so asking for a small fixed number of entries per call is O(1). O(M), where M is the total number of entries scanned when used with the IDLE filter. When the command returns just the summary and the list of consumers is small, it runs in O(1) time; otherwise, an additional O(N) time for iterating every consumer.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @stream
    /// * @slow
    #[cfg(feature = "streams")]
    #[cfg_attr(docsrs, doc(cfg(feature = "streams")))]
    fn xpending<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(key: K0, group: T0, filters: Option<T1>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("XPENDING");
            rv.arg(key);
            rv.arg(group);
            rv.arg(filters);
            rv.query_async(self).await
        })
    }

    /// XRANGE
    /// 
    /// Return a range of elements in a stream, with IDs matching the specified IDs interval
    /// 
    /// Since: Redis 5.0.0
    /// Group: Stream
    /// Complexity: O(N) with N being the number of elements being returned. If N is constant (e.g. always asking for the first 10 elements with COUNT), you can consider it O(1).
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @stream
    /// * @slow
    #[cfg(feature = "streams")]
    #[cfg_attr(docsrs, doc(cfg(feature = "streams")))]
    fn xrange<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(key: K0, start: T0, end: T1) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("XRANGE");
            rv.arg(key);
            rv.arg(start);
            rv.arg(end);
            rv.query_async(self).await
        })
    }

    /// XREAD
    /// 
    /// Return never seen elements in multiple streams, with IDs greater than the ones reported by the caller for each stream. Can block.
    /// 
    /// Since: Redis 5.0.0
    /// Group: Stream
    /// Complexity: For each stream mentioned: O(N) with N being the number of elements being returned, it means that XREAD-ing with a fixed COUNT is O(1). Note that when the BLOCK option is used, XADD will pay O(M) time in order to serve the M clients blocked on the stream getting new data.
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Blocking: This command may block the requesting client.
    /// * Movablekeys: This first key, last key, and step values don't determine all key positions. Clients need to use COMMAND GETKEYS or key specifications in this case. See below for more details.
    /// ACL Categories:
    /// * @read
    /// * @stream
    /// * @slow
    /// * @blocking
    #[cfg(feature = "streams")]
    #[cfg_attr(docsrs, doc(cfg(feature = "streams")))]
    fn xread<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("XREAD");
            rv.query_async(self).await
        })
    }

    /// XREADGROUP
    /// 
    /// Return new entries from a stream using a consumer group, or access the history of the pending entries for a given consumer. Can block.
    /// 
    /// Since: Redis 5.0.0
    /// Group: Stream
    /// Complexity: For each stream mentioned: O(M) with M being the number of elements returned. If M is constant (e.g. always asking for the first 10 elements with COUNT), you can consider it O(1). On the other side when XREADGROUP blocks, XADD will pay the O(N) time in order to serve the N clients blocked on the stream getting new data.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Blocking: This command may block the requesting client.
    /// * Movablekeys: This first key, last key, and step values don't determine all key positions. Clients need to use COMMAND GETKEYS or key specifications in this case. See below for more details.
    /// ACL Categories:
    /// * @write
    /// * @stream
    /// * @slow
    /// * @blocking
    #[cfg(feature = "streams")]
    #[cfg_attr(docsrs, doc(cfg(feature = "streams")))]
    fn xreadgroup<'a>() -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("XREADGROUP");
            rv.query_async(self).await
        })
    }

    /// XREVRANGE
    /// 
    /// Return a range of elements in a stream, with IDs matching the specified IDs interval, in reverse order (from greater to smaller IDs) compared to XRANGE
    /// 
    /// Since: Redis 5.0.0
    /// Group: Stream
    /// Complexity: O(N) with N being the number of elements returned. If N is constant (e.g. always asking for the first 10 elements with COUNT), you can consider it O(1).
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @stream
    /// * @slow
    #[cfg(feature = "streams")]
    #[cfg_attr(docsrs, doc(cfg(feature = "streams")))]
    fn xrevrange<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a, T1: ToRedisArgs + Send + Sync + 'a>(key: K0, end: T0, start: T1) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("XREVRANGE");
            rv.arg(key);
            rv.arg(end);
            rv.arg(start);
            rv.query_async(self).await
        })
    }

    /// XSETID
    /// 
    /// An internal command for replicating stream values
    /// 
    /// Since: Redis 5.0.0
    /// Group: Stream
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @write
    /// * @stream
    /// * @fast
    #[cfg(feature = "streams")]
    #[cfg_attr(docsrs, doc(cfg(feature = "streams")))]
    fn xsetid<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, last_id: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("XSETID");
            rv.arg(key);
            rv.arg(last_id);
            rv.query_async(self).await
        })
    }

    /// XTRIM
    /// 
    /// Trims the stream to (approximately if '~' is passed) a certain size
    /// 
    /// Since: Redis 5.0.0
    /// Group: Stream
    /// Complexity: O(N), with N being the number of evicted entries. Constant times are very small however, since entries are organized in macro nodes containing multiple entries that can be released with a single deallocation.
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// ACL Categories:
    /// * @write
    /// * @stream
    /// * @slow
    #[cfg(feature = "streams")]
    #[cfg_attr(docsrs, doc(cfg(feature = "streams")))]
    fn xtrim<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, trim: T0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("XTRIM");
            rv.arg(key);
            rv.arg(trim);
            rv.query_async(self).await
        })
    }

    /// BITCOUNT
    /// 
    /// Count set bits in a string
    /// 
    /// Since: Redis 2.6.0
    /// Group: Bitmap
    /// Complexity: O(N)
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @bitmap
    /// * @slow
    fn bitcount<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, index: Option<T0>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("BITCOUNT");
            rv.arg(key);
            rv.arg(index);
            rv.query_async(self).await
        })
    }

    /// BITFIELD
    /// 
    /// Perform arbitrary bitfield integer operations on strings
    /// 
    /// Since: Redis 3.2.0
    /// Group: Bitmap
    /// Complexity: O(1) for each subcommand specified
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// * Movablekeys: This first key, last key, and step values don't determine all key positions. Clients need to use COMMAND GETKEYS or key specifications in this case. See below for more details.
    /// ACL Categories:
    /// * @write
    /// * @bitmap
    /// * @slow
    fn bitfield<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("BITFIELD");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// BITFIELD_RO
    /// 
    /// Perform arbitrary bitfield integer operations on strings. Read-only variant of BITFIELD
    /// 
    /// Since: Redis 6.2.0
    /// Group: Bitmap
    /// Complexity: O(1) for each subcommand specified
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @read
    /// * @bitmap
    /// * @fast
    fn bitfield_ro<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("BITFIELD_RO");
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// BITOP
    /// 
    /// Perform bitwise operations between strings
    /// 
    /// Since: Redis 2.6.0
    /// Group: Bitmap
    /// Complexity: O(N)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// ACL Categories:
    /// * @write
    /// * @bitmap
    /// * @slow
    fn bitop<'a, T0: ToRedisArgs + Send + Sync + 'a, K0: ToRedisArgs + Send + Sync + 'a, K1: ToRedisArgs + Send + Sync + 'a>(operation: T0, destkey: K0, key: &'a [K1]) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("BITOP");
            rv.arg(operation);
            rv.arg(destkey);
            rv.arg(key);
            rv.query_async(self).await
        })
    }

    /// BITPOS
    /// 
    /// Find first bit set or clear in a string
    /// 
    /// Since: Redis 2.8.7
    /// Group: Bitmap
    /// Complexity: O(N)
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// ACL Categories:
    /// * @read
    /// * @bitmap
    /// * @slow
    fn bitpos<'a, K0: ToRedisArgs + Send + Sync + 'a, T0: ToRedisArgs + Send + Sync + 'a>(key: K0, bit: i64, index: Option<T0>) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("BITPOS");
            rv.arg(key);
            rv.arg(bit);
            rv.arg(index);
            rv.query_async(self).await
        })
    }

    /// GETBIT
    /// 
    /// Returns the bit value at offset in the string value stored at key
    /// 
    /// Since: Redis 2.2.0
    /// Group: Bitmap
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Readonly: This command doesn't modify data.
    /// * Fast: This command operates in constant or log(N) time. This flag is used for monitoring latency with the LATENCY command.
    /// ACL Categories:
    /// * @read
    /// * @bitmap
    /// * @fast
    fn getbit<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0, offset: i64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("GETBIT");
            rv.arg(key);
            rv.arg(offset);
            rv.query_async(self).await
        })
    }

    /// SETBIT
    /// 
    /// Sets or clears the bit at offset in the string value stored at key
    /// 
    /// Since: Redis 2.2.0
    /// Group: Bitmap
    /// Complexity: O(1)
    /// CommandFlags:
    /// * Write: This command may modify data.
    /// * Denyoom: This command is rejected if the server's memory usage is too high (see the maxmemory configuration directive).
    /// ACL Categories:
    /// * @write
    /// * @bitmap
    /// * @slow
    fn setbit<'a, K0: ToRedisArgs + Send + Sync + 'a>(key: K0, offset: i64, value: i64) -> Self {
        Box::pin(async move {
            let mut rv = Cmd::new();
            rv.arg("SETBIT");
            rv.arg(key);
            rv.arg(offset);
            rv.arg(value);
            rv.query_async(self).await
        })
    }

}
