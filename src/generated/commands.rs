#![cfg_attr(rustfmt, rustfmt_skip)]
#[allow(deprecated)]
use crate::connection::ConnectionLike;
use crate::cmd::Cmd;
use crate::types::{FromRedisValue, RedisResult, ToRedisArgs};

/// Implements common redis commands for connection like objects.  This
/// allows you to send commands straight to a connection or client.  It
/// is also implemented for redis results of clients which makes for
/// very convenient access in some basic cases.
///
/// This allows you to use nicer syntax for some common operations.
/// For instance this code:
///
/// ```rust,no_run
/// # fn do_something() -> redis::RedisResult<()> {
/// let client = redis::Client::open("redis://127.0.0.1/")?;
/// let mut con = client.get_connection()?;
/// redis::cmd("SET").arg("my_key").arg(42).execute(&mut con);
/// assert_eq!(redis::cmd("GET").arg("my_key").query(&mut con), Ok(42));
/// # Ok(()) }
/// ```
///
/// Will become this:
///
/// ```rust,no_run
/// # fn do_something() -> redis::RedisResult<()> {
/// use redis::Commands;
/// let client = redis::Client::open("redis://127.0.0.1/")?;
/// let mut con = client.get_connection()?;
/// con.set("my_key", 42)?;
/// assert_eq!(con.get("my_key"), Ok(42));
/// # Ok(()) }
/// ```
pub trait Commands : ConnectionLike + Sized {
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
    fn copy<K0: ToRedisArgs, K1: ToRedisArgs, RV: FromRedisValue>(&mut self, source: K0, destination: K1) -> RedisResult<RV> {
        Cmd::copy(source, destination).query(self)
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
    fn del<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: &[K0]) -> RedisResult<RV> {
        Cmd::del(key).query(self)
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
    fn dump<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::dump(key).query(self)
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
    fn exists<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: &[K0]) -> RedisResult<RV> {
        Cmd::exists(key).query(self)
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
    fn expire<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, seconds: i64) -> RedisResult<RV> {
        Cmd::expire(key, seconds).query(self)
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
    fn expireat<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::expireat(key).query(self)
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
    fn expiretime<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::expiretime(key).query(self)
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
    fn keys<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, pattern: K0) -> RedisResult<RV> {
        Cmd::keys(pattern).query(self)
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
    fn migrate<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, host: T0, port: i64, destination_db: i64, timeout: i64) -> RedisResult<RV> {
        Cmd::migrate(host, port, destination_db, timeout).query(self)
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
    fn move_key<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, db: i64) -> RedisResult<RV> {
        Cmd::move_key(key, db).query(self)
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
    fn object_encoding<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::object_encoding(key).query(self)
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
    fn object_freq<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::object_freq(key).query(self)
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
    fn object_help<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::object_help().query(self)
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
    fn object_idletime<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::object_idletime(key).query(self)
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
    fn object_refcount<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::object_refcount(key).query(self)
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
    fn persist<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::persist(key).query(self)
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
    fn pexpire<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, milliseconds: i64) -> RedisResult<RV> {
        Cmd::pexpire(key, milliseconds).query(self)
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
    fn pexpireat<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::pexpireat(key).query(self)
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
    fn pexpiretime<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::pexpiretime(key).query(self)
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
    fn pttl<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::pttl(key).query(self)
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
    fn randomkey<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::randomkey().query(self)
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
    fn rename<K0: ToRedisArgs, K1: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, newkey: K1) -> RedisResult<RV> {
        Cmd::rename(key, newkey).query(self)
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
    fn renamenx<K0: ToRedisArgs, K1: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, newkey: K1) -> RedisResult<RV> {
        Cmd::renamenx(key, newkey).query(self)
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
    fn restore<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, ttl: i64, serialized_value: T0) -> RedisResult<RV> {
        Cmd::restore(key, ttl, serialized_value).query(self)
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
    fn sort<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::sort(key).query(self)
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
    fn sort_ro<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::sort_ro(key).query(self)
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
    fn touch<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: &[K0]) -> RedisResult<RV> {
        Cmd::touch(key).query(self)
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
    fn ttl<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::ttl(key).query(self)
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
    fn r#type<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::r#type(key).query(self)
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
    fn unlink<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: &[K0]) -> RedisResult<RV> {
        Cmd::unlink(key).query(self)
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
    fn wait<RV: FromRedisValue>(&mut self, numreplicas: i64, timeout: i64) -> RedisResult<RV> {
        Cmd::wait(numreplicas, timeout).query(self)
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
    fn append<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, value: T0) -> RedisResult<RV> {
        Cmd::append(key, value).query(self)
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
    fn decr<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::decr(key).query(self)
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
    fn decrby<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, decrement: i64) -> RedisResult<RV> {
        Cmd::decrby(key, decrement).query(self)
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
    fn get<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::get(key).query(self)
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
    fn getdel<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::getdel(key).query(self)
    }

    #[deprecated(since = "0.22.0", note = "With version 0.22.0 redis crate switched to a generated api. This is a deprecated old handwritten function that now aliases to the generated on and will be removed in a future update. ")]
    /// This is an alias for [`getdel`]
    fn get_del<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        self.getdel(key)
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
    fn getex<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::getex(key).query(self)
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
    fn getrange<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, start: i64, end: i64) -> RedisResult<RV> {
        Cmd::getrange(key, start, end).query(self)
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
    #[deprecated = "Deprecated in redis since redis version 6.2.0."]
    fn getset<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, value: T0) -> RedisResult<RV> {
        Cmd::getset(key, value).query(self)
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
    fn incr<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::incr(key).query(self)
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
    fn incrby<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, increment: i64) -> RedisResult<RV> {
        Cmd::incrby(key, increment).query(self)
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
    fn incrbyfloat<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, increment: f64) -> RedisResult<RV> {
        Cmd::incrbyfloat(key, increment).query(self)
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
    fn lcs<K0: ToRedisArgs, K1: ToRedisArgs, RV: FromRedisValue>(&mut self, key1: K0, key2: K1) -> RedisResult<RV> {
        Cmd::lcs(key1, key2).query(self)
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
    fn mget<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: &[K0]) -> RedisResult<RV> {
        Cmd::mget(key).query(self)
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
    fn mset<K0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, key_value: &[(K0, T1)]) -> RedisResult<RV> {
        Cmd::mset(key_value).query(self)
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
    fn msetnx<K0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, key_value: &[(K0, T1)]) -> RedisResult<RV> {
        Cmd::msetnx(key_value).query(self)
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
    fn psetex<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, milliseconds: i64, value: T0) -> RedisResult<RV> {
        Cmd::psetex(key, milliseconds, value).query(self)
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
    fn set<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, value: T0) -> RedisResult<RV> {
        Cmd::set(key, value).query(self)
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
    fn setex<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, seconds: i64, value: T0) -> RedisResult<RV> {
        Cmd::setex(key, seconds, value).query(self)
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
    fn setnx<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, value: T0) -> RedisResult<RV> {
        Cmd::setnx(key, value).query(self)
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
    fn setrange<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, offset: i64, value: T0) -> RedisResult<RV> {
        Cmd::setrange(key, offset, value).query(self)
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
    fn strlen<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::strlen(key).query(self)
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
    #[deprecated = "Deprecated in redis since redis version 2.0.0."]
    fn substr<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, start: i64, end: i64) -> RedisResult<RV> {
        Cmd::substr(key, start, end).query(self)
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
    fn blmove<K0: ToRedisArgs, K1: ToRedisArgs, RV: FromRedisValue>(&mut self, source: K0, destination: K1, timeout: f64) -> RedisResult<RV> {
        Cmd::blmove(source, destination, timeout).query(self)
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
    fn blmpop<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, timeout: f64, numkeys: i64, key: &[K0]) -> RedisResult<RV> {
        Cmd::blmpop(timeout, numkeys, key).query(self)
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
    fn blpop<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: &[K0], timeout: f64) -> RedisResult<RV> {
        Cmd::blpop(key, timeout).query(self)
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
    fn brpop<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: &[K0], timeout: f64) -> RedisResult<RV> {
        Cmd::brpop(key, timeout).query(self)
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
    #[deprecated = "Deprecated in redis since redis version 6.2.0."]
    fn brpoplpush<K0: ToRedisArgs, K1: ToRedisArgs, RV: FromRedisValue>(&mut self, source: K0, destination: K1, timeout: f64) -> RedisResult<RV> {
        Cmd::brpoplpush(source, destination, timeout).query(self)
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
    fn lindex<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, index: i64) -> RedisResult<RV> {
        Cmd::lindex(key, index).query(self)
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
    fn linsert<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, pivot: T0, element: T1) -> RedisResult<RV> {
        Cmd::linsert(key, pivot, element).query(self)
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
    fn llen<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::llen(key).query(self)
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
    fn lmove<K0: ToRedisArgs, K1: ToRedisArgs, RV: FromRedisValue>(&mut self, source: K0, destination: K1) -> RedisResult<RV> {
        Cmd::lmove(source, destination).query(self)
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
    fn lmpop<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, numkeys: i64, key: &[K0]) -> RedisResult<RV> {
        Cmd::lmpop(numkeys, key).query(self)
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
    fn lpop<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, count: Option<i64>) -> RedisResult<RV> {
        Cmd::lpop(key, count).query(self)
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
    fn lpos<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, element: T0) -> RedisResult<RV> {
        Cmd::lpos(key, element).query(self)
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
    fn lpush<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, element: &[T0]) -> RedisResult<RV> {
        Cmd::lpush(key, element).query(self)
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
    fn lpushx<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, element: &[T0]) -> RedisResult<RV> {
        Cmd::lpushx(key, element).query(self)
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
    fn lrange<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, start: i64, stop: i64) -> RedisResult<RV> {
        Cmd::lrange(key, start, stop).query(self)
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
    fn lrem<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, count: i64, element: T0) -> RedisResult<RV> {
        Cmd::lrem(key, count, element).query(self)
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
    fn lset<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, index: i64, element: T0) -> RedisResult<RV> {
        Cmd::lset(key, index, element).query(self)
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
    fn ltrim<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, start: i64, stop: i64) -> RedisResult<RV> {
        Cmd::ltrim(key, start, stop).query(self)
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
    fn rpop<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, count: Option<i64>) -> RedisResult<RV> {
        Cmd::rpop(key, count).query(self)
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
    #[deprecated = "Deprecated in redis since redis version 6.2.0."]
    fn rpoplpush<K0: ToRedisArgs, K1: ToRedisArgs, RV: FromRedisValue>(&mut self, source: K0, destination: K1) -> RedisResult<RV> {
        Cmd::rpoplpush(source, destination).query(self)
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
    fn rpush<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, element: &[T0]) -> RedisResult<RV> {
        Cmd::rpush(key, element).query(self)
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
    fn rpushx<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, element: &[T0]) -> RedisResult<RV> {
        Cmd::rpushx(key, element).query(self)
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
    fn sadd<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, member: &[T0]) -> RedisResult<RV> {
        Cmd::sadd(key, member).query(self)
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
    fn scard<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::scard(key).query(self)
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
    fn sdiff<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: &[K0]) -> RedisResult<RV> {
        Cmd::sdiff(key).query(self)
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
    fn sdiffstore<K0: ToRedisArgs, K1: ToRedisArgs, RV: FromRedisValue>(&mut self, destination: K0, key: &[K1]) -> RedisResult<RV> {
        Cmd::sdiffstore(destination, key).query(self)
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
    fn sinter<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: &[K0]) -> RedisResult<RV> {
        Cmd::sinter(key).query(self)
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
    fn sintercard<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, numkeys: i64, key: &[K0]) -> RedisResult<RV> {
        Cmd::sintercard(numkeys, key).query(self)
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
    fn sinterstore<K0: ToRedisArgs, K1: ToRedisArgs, RV: FromRedisValue>(&mut self, destination: K0, key: &[K1]) -> RedisResult<RV> {
        Cmd::sinterstore(destination, key).query(self)
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
    fn sismember<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, member: T0) -> RedisResult<RV> {
        Cmd::sismember(key, member).query(self)
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
    fn smembers<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::smembers(key).query(self)
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
    fn smismember<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, member: &[T0]) -> RedisResult<RV> {
        Cmd::smismember(key, member).query(self)
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
    fn smove<K0: ToRedisArgs, K1: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, source: K0, destination: K1, member: T0) -> RedisResult<RV> {
        Cmd::smove(source, destination, member).query(self)
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
    fn spop<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, count: Option<i64>) -> RedisResult<RV> {
        Cmd::spop(key, count).query(self)
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
    fn srandmember<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, count: Option<i64>) -> RedisResult<RV> {
        Cmd::srandmember(key, count).query(self)
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
    fn srem<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, member: &[T0]) -> RedisResult<RV> {
        Cmd::srem(key, member).query(self)
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
    fn sunion<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: &[K0]) -> RedisResult<RV> {
        Cmd::sunion(key).query(self)
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
    fn sunionstore<K0: ToRedisArgs, K1: ToRedisArgs, RV: FromRedisValue>(&mut self, destination: K0, key: &[K1]) -> RedisResult<RV> {
        Cmd::sunionstore(destination, key).query(self)
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
    fn bzmpop<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, timeout: f64, numkeys: i64, key: &[K0]) -> RedisResult<RV> {
        Cmd::bzmpop(timeout, numkeys, key).query(self)
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
    fn bzpopmax<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: &[K0], timeout: f64) -> RedisResult<RV> {
        Cmd::bzpopmax(key, timeout).query(self)
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
    fn bzpopmin<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: &[K0], timeout: f64) -> RedisResult<RV> {
        Cmd::bzpopmin(key, timeout).query(self)
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
    fn zadd<K0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, score_member: &[(f64, T1)]) -> RedisResult<RV> {
        Cmd::zadd(key, score_member).query(self)
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
    fn zcard<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::zcard(key).query(self)
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
    fn zcount<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, min: f64, max: f64) -> RedisResult<RV> {
        Cmd::zcount(key, min, max).query(self)
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
    fn zdiff<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, numkeys: i64, key: &[K0]) -> RedisResult<RV> {
        Cmd::zdiff(numkeys, key).query(self)
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
    fn zdiffstore<K0: ToRedisArgs, K1: ToRedisArgs, RV: FromRedisValue>(&mut self, destination: K0, numkeys: i64, key: &[K1]) -> RedisResult<RV> {
        Cmd::zdiffstore(destination, numkeys, key).query(self)
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
    fn zincrby<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, increment: i64, member: T0) -> RedisResult<RV> {
        Cmd::zincrby(key, increment, member).query(self)
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
    fn zinter<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, numkeys: i64, key: &[K0]) -> RedisResult<RV> {
        Cmd::zinter(numkeys, key).query(self)
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
    fn zintercard<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, numkeys: i64, key: &[K0]) -> RedisResult<RV> {
        Cmd::zintercard(numkeys, key).query(self)
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
    fn zinterstore<K0: ToRedisArgs, K1: ToRedisArgs, RV: FromRedisValue>(&mut self, destination: K0, numkeys: i64, key: &[K1]) -> RedisResult<RV> {
        Cmd::zinterstore(destination, numkeys, key).query(self)
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
    fn zlexcount<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, min: T0, max: T1) -> RedisResult<RV> {
        Cmd::zlexcount(key, min, max).query(self)
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
    fn zmpop<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, numkeys: i64, key: &[K0]) -> RedisResult<RV> {
        Cmd::zmpop(numkeys, key).query(self)
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
    fn zmscore<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, member: &[T0]) -> RedisResult<RV> {
        Cmd::zmscore(key, member).query(self)
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
    fn zpopmax<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, count: Option<i64>) -> RedisResult<RV> {
        Cmd::zpopmax(key, count).query(self)
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
    fn zpopmin<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, count: Option<i64>) -> RedisResult<RV> {
        Cmd::zpopmin(key, count).query(self)
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
    fn zrandmember<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, options: Option<T0>) -> RedisResult<RV> {
        Cmd::zrandmember(key, options).query(self)
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
    fn zrange<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, min: T0, max: T1) -> RedisResult<RV> {
        Cmd::zrange(key, min, max).query(self)
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
    #[deprecated = "Deprecated in redis since redis version 6.2.0."]
    fn zrangebylex<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, min: T0, max: T1) -> RedisResult<RV> {
        Cmd::zrangebylex(key, min, max).query(self)
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
    #[deprecated = "Deprecated in redis since redis version 6.2.0."]
    fn zrangebyscore<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, min: f64, max: f64) -> RedisResult<RV> {
        Cmd::zrangebyscore(key, min, max).query(self)
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
    fn zrangestore<K0: ToRedisArgs, K1: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, dst: K0, src: K1, min: T0, max: T1) -> RedisResult<RV> {
        Cmd::zrangestore(dst, src, min, max).query(self)
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
    fn zrank<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, member: T0) -> RedisResult<RV> {
        Cmd::zrank(key, member).query(self)
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
    fn zrem<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, member: &[T0]) -> RedisResult<RV> {
        Cmd::zrem(key, member).query(self)
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
    fn zremrangebylex<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, min: T0, max: T1) -> RedisResult<RV> {
        Cmd::zremrangebylex(key, min, max).query(self)
    }

    #[deprecated(since = "0.22.0", note = "With version 0.22.0 redis crate switched to a generated api. This is a deprecated old handwritten function that now aliases to the generated on and will be removed in a future update. ")]
    /// This is an alias for [`zremrangebylex`]
    fn zrembylex<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, min: T0, max: T1) -> RedisResult<RV> {
        self.zremrangebylex(key, min, max)
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
    fn zremrangebyrank<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, start: i64, stop: i64) -> RedisResult<RV> {
        Cmd::zremrangebyrank(key, start, stop).query(self)
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
    fn zremrangebyscore<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, min: f64, max: f64) -> RedisResult<RV> {
        Cmd::zremrangebyscore(key, min, max).query(self)
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
    #[deprecated = "Deprecated in redis since redis version 6.2.0."]
    fn zrevrange<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, start: i64, stop: i64) -> RedisResult<RV> {
        Cmd::zrevrange(key, start, stop).query(self)
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
    #[deprecated = "Deprecated in redis since redis version 6.2.0."]
    fn zrevrangebylex<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, max: T0, min: T1) -> RedisResult<RV> {
        Cmd::zrevrangebylex(key, max, min).query(self)
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
    #[deprecated = "Deprecated in redis since redis version 6.2.0."]
    fn zrevrangebyscore<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, max: f64, min: f64) -> RedisResult<RV> {
        Cmd::zrevrangebyscore(key, max, min).query(self)
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
    fn zrevrank<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, member: T0) -> RedisResult<RV> {
        Cmd::zrevrank(key, member).query(self)
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
    fn zscore<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, member: T0) -> RedisResult<RV> {
        Cmd::zscore(key, member).query(self)
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
    fn zunion<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, numkeys: i64, key: &[K0]) -> RedisResult<RV> {
        Cmd::zunion(numkeys, key).query(self)
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
    fn zunionstore<K0: ToRedisArgs, K1: ToRedisArgs, RV: FromRedisValue>(&mut self, destination: K0, numkeys: i64, key: &[K1]) -> RedisResult<RV> {
        Cmd::zunionstore(destination, numkeys, key).query(self)
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
    fn hdel<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, field: &[T0]) -> RedisResult<RV> {
        Cmd::hdel(key, field).query(self)
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
    fn hexists<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, field: T0) -> RedisResult<RV> {
        Cmd::hexists(key, field).query(self)
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
    fn hget<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, field: T0) -> RedisResult<RV> {
        Cmd::hget(key, field).query(self)
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
    fn hgetall<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::hgetall(key).query(self)
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
    fn hincrby<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, field: T0, increment: i64) -> RedisResult<RV> {
        Cmd::hincrby(key, field, increment).query(self)
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
    fn hincrbyfloat<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, field: T0, increment: f64) -> RedisResult<RV> {
        Cmd::hincrbyfloat(key, field, increment).query(self)
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
    fn hkeys<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::hkeys(key).query(self)
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
    fn hlen<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::hlen(key).query(self)
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
    fn hmget<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, field: &[T0]) -> RedisResult<RV> {
        Cmd::hmget(key, field).query(self)
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
    #[deprecated = "Deprecated in redis since redis version 4.0.0."]
    fn hmset<K0: ToRedisArgs, T1: ToRedisArgs, T2: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, field_value: &[(T1, T2)]) -> RedisResult<RV> {
        Cmd::hmset(key, field_value).query(self)
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
    fn hrandfield<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, options: Option<T0>) -> RedisResult<RV> {
        Cmd::hrandfield(key, options).query(self)
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
    fn hset<K0: ToRedisArgs, T1: ToRedisArgs, T2: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, field_value: &[(T1, T2)]) -> RedisResult<RV> {
        Cmd::hset(key, field_value).query(self)
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
    fn hsetnx<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, field: T0, value: T1) -> RedisResult<RV> {
        Cmd::hsetnx(key, field, value).query(self)
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
    fn hstrlen<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, field: T0) -> RedisResult<RV> {
        Cmd::hstrlen(key, field).query(self)
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
    fn hvals<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::hvals(key).query(self)
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
    fn psubscribe<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, pattern: &[K0]) -> RedisResult<RV> {
        Cmd::psubscribe(pattern).query(self)
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
    fn publish<T0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, channel: T0, message: T1) -> RedisResult<RV> {
        Cmd::publish(channel, message).query(self)
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
    fn pubsub<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::pubsub().query(self)
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
    fn pubsub_channels<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, pattern: Option<K0>) -> RedisResult<RV> {
        Cmd::pubsub_channels(pattern).query(self)
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
    fn pubsub_help<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::pubsub_help().query(self)
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
    fn pubsub_numpat<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::pubsub_numpat().query(self)
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
    fn pubsub_numsub<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, channel: Option<&[T0]>) -> RedisResult<RV> {
        Cmd::pubsub_numsub(channel).query(self)
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
    fn pubsub_shardchannels<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, pattern: Option<K0>) -> RedisResult<RV> {
        Cmd::pubsub_shardchannels(pattern).query(self)
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
    fn pubsub_shardnumsub<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, shardchannel: Option<&[T0]>) -> RedisResult<RV> {
        Cmd::pubsub_shardnumsub(shardchannel).query(self)
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
    fn punsubscribe<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, pattern: Option<&[K0]>) -> RedisResult<RV> {
        Cmd::punsubscribe(pattern).query(self)
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
    fn spublish<T0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, shardchannel: T0, message: T1) -> RedisResult<RV> {
        Cmd::spublish(shardchannel, message).query(self)
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
    fn ssubscribe<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, shardchannel: &[T0]) -> RedisResult<RV> {
        Cmd::ssubscribe(shardchannel).query(self)
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
    fn subscribe<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, channel: &[T0]) -> RedisResult<RV> {
        Cmd::subscribe(channel).query(self)
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
    fn sunsubscribe<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, shardchannel: Option<&[T0]>) -> RedisResult<RV> {
        Cmd::sunsubscribe(shardchannel).query(self)
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
    fn unsubscribe<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, channel: Option<&[T0]>) -> RedisResult<RV> {
        Cmd::unsubscribe(channel).query(self)
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
    fn discard<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::discard().query(self)
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
    fn exec<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::exec().query(self)
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
    fn multi<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::multi().query(self)
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
    fn unwatch<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::unwatch().query(self)
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
    fn watch<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: &[K0]) -> RedisResult<RV> {
        Cmd::watch(key).query(self)
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
    fn auth<T0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, username: Option<T0>, password: T1) -> RedisResult<RV> {
        Cmd::auth(username, password).query(self)
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
    fn client<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::client().query(self)
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
    fn client_caching<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::client_caching().query(self)
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
    fn client_getname<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::client_getname().query(self)
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
    fn client_getredir<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::client_getredir().query(self)
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
    fn client_help<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::client_help().query(self)
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
    fn client_id<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::client_id().query(self)
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
    fn client_info<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::client_info().query(self)
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
    fn client_list<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::client_list().query(self)
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
    fn client_no_evict<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::client_no_evict().query(self)
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
    fn client_pause<RV: FromRedisValue>(&mut self, timeout: i64) -> RedisResult<RV> {
        Cmd::client_pause(timeout).query(self)
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
    fn client_reply<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::client_reply().query(self)
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
    fn client_setname<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, connection_name: T0) -> RedisResult<RV> {
        Cmd::client_setname(connection_name).query(self)
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
    fn client_tracking<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::client_tracking().query(self)
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
    fn client_trackinginfo<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::client_trackinginfo().query(self)
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
    fn client_unblock<RV: FromRedisValue>(&mut self, client_id: i64) -> RedisResult<RV> {
        Cmd::client_unblock(client_id).query(self)
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
    fn client_unpause<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::client_unpause().query(self)
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
    fn echo<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, message: T0) -> RedisResult<RV> {
        Cmd::echo(message).query(self)
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
    fn hello<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, arguments: Option<T0>) -> RedisResult<RV> {
        Cmd::hello(arguments).query(self)
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
    fn ping<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, message: Option<T0>) -> RedisResult<RV> {
        Cmd::ping(message).query(self)
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
    fn quit<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::quit().query(self)
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
    fn reset<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::reset().query(self)
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
    fn select<RV: FromRedisValue>(&mut self, index: i64) -> RedisResult<RV> {
        Cmd::select(index).query(self)
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
    fn acl<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::acl().query(self)
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
    fn acl_cat<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, categoryname: Option<T0>) -> RedisResult<RV> {
        Cmd::acl_cat(categoryname).query(self)
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
    fn acl_deluser<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, username: &[T0]) -> RedisResult<RV> {
        Cmd::acl_deluser(username).query(self)
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
    fn acl_dryrun<T0: ToRedisArgs, T1: ToRedisArgs, T2: ToRedisArgs, RV: FromRedisValue>(&mut self, username: T0, command: T1, arg: Option<&[T2]>) -> RedisResult<RV> {
        Cmd::acl_dryrun(username, command, arg).query(self)
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
    fn acl_genpass<RV: FromRedisValue>(&mut self, bits: Option<i64>) -> RedisResult<RV> {
        Cmd::acl_genpass(bits).query(self)
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
    fn acl_getuser<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, username: T0) -> RedisResult<RV> {
        Cmd::acl_getuser(username).query(self)
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
    fn acl_help<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::acl_help().query(self)
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
    fn acl_list<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::acl_list().query(self)
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
    fn acl_load<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::acl_load().query(self)
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
    fn acl_log<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::acl_log().query(self)
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
    fn acl_save<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::acl_save().query(self)
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
    fn acl_setuser<T0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, username: T0, rule: Option<&[T1]>) -> RedisResult<RV> {
        Cmd::acl_setuser(username, rule).query(self)
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
    fn acl_users<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::acl_users().query(self)
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
    fn acl_whoami<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::acl_whoami().query(self)
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
    fn bgrewriteaof<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::bgrewriteaof().query(self)
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
    fn bgsave<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::bgsave().query(self)
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
    fn command<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::command().query(self)
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
    fn command_count<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::command_count().query(self)
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
    fn command_docs<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, command_name: Option<&[T0]>) -> RedisResult<RV> {
        Cmd::command_docs(command_name).query(self)
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
    fn command_getkeys<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::command_getkeys().query(self)
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
    fn command_getkeysandflags<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::command_getkeysandflags().query(self)
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
    fn command_help<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::command_help().query(self)
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
    fn command_info<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, command_name: Option<&[T0]>) -> RedisResult<RV> {
        Cmd::command_info(command_name).query(self)
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
    fn command_list<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::command_list().query(self)
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
    fn config<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::config().query(self)
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
    fn config_get<T1: ToRedisArgs, RV: FromRedisValue>(&mut self, parameter: &[T1]) -> RedisResult<RV> {
        Cmd::config_get(parameter).query(self)
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
    fn config_help<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::config_help().query(self)
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
    fn config_resetstat<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::config_resetstat().query(self)
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
    fn config_rewrite<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::config_rewrite().query(self)
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
    fn config_set<T1: ToRedisArgs, T2: ToRedisArgs, RV: FromRedisValue>(&mut self, parameter_value: &[(T1, T2)]) -> RedisResult<RV> {
        Cmd::config_set(parameter_value).query(self)
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
    fn dbsize<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::dbsize().query(self)
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
    fn debug<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::debug().query(self)
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
    fn failover<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::failover().query(self)
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
    fn flushall<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::flushall().query(self)
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
    fn flushdb<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::flushdb().query(self)
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
    fn info<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, section: Option<&[T0]>) -> RedisResult<RV> {
        Cmd::info(section).query(self)
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
    fn lastsave<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::lastsave().query(self)
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
    fn latency<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::latency().query(self)
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
    fn latency_doctor<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::latency_doctor().query(self)
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
    fn latency_graph<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, event: T0) -> RedisResult<RV> {
        Cmd::latency_graph(event).query(self)
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
    fn latency_help<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::latency_help().query(self)
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
    fn latency_histogram<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, command: Option<&[T0]>) -> RedisResult<RV> {
        Cmd::latency_histogram(command).query(self)
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
    fn latency_history<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, event: T0) -> RedisResult<RV> {
        Cmd::latency_history(event).query(self)
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
    fn latency_latest<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::latency_latest().query(self)
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
    fn latency_reset<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, event: Option<&[T0]>) -> RedisResult<RV> {
        Cmd::latency_reset(event).query(self)
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
    fn lolwut<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::lolwut().query(self)
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
    fn memory<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::memory().query(self)
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
    fn memory_doctor<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::memory_doctor().query(self)
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
    fn memory_help<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::memory_help().query(self)
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
    fn memory_malloc_stats<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::memory_malloc_stats().query(self)
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
    fn memory_purge<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::memory_purge().query(self)
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
    fn memory_stats<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::memory_stats().query(self)
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
    fn memory_usage<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::memory_usage(key).query(self)
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
    fn module<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::module().query(self)
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
    fn module_help<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::module_help().query(self)
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
    fn module_list<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::module_list().query(self)
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
    fn module_load<T0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, path: T0, arg: Option<&[T1]>) -> RedisResult<RV> {
        Cmd::module_load(path, arg).query(self)
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
    fn module_loadex<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, path: T0) -> RedisResult<RV> {
        Cmd::module_loadex(path).query(self)
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
    fn module_unload<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, name: T0) -> RedisResult<RV> {
        Cmd::module_unload(name).query(self)
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
    fn monitor<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::monitor().query(self)
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
    fn psync<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, replicationid: T0, offset: i64) -> RedisResult<RV> {
        Cmd::psync(replicationid, offset).query(self)
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
    fn replconf<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::replconf().query(self)
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
    fn replicaof<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, host: T0, port: i64) -> RedisResult<RV> {
        Cmd::replicaof(host, port).query(self)
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
    fn restore_asking<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, ttl: i64, serialized_value: T0) -> RedisResult<RV> {
        Cmd::restore_asking(key, ttl, serialized_value).query(self)
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
    fn role<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::role().query(self)
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
    fn save<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::save().query(self)
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
    fn shutdown<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::shutdown().query(self)
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
    #[deprecated = "Deprecated in redis since redis version 5.0.0."]
    fn slaveof<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, host: T0, port: i64) -> RedisResult<RV> {
        Cmd::slaveof(host, port).query(self)
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
    fn slowlog<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::slowlog().query(self)
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
    fn slowlog_get<RV: FromRedisValue>(&mut self, count: Option<i64>) -> RedisResult<RV> {
        Cmd::slowlog_get(count).query(self)
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
    fn slowlog_help<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::slowlog_help().query(self)
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
    fn slowlog_len<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::slowlog_len().query(self)
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
    fn slowlog_reset<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::slowlog_reset().query(self)
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
    fn swapdb<RV: FromRedisValue>(&mut self, index1: i64, index2: i64) -> RedisResult<RV> {
        Cmd::swapdb(index1, index2).query(self)
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
    fn sync<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::sync().query(self)
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
    fn time<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::time().query(self)
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
    fn eval<T0: ToRedisArgs, K0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, script: T0, numkeys: i64, key: Option<&[K0]>, arg: Option<&[T1]>) -> RedisResult<RV> {
        Cmd::eval(script, numkeys, key, arg).query(self)
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
    fn evalsha<T0: ToRedisArgs, K0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, sha1: T0, numkeys: i64, key: Option<&[K0]>, arg: Option<&[T1]>) -> RedisResult<RV> {
        Cmd::evalsha(sha1, numkeys, key, arg).query(self)
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
    fn evalsha_ro<T0: ToRedisArgs, K0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, sha1: T0, numkeys: i64, key: &[K0], arg: &[T1]) -> RedisResult<RV> {
        Cmd::evalsha_ro(sha1, numkeys, key, arg).query(self)
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
    fn eval_ro<T0: ToRedisArgs, K0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, script: T0, numkeys: i64, key: &[K0], arg: &[T1]) -> RedisResult<RV> {
        Cmd::eval_ro(script, numkeys, key, arg).query(self)
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
    fn fcall<T0: ToRedisArgs, K0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, function: T0, numkeys: i64, key: &[K0], arg: &[T1]) -> RedisResult<RV> {
        Cmd::fcall(function, numkeys, key, arg).query(self)
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
    fn fcall_ro<T0: ToRedisArgs, K0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, function: T0, numkeys: i64, key: &[K0], arg: &[T1]) -> RedisResult<RV> {
        Cmd::fcall_ro(function, numkeys, key, arg).query(self)
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
    fn function<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::function().query(self)
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
    fn function_delete<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, library_name: T0) -> RedisResult<RV> {
        Cmd::function_delete(library_name).query(self)
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
    fn function_dump<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::function_dump().query(self)
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
    fn function_flush<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::function_flush().query(self)
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
    fn function_help<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::function_help().query(self)
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
    fn function_kill<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::function_kill().query(self)
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
    fn function_list<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::function_list().query(self)
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
    fn function_load<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, function_code: T0) -> RedisResult<RV> {
        Cmd::function_load(function_code).query(self)
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
    fn function_restore<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, serialized_value: T0) -> RedisResult<RV> {
        Cmd::function_restore(serialized_value).query(self)
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
    fn function_stats<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::function_stats().query(self)
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
    fn script<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::script().query(self)
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
    fn script_debug<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::script_debug().query(self)
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
    fn script_exists<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, sha1: &[T0]) -> RedisResult<RV> {
        Cmd::script_exists(sha1).query(self)
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
    fn script_flush<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::script_flush().query(self)
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
    fn script_help<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::script_help().query(self)
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
    fn script_kill<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::script_kill().query(self)
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
    fn script_load<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, script: T0) -> RedisResult<RV> {
        Cmd::script_load(script).query(self)
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
    fn pfadd<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, element: Option<&[T0]>) -> RedisResult<RV> {
        Cmd::pfadd(key, element).query(self)
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
    fn pfcount<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: &[K0]) -> RedisResult<RV> {
        Cmd::pfcount(key).query(self)
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
    fn pfdebug<T0: ToRedisArgs, K0: ToRedisArgs, RV: FromRedisValue>(&mut self, subcommand: T0, key: K0) -> RedisResult<RV> {
        Cmd::pfdebug(subcommand, key).query(self)
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
    fn pfmerge<K0: ToRedisArgs, K1: ToRedisArgs, RV: FromRedisValue>(&mut self, destkey: K0, sourcekey: &[K1]) -> RedisResult<RV> {
        Cmd::pfmerge(destkey, sourcekey).query(self)
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
    fn pfselftest<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::pfselftest().query(self)
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
    fn asking<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::asking().query(self)
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
    fn cluster<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::cluster().query(self)
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
    fn cluster_addslots<RV: FromRedisValue>(&mut self, slot: &[i64]) -> RedisResult<RV> {
        Cmd::cluster_addslots(slot).query(self)
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
    fn cluster_addslotsrange<RV: FromRedisValue>(&mut self, start_slot_end_slot: &[(i64, i64)]) -> RedisResult<RV> {
        Cmd::cluster_addslotsrange(start_slot_end_slot).query(self)
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
    fn cluster_bumpepoch<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::cluster_bumpepoch().query(self)
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
    fn cluster_count_failure_reports<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, node_id: T0) -> RedisResult<RV> {
        Cmd::cluster_count_failure_reports(node_id).query(self)
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
    fn cluster_countkeysinslot<RV: FromRedisValue>(&mut self, slot: i64) -> RedisResult<RV> {
        Cmd::cluster_countkeysinslot(slot).query(self)
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
    fn cluster_delslots<RV: FromRedisValue>(&mut self, slot: &[i64]) -> RedisResult<RV> {
        Cmd::cluster_delslots(slot).query(self)
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
    fn cluster_delslotsrange<RV: FromRedisValue>(&mut self, start_slot_end_slot: &[(i64, i64)]) -> RedisResult<RV> {
        Cmd::cluster_delslotsrange(start_slot_end_slot).query(self)
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
    fn cluster_failover<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::cluster_failover().query(self)
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
    fn cluster_flushslots<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::cluster_flushslots().query(self)
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
    fn cluster_forget<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, node_id: T0) -> RedisResult<RV> {
        Cmd::cluster_forget(node_id).query(self)
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
    fn cluster_getkeysinslot<RV: FromRedisValue>(&mut self, slot: i64, count: i64) -> RedisResult<RV> {
        Cmd::cluster_getkeysinslot(slot, count).query(self)
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
    fn cluster_help<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::cluster_help().query(self)
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
    fn cluster_info<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::cluster_info().query(self)
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
    fn cluster_keyslot<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: T0) -> RedisResult<RV> {
        Cmd::cluster_keyslot(key).query(self)
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
    fn cluster_links<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::cluster_links().query(self)
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
    fn cluster_meet<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, ip: T0, port: i64) -> RedisResult<RV> {
        Cmd::cluster_meet(ip, port).query(self)
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
    fn cluster_myid<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::cluster_myid().query(self)
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
    fn cluster_nodes<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::cluster_nodes().query(self)
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
    fn cluster_replicas<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, node_id: T0) -> RedisResult<RV> {
        Cmd::cluster_replicas(node_id).query(self)
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
    fn cluster_replicate<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, node_id: T0) -> RedisResult<RV> {
        Cmd::cluster_replicate(node_id).query(self)
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
    fn cluster_reset<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::cluster_reset().query(self)
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
    fn cluster_saveconfig<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::cluster_saveconfig().query(self)
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
    fn cluster_set_config_epoch<RV: FromRedisValue>(&mut self, config_epoch: i64) -> RedisResult<RV> {
        Cmd::cluster_set_config_epoch(config_epoch).query(self)
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
    fn cluster_setslot<RV: FromRedisValue>(&mut self, slot: i64) -> RedisResult<RV> {
        Cmd::cluster_setslot(slot).query(self)
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
    fn cluster_shards<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::cluster_shards().query(self)
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
    #[deprecated = "Deprecated in redis since redis version 5.0.0."]
    fn cluster_slaves<T0: ToRedisArgs, RV: FromRedisValue>(&mut self, node_id: T0) -> RedisResult<RV> {
        Cmd::cluster_slaves(node_id).query(self)
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
    #[deprecated = "Deprecated in redis since redis version 7.0.0."]
    fn cluster_slots<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::cluster_slots().query(self)
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
    fn readonly<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::readonly().query(self)
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
    fn readwrite<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::readwrite().query(self)
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
    fn geoadd<K0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, longitude_latitude_member: &[(f64, f64, T1)]) -> RedisResult<RV> {
        Cmd::geoadd(key, longitude_latitude_member).query(self)
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
    fn geodist<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, member1: T0, member2: T1) -> RedisResult<RV> {
        Cmd::geodist(key, member1, member2).query(self)
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
    fn geohash<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, member: &[T0]) -> RedisResult<RV> {
        Cmd::geohash(key, member).query(self)
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
    fn geopos<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, member: &[T0]) -> RedisResult<RV> {
        Cmd::geopos(key, member).query(self)
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
    #[deprecated = "Deprecated in redis since redis version 6.2.0."]
    fn georadius<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, longitude: f64, latitude: f64, radius: f64, count: Option<T0>) -> RedisResult<RV> {
        Cmd::georadius(key, longitude, latitude, radius, count).query(self)
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
    #[deprecated = "Deprecated in redis since redis version 6.2.0."]
    fn georadiusbymember<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, member: T0, radius: f64, count: Option<T1>) -> RedisResult<RV> {
        Cmd::georadiusbymember(key, member, radius, count).query(self)
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
    #[deprecated = "Deprecated in redis since redis version 6.2.0."]
    fn georadiusbymember_ro<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, member: T0, radius: f64, count: Option<T1>) -> RedisResult<RV> {
        Cmd::georadiusbymember_ro(key, member, radius, count).query(self)
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
    #[deprecated = "Deprecated in redis since redis version 6.2.0."]
    fn georadius_ro<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, longitude: f64, latitude: f64, radius: f64, count: Option<T0>) -> RedisResult<RV> {
        Cmd::georadius_ro(key, longitude, latitude, radius, count).query(self)
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
    fn geosearch<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, count: Option<T0>) -> RedisResult<RV> {
        Cmd::geosearch(key, count).query(self)
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
    fn geosearchstore<K0: ToRedisArgs, K1: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, destination: K0, source: K1, count: Option<T0>) -> RedisResult<RV> {
        Cmd::geosearchstore(destination, source, count).query(self)
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
    fn xack<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, group: T0, id: &[T1]) -> RedisResult<RV> {
        Cmd::xack(key, group, id).query(self)
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
    fn xadd<K0: ToRedisArgs, T0: ToRedisArgs, T2: ToRedisArgs, T3: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, trim: Option<T0>, field_value: &[(T2, T3)]) -> RedisResult<RV> {
        Cmd::xadd(key, trim, field_value).query(self)
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
    fn xautoclaim<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs, T2: ToRedisArgs, T3: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, group: T0, consumer: T1, min_idle_time: T2, start: T3) -> RedisResult<RV> {
        Cmd::xautoclaim(key, group, consumer, min_idle_time, start).query(self)
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
    fn xclaim<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs, T2: ToRedisArgs, T3: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, group: T0, consumer: T1, min_idle_time: T2, id: &[T3]) -> RedisResult<RV> {
        Cmd::xclaim(key, group, consumer, min_idle_time, id).query(self)
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
    fn xdel<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, id: &[T0]) -> RedisResult<RV> {
        Cmd::xdel(key, id).query(self)
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
    fn xgroup<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::xgroup().query(self)
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
    fn xgroup_create<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, groupname: T0) -> RedisResult<RV> {
        Cmd::xgroup_create(key, groupname).query(self)
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
    fn xgroup_createconsumer<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, groupname: T0, consumername: T1) -> RedisResult<RV> {
        Cmd::xgroup_createconsumer(key, groupname, consumername).query(self)
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
    fn xgroup_delconsumer<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, groupname: T0, consumername: T1) -> RedisResult<RV> {
        Cmd::xgroup_delconsumer(key, groupname, consumername).query(self)
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
    fn xgroup_destroy<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, groupname: T0) -> RedisResult<RV> {
        Cmd::xgroup_destroy(key, groupname).query(self)
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
    fn xgroup_help<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::xgroup_help().query(self)
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
    fn xgroup_setid<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, groupname: T0) -> RedisResult<RV> {
        Cmd::xgroup_setid(key, groupname).query(self)
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
    fn xinfo<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::xinfo().query(self)
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
    fn xinfo_consumers<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, groupname: T0) -> RedisResult<RV> {
        Cmd::xinfo_consumers(key, groupname).query(self)
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
    fn xinfo_groups<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::xinfo_groups(key).query(self)
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
    fn xinfo_help<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::xinfo_help().query(self)
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
    fn xinfo_stream<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::xinfo_stream(key).query(self)
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
    fn xlen<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::xlen(key).query(self)
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
    fn xpending<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, group: T0, filters: Option<T1>) -> RedisResult<RV> {
        Cmd::xpending(key, group, filters).query(self)
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
    fn xrange<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, start: T0, end: T1) -> RedisResult<RV> {
        Cmd::xrange(key, start, end).query(self)
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
    fn xread<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::xread().query(self)
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
    fn xreadgroup<RV: FromRedisValue>(&mut self) -> RedisResult<RV> {
        Cmd::xreadgroup().query(self)
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
    fn xrevrange<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, end: T0, start: T1) -> RedisResult<RV> {
        Cmd::xrevrange(key, end, start).query(self)
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
    fn xsetid<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, last_id: T0) -> RedisResult<RV> {
        Cmd::xsetid(key, last_id).query(self)
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
    fn xtrim<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, trim: T0) -> RedisResult<RV> {
        Cmd::xtrim(key, trim).query(self)
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
    fn bitcount<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, index: Option<T0>) -> RedisResult<RV> {
        Cmd::bitcount(key, index).query(self)
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
    fn bitfield<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::bitfield(key).query(self)
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
    fn bitfield_ro<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0) -> RedisResult<RV> {
        Cmd::bitfield_ro(key).query(self)
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
    fn bitop<T0: ToRedisArgs, K0: ToRedisArgs, K1: ToRedisArgs, RV: FromRedisValue>(&mut self, operation: T0, destkey: K0, key: &[K1]) -> RedisResult<RV> {
        Cmd::bitop(operation, destkey, key).query(self)
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
    fn bitpos<K0: ToRedisArgs, T0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, bit: i64, index: Option<T0>) -> RedisResult<RV> {
        Cmd::bitpos(key, bit, index).query(self)
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
    fn getbit<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, offset: i64) -> RedisResult<RV> {
        Cmd::getbit(key, offset).query(self)
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
    fn setbit<K0: ToRedisArgs, RV: FromRedisValue>(&mut self, key: K0, offset: i64, value: i64) -> RedisResult<RV> {
        Cmd::setbit(key, offset, value).query(self)
    }

}
