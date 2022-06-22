use crate::types::{FromRedisValue, NumericBehavior, RedisResult, ToRedisArgs, RedisWrite, Expiry};
use crate::connection::{Connection, ConnectionLike, Msg};
use crate::cmd::{cmd, Cmd};

impl Cmd {
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
    pub fn copy<K0: ToRedisArgs, K1: ToRedisArgs>(source: K0, destination: K1) -> Self {
        let mut rv = Cmd::new();
        rv.arg("COPY");
        rv.arg(source);
        rv.arg(destination);
        rv
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
    pub fn del<K0: ToRedisArgs>(key: &[K0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("DEL");
        rv.arg(key);
        rv
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
    pub fn dump<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("DUMP");
        rv.arg(key);
        rv
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
    pub fn exists<K0: ToRedisArgs>(key: &[K0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("EXISTS");
        rv.arg(key);
        rv
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
    pub fn expire<K0: ToRedisArgs>(key: K0, seconds: i64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("EXPIRE");
        rv.arg(key);
        rv.arg(seconds);
        rv
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
    pub fn expireat<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("EXPIREAT");
        rv.arg(key);
        rv
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
    pub fn expiretime<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("EXPIRETIME");
        rv.arg(key);
        rv
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
    pub fn keys<K0: ToRedisArgs>(pattern: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("KEYS");
        rv.arg(pattern);
        rv
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
    pub fn migrate<T0: ToRedisArgs>(host: T0, port: i64, destination_db: i64, timeout: i64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("MIGRATE");
        rv.arg(host);
        rv.arg(port);
        rv.arg(destination_db);
        rv.arg(timeout);
        rv
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
    pub fn move_key<K0: ToRedisArgs>(key: K0, db: i64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("MOVE");
        rv.arg(key);
        rv.arg(db);
        rv
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
    pub fn object_encoding<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("OBJECT ENCODING");
        rv.arg(key);
        rv
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
    pub fn object_freq<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("OBJECT FREQ");
        rv.arg(key);
        rv
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
    pub fn object_help() -> Self {
        let mut rv = Cmd::new();
        rv.arg("OBJECT HELP");
        rv
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
    pub fn object_idletime<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("OBJECT IDLETIME");
        rv.arg(key);
        rv
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
    pub fn object_refcount<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("OBJECT REFCOUNT");
        rv.arg(key);
        rv
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
    pub fn persist<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("PERSIST");
        rv.arg(key);
        rv
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
    pub fn pexpire<K0: ToRedisArgs>(key: K0, milliseconds: i64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("PEXPIRE");
        rv.arg(key);
        rv.arg(milliseconds);
        rv
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
    pub fn pexpireat<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("PEXPIREAT");
        rv.arg(key);
        rv
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
    pub fn pexpiretime<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("PEXPIRETIME");
        rv.arg(key);
        rv
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
    pub fn pttl<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("PTTL");
        rv.arg(key);
        rv
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
    pub fn randomkey() -> Self {
        let mut rv = Cmd::new();
        rv.arg("RANDOMKEY");
        rv
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
    pub fn rename<K0: ToRedisArgs, K1: ToRedisArgs>(key: K0, newkey: K1) -> Self {
        let mut rv = Cmd::new();
        rv.arg("RENAME");
        rv.arg(key);
        rv.arg(newkey);
        rv
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
    pub fn renamenx<K0: ToRedisArgs, K1: ToRedisArgs>(key: K0, newkey: K1) -> Self {
        let mut rv = Cmd::new();
        rv.arg("RENAMENX");
        rv.arg(key);
        rv.arg(newkey);
        rv
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
    pub fn restore<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, ttl: i64, serialized_value: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("RESTORE");
        rv.arg(key);
        rv.arg(ttl);
        rv.arg(serialized_value);
        rv
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
    pub fn sort<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SORT");
        rv.arg(key);
        rv
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
    pub fn sort_ro<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SORT_RO");
        rv.arg(key);
        rv
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
    pub fn touch<K0: ToRedisArgs>(key: &[K0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("TOUCH");
        rv.arg(key);
        rv
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
    pub fn ttl<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("TTL");
        rv.arg(key);
        rv
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
    pub fn r#type<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("TYPE");
        rv.arg(key);
        rv
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
    pub fn unlink<K0: ToRedisArgs>(key: &[K0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("UNLINK");
        rv.arg(key);
        rv
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
    pub fn wait(numreplicas: i64, timeout: i64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("WAIT");
        rv.arg(numreplicas);
        rv.arg(timeout);
        rv
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
    pub fn append<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, value: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("APPEND");
        rv.arg(key);
        rv.arg(value);
        rv
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
    pub fn decr<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("DECR");
        rv.arg(key);
        rv
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
    pub fn decrby<K0: ToRedisArgs>(key: K0, decrement: i64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("DECRBY");
        rv.arg(key);
        rv.arg(decrement);
        rv
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
    pub fn get<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("GET");
        rv.arg(key);
        rv
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
    pub fn getdel<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("GETDEL");
        rv.arg(key);
        rv
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
    pub fn getex<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("GETEX");
        rv.arg(key);
        rv
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
    pub fn getrange<K0: ToRedisArgs>(key: K0, start: i64, end: i64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("GETRANGE");
        rv.arg(key);
        rv.arg(start);
        rv.arg(end);
        rv
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
    pub fn getset<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, value: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("GETSET");
        rv.arg(key);
        rv.arg(value);
        rv
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
    pub fn incr<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("INCR");
        rv.arg(key);
        rv
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
    pub fn incrby<K0: ToRedisArgs>(key: K0, increment: i64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("INCRBY");
        rv.arg(key);
        rv.arg(increment);
        rv
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
    pub fn incrbyfloat<K0: ToRedisArgs>(key: K0, increment: f64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("INCRBYFLOAT");
        rv.arg(key);
        rv.arg(increment);
        rv
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
    pub fn lcs<K0: ToRedisArgs, K1: ToRedisArgs>(key1: K0, key2: K1) -> Self {
        let mut rv = Cmd::new();
        rv.arg("LCS");
        rv.arg(key1);
        rv.arg(key2);
        rv
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
    pub fn mget<K0: ToRedisArgs>(key: &[K0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("MGET");
        rv.arg(key);
        rv
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
    pub fn mset<K0: ToRedisArgs, T1: ToRedisArgs>(key_value: &[(K0, T1)]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("MSET");
        rv.arg(key_value);
        rv
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
    pub fn msetnx<K0: ToRedisArgs, T1: ToRedisArgs>(key_value: &[(K0, T1)]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("MSETNX");
        rv.arg(key_value);
        rv
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
    pub fn psetex<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, milliseconds: i64, value: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("PSETEX");
        rv.arg(key);
        rv.arg(milliseconds);
        rv.arg(value);
        rv
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
    pub fn set<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, value: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SET");
        rv.arg(key);
        rv.arg(value);
        rv
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
    pub fn setex<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, seconds: i64, value: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SETEX");
        rv.arg(key);
        rv.arg(seconds);
        rv.arg(value);
        rv
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
    pub fn setnx<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, value: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SETNX");
        rv.arg(key);
        rv.arg(value);
        rv
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
    pub fn setrange<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, offset: i64, value: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SETRANGE");
        rv.arg(key);
        rv.arg(offset);
        rv.arg(value);
        rv
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
    pub fn strlen<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("STRLEN");
        rv.arg(key);
        rv
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
    pub fn substr<K0: ToRedisArgs>(key: K0, start: i64, end: i64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SUBSTR");
        rv.arg(key);
        rv.arg(start);
        rv.arg(end);
        rv
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
    pub fn blmove<K0: ToRedisArgs, K1: ToRedisArgs>(source: K0, destination: K1, timeout: f64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("BLMOVE");
        rv.arg(source);
        rv.arg(destination);
        rv.arg(timeout);
        rv
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
    pub fn blmpop<K0: ToRedisArgs>(timeout: f64, numkeys: i64, key: &[K0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("BLMPOP");
        rv.arg(timeout);
        rv.arg(numkeys);
        rv.arg(key);
        rv
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
    pub fn blpop<K0: ToRedisArgs>(key: &[K0], timeout: f64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("BLPOP");
        rv.arg(key);
        rv.arg(timeout);
        rv
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
    pub fn brpop<K0: ToRedisArgs>(key: &[K0], timeout: f64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("BRPOP");
        rv.arg(key);
        rv.arg(timeout);
        rv
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
    pub fn brpoplpush<K0: ToRedisArgs, K1: ToRedisArgs>(source: K0, destination: K1, timeout: f64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("BRPOPLPUSH");
        rv.arg(source);
        rv.arg(destination);
        rv.arg(timeout);
        rv
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
    pub fn lindex<K0: ToRedisArgs>(key: K0, index: i64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("LINDEX");
        rv.arg(key);
        rv.arg(index);
        rv
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
    pub fn linsert<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(key: K0, pivot: T0, element: T1) -> Self {
        let mut rv = Cmd::new();
        rv.arg("LINSERT");
        rv.arg(key);
        rv.arg(pivot);
        rv.arg(element);
        rv
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
    pub fn llen<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("LLEN");
        rv.arg(key);
        rv
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
    pub fn lmove<K0: ToRedisArgs, K1: ToRedisArgs>(source: K0, destination: K1) -> Self {
        let mut rv = Cmd::new();
        rv.arg("LMOVE");
        rv.arg(source);
        rv.arg(destination);
        rv
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
    pub fn lmpop<K0: ToRedisArgs>(numkeys: i64, key: &[K0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("LMPOP");
        rv.arg(numkeys);
        rv.arg(key);
        rv
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
    pub fn lpop<K0: ToRedisArgs>(key: K0, count: Option<i64>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("LPOP");
        rv.arg(key);
        rv.arg(count);
        rv
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
    pub fn lpos<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, element: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("LPOS");
        rv.arg(key);
        rv.arg(element);
        rv
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
    pub fn lpush<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, element: &[T0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("LPUSH");
        rv.arg(key);
        rv.arg(element);
        rv
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
    pub fn lpushx<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, element: &[T0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("LPUSHX");
        rv.arg(key);
        rv.arg(element);
        rv
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
    pub fn lrange<K0: ToRedisArgs>(key: K0, start: i64, stop: i64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("LRANGE");
        rv.arg(key);
        rv.arg(start);
        rv.arg(stop);
        rv
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
    pub fn lrem<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, count: i64, element: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("LREM");
        rv.arg(key);
        rv.arg(count);
        rv.arg(element);
        rv
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
    pub fn lset<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, index: i64, element: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("LSET");
        rv.arg(key);
        rv.arg(index);
        rv.arg(element);
        rv
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
    pub fn ltrim<K0: ToRedisArgs>(key: K0, start: i64, stop: i64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("LTRIM");
        rv.arg(key);
        rv.arg(start);
        rv.arg(stop);
        rv
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
    pub fn rpop<K0: ToRedisArgs>(key: K0, count: Option<i64>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("RPOP");
        rv.arg(key);
        rv.arg(count);
        rv
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
    pub fn rpoplpush<K0: ToRedisArgs, K1: ToRedisArgs>(source: K0, destination: K1) -> Self {
        let mut rv = Cmd::new();
        rv.arg("RPOPLPUSH");
        rv.arg(source);
        rv.arg(destination);
        rv
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
    pub fn rpush<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, element: &[T0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("RPUSH");
        rv.arg(key);
        rv.arg(element);
        rv
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
    pub fn rpushx<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, element: &[T0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("RPUSHX");
        rv.arg(key);
        rv.arg(element);
        rv
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
    pub fn sadd<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, member: &[T0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SADD");
        rv.arg(key);
        rv.arg(member);
        rv
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
    pub fn scard<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SCARD");
        rv.arg(key);
        rv
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
    pub fn sdiff<K0: ToRedisArgs>(key: &[K0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SDIFF");
        rv.arg(key);
        rv
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
    pub fn sdiffstore<K0: ToRedisArgs, K1: ToRedisArgs>(destination: K0, key: &[K1]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SDIFFSTORE");
        rv.arg(destination);
        rv.arg(key);
        rv
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
    pub fn sinter<K0: ToRedisArgs>(key: &[K0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SINTER");
        rv.arg(key);
        rv
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
    pub fn sintercard<K0: ToRedisArgs>(numkeys: i64, key: &[K0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SINTERCARD");
        rv.arg(numkeys);
        rv.arg(key);
        rv
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
    pub fn sinterstore<K0: ToRedisArgs, K1: ToRedisArgs>(destination: K0, key: &[K1]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SINTERSTORE");
        rv.arg(destination);
        rv.arg(key);
        rv
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
    pub fn sismember<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, member: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SISMEMBER");
        rv.arg(key);
        rv.arg(member);
        rv
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
    pub fn smembers<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SMEMBERS");
        rv.arg(key);
        rv
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
    pub fn smismember<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, member: &[T0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SMISMEMBER");
        rv.arg(key);
        rv.arg(member);
        rv
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
    pub fn smove<K0: ToRedisArgs, K1: ToRedisArgs, T0: ToRedisArgs>(source: K0, destination: K1, member: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SMOVE");
        rv.arg(source);
        rv.arg(destination);
        rv.arg(member);
        rv
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
    pub fn spop<K0: ToRedisArgs>(key: K0, count: Option<i64>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SPOP");
        rv.arg(key);
        rv.arg(count);
        rv
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
    pub fn srandmember<K0: ToRedisArgs>(key: K0, count: Option<i64>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SRANDMEMBER");
        rv.arg(key);
        rv.arg(count);
        rv
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
    pub fn srem<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, member: &[T0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SREM");
        rv.arg(key);
        rv.arg(member);
        rv
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
    pub fn sunion<K0: ToRedisArgs>(key: &[K0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SUNION");
        rv.arg(key);
        rv
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
    pub fn sunionstore<K0: ToRedisArgs, K1: ToRedisArgs>(destination: K0, key: &[K1]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SUNIONSTORE");
        rv.arg(destination);
        rv.arg(key);
        rv
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
    pub fn bzmpop<K0: ToRedisArgs>(timeout: f64, numkeys: i64, key: &[K0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("BZMPOP");
        rv.arg(timeout);
        rv.arg(numkeys);
        rv.arg(key);
        rv
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
    pub fn bzpopmax<K0: ToRedisArgs>(key: &[K0], timeout: f64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("BZPOPMAX");
        rv.arg(key);
        rv.arg(timeout);
        rv
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
    pub fn bzpopmin<K0: ToRedisArgs>(key: &[K0], timeout: f64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("BZPOPMIN");
        rv.arg(key);
        rv.arg(timeout);
        rv
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
    pub fn zadd<K0: ToRedisArgs, T1: ToRedisArgs>(key: K0, score_member: &[(f64, T1)]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZADD");
        rv.arg(key);
        rv.arg(score_member);
        rv
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
    pub fn zcard<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZCARD");
        rv.arg(key);
        rv
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
    pub fn zcount<K0: ToRedisArgs>(key: K0, min: f64, max: f64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZCOUNT");
        rv.arg(key);
        rv.arg(min);
        rv.arg(max);
        rv
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
    pub fn zdiff<K0: ToRedisArgs>(numkeys: i64, key: &[K0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZDIFF");
        rv.arg(numkeys);
        rv.arg(key);
        rv
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
    pub fn zdiffstore<K0: ToRedisArgs, K1: ToRedisArgs>(destination: K0, numkeys: i64, key: &[K1]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZDIFFSTORE");
        rv.arg(destination);
        rv.arg(numkeys);
        rv.arg(key);
        rv
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
    pub fn zincrby<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, increment: i64, member: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZINCRBY");
        rv.arg(key);
        rv.arg(increment);
        rv.arg(member);
        rv
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
    pub fn zinter<K0: ToRedisArgs>(numkeys: i64, key: &[K0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZINTER");
        rv.arg(numkeys);
        rv.arg(key);
        rv
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
    pub fn zintercard<K0: ToRedisArgs>(numkeys: i64, key: &[K0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZINTERCARD");
        rv.arg(numkeys);
        rv.arg(key);
        rv
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
    pub fn zinterstore<K0: ToRedisArgs, K1: ToRedisArgs>(destination: K0, numkeys: i64, key: &[K1]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZINTERSTORE");
        rv.arg(destination);
        rv.arg(numkeys);
        rv.arg(key);
        rv
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
    pub fn zlexcount<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(key: K0, min: T0, max: T1) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZLEXCOUNT");
        rv.arg(key);
        rv.arg(min);
        rv.arg(max);
        rv
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
    pub fn zmpop<K0: ToRedisArgs>(numkeys: i64, key: &[K0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZMPOP");
        rv.arg(numkeys);
        rv.arg(key);
        rv
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
    pub fn zmscore<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, member: &[T0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZMSCORE");
        rv.arg(key);
        rv.arg(member);
        rv
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
    pub fn zpopmax<K0: ToRedisArgs>(key: K0, count: Option<i64>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZPOPMAX");
        rv.arg(key);
        rv.arg(count);
        rv
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
    pub fn zpopmin<K0: ToRedisArgs>(key: K0, count: Option<i64>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZPOPMIN");
        rv.arg(key);
        rv.arg(count);
        rv
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
    pub fn zrandmember<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, options: Option<T0>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZRANDMEMBER");
        rv.arg(key);
        rv.arg(options);
        rv
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
    pub fn zrange<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(key: K0, min: T0, max: T1) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZRANGE");
        rv.arg(key);
        rv.arg(min);
        rv.arg(max);
        rv
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
    pub fn zrangebylex<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(key: K0, min: T0, max: T1) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZRANGEBYLEX");
        rv.arg(key);
        rv.arg(min);
        rv.arg(max);
        rv
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
    pub fn zrangebyscore<K0: ToRedisArgs>(key: K0, min: f64, max: f64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZRANGEBYSCORE");
        rv.arg(key);
        rv.arg(min);
        rv.arg(max);
        rv
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
    pub fn zrangestore<K0: ToRedisArgs, K1: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(dst: K0, src: K1, min: T0, max: T1) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZRANGESTORE");
        rv.arg(dst);
        rv.arg(src);
        rv.arg(min);
        rv.arg(max);
        rv
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
    pub fn zrank<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, member: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZRANK");
        rv.arg(key);
        rv.arg(member);
        rv
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
    pub fn zrem<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, member: &[T0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZREM");
        rv.arg(key);
        rv.arg(member);
        rv
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
    pub fn zremrangebylex<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(key: K0, min: T0, max: T1) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZREMRANGEBYLEX");
        rv.arg(key);
        rv.arg(min);
        rv.arg(max);
        rv
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
    pub fn zremrangebyrank<K0: ToRedisArgs>(key: K0, start: i64, stop: i64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZREMRANGEBYRANK");
        rv.arg(key);
        rv.arg(start);
        rv.arg(stop);
        rv
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
    pub fn zremrangebyscore<K0: ToRedisArgs>(key: K0, min: f64, max: f64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZREMRANGEBYSCORE");
        rv.arg(key);
        rv.arg(min);
        rv.arg(max);
        rv
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
    pub fn zrevrange<K0: ToRedisArgs>(key: K0, start: i64, stop: i64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZREVRANGE");
        rv.arg(key);
        rv.arg(start);
        rv.arg(stop);
        rv
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
    pub fn zrevrangebylex<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(key: K0, max: T0, min: T1) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZREVRANGEBYLEX");
        rv.arg(key);
        rv.arg(max);
        rv.arg(min);
        rv
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
    pub fn zrevrangebyscore<K0: ToRedisArgs>(key: K0, max: f64, min: f64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZREVRANGEBYSCORE");
        rv.arg(key);
        rv.arg(max);
        rv.arg(min);
        rv
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
    pub fn zrevrank<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, member: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZREVRANK");
        rv.arg(key);
        rv.arg(member);
        rv
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
    pub fn zscore<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, member: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZSCORE");
        rv.arg(key);
        rv.arg(member);
        rv
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
    pub fn zunion<K0: ToRedisArgs>(numkeys: i64, key: &[K0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZUNION");
        rv.arg(numkeys);
        rv.arg(key);
        rv
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
    pub fn zunionstore<K0: ToRedisArgs, K1: ToRedisArgs>(destination: K0, numkeys: i64, key: &[K1]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ZUNIONSTORE");
        rv.arg(destination);
        rv.arg(numkeys);
        rv.arg(key);
        rv
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
    pub fn hdel<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, field: &[T0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("HDEL");
        rv.arg(key);
        rv.arg(field);
        rv
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
    pub fn hexists<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, field: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("HEXISTS");
        rv.arg(key);
        rv.arg(field);
        rv
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
    pub fn hget<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, field: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("HGET");
        rv.arg(key);
        rv.arg(field);
        rv
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
    pub fn hgetall<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("HGETALL");
        rv.arg(key);
        rv
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
    pub fn hincrby<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, field: T0, increment: i64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("HINCRBY");
        rv.arg(key);
        rv.arg(field);
        rv.arg(increment);
        rv
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
    pub fn hincrbyfloat<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, field: T0, increment: f64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("HINCRBYFLOAT");
        rv.arg(key);
        rv.arg(field);
        rv.arg(increment);
        rv
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
    pub fn hkeys<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("HKEYS");
        rv.arg(key);
        rv
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
    pub fn hlen<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("HLEN");
        rv.arg(key);
        rv
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
    pub fn hmget<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, field: &[T0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("HMGET");
        rv.arg(key);
        rv.arg(field);
        rv
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
    pub fn hmset<K0: ToRedisArgs, T1: ToRedisArgs, T2: ToRedisArgs>(key: K0, field_value: &[(T1, T2)]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("HMSET");
        rv.arg(key);
        rv.arg(field_value);
        rv
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
    pub fn hrandfield<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, options: Option<T0>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("HRANDFIELD");
        rv.arg(key);
        rv.arg(options);
        rv
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
    pub fn hset<K0: ToRedisArgs, T1: ToRedisArgs, T2: ToRedisArgs>(key: K0, field_value: &[(T1, T2)]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("HSET");
        rv.arg(key);
        rv.arg(field_value);
        rv
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
    pub fn hsetnx<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(key: K0, field: T0, value: T1) -> Self {
        let mut rv = Cmd::new();
        rv.arg("HSETNX");
        rv.arg(key);
        rv.arg(field);
        rv.arg(value);
        rv
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
    pub fn hstrlen<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, field: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("HSTRLEN");
        rv.arg(key);
        rv.arg(field);
        rv
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
    pub fn hvals<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("HVALS");
        rv.arg(key);
        rv
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
    pub fn psubscribe<K0: ToRedisArgs>(pattern: &[K0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("PSUBSCRIBE");
        rv.arg(pattern);
        rv
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
    pub fn publish<T0: ToRedisArgs, T1: ToRedisArgs>(channel: T0, message: T1) -> Self {
        let mut rv = Cmd::new();
        rv.arg("PUBLISH");
        rv.arg(channel);
        rv.arg(message);
        rv
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
    pub fn pubsub() -> Self {
        let mut rv = Cmd::new();
        rv.arg("PUBSUB");
        rv
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
    pub fn pubsub_channels<K0: ToRedisArgs>(pattern: Option<K0>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("PUBSUB CHANNELS");
        rv.arg(pattern);
        rv
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
    pub fn pubsub_help() -> Self {
        let mut rv = Cmd::new();
        rv.arg("PUBSUB HELP");
        rv
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
    pub fn pubsub_numpat() -> Self {
        let mut rv = Cmd::new();
        rv.arg("PUBSUB NUMPAT");
        rv
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
    pub fn pubsub_numsub<T0: ToRedisArgs>(channel: Option<&[T0]>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("PUBSUB NUMSUB");
        rv.arg(channel);
        rv
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
    pub fn pubsub_shardchannels<K0: ToRedisArgs>(pattern: Option<K0>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("PUBSUB SHARDCHANNELS");
        rv.arg(pattern);
        rv
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
    pub fn pubsub_shardnumsub<T0: ToRedisArgs>(shardchannel: Option<&[T0]>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("PUBSUB SHARDNUMSUB");
        rv.arg(shardchannel);
        rv
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
    pub fn punsubscribe<K0: ToRedisArgs>(pattern: Option<&[K0]>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("PUNSUBSCRIBE");
        rv.arg(pattern);
        rv
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
    pub fn spublish<T0: ToRedisArgs, T1: ToRedisArgs>(shardchannel: T0, message: T1) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SPUBLISH");
        rv.arg(shardchannel);
        rv.arg(message);
        rv
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
    pub fn ssubscribe<T0: ToRedisArgs>(shardchannel: &[T0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SSUBSCRIBE");
        rv.arg(shardchannel);
        rv
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
    pub fn subscribe<T0: ToRedisArgs>(channel: &[T0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SUBSCRIBE");
        rv.arg(channel);
        rv
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
    pub fn sunsubscribe<T0: ToRedisArgs>(shardchannel: Option<&[T0]>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SUNSUBSCRIBE");
        rv.arg(shardchannel);
        rv
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
    pub fn unsubscribe<T0: ToRedisArgs>(channel: Option<&[T0]>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("UNSUBSCRIBE");
        rv.arg(channel);
        rv
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
    pub fn discard() -> Self {
        let mut rv = Cmd::new();
        rv.arg("DISCARD");
        rv
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
    pub fn exec() -> Self {
        let mut rv = Cmd::new();
        rv.arg("EXEC");
        rv
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
    pub fn multi() -> Self {
        let mut rv = Cmd::new();
        rv.arg("MULTI");
        rv
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
    pub fn unwatch() -> Self {
        let mut rv = Cmd::new();
        rv.arg("UNWATCH");
        rv
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
    pub fn watch<K0: ToRedisArgs>(key: &[K0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("WATCH");
        rv.arg(key);
        rv
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
    pub fn auth<T0: ToRedisArgs, T1: ToRedisArgs>(username: Option<T0>, password: T1) -> Self {
        let mut rv = Cmd::new();
        rv.arg("AUTH");
        rv.arg(username);
        rv.arg(password);
        rv
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
    pub fn client() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLIENT");
        rv
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
    pub fn client_caching() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLIENT CACHING");
        rv
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
    pub fn client_getname() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLIENT GETNAME");
        rv
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
    pub fn client_getredir() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLIENT GETREDIR");
        rv
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
    pub fn client_help() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLIENT HELP");
        rv
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
    pub fn client_id() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLIENT ID");
        rv
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
    pub fn client_info() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLIENT INFO");
        rv
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
    pub fn client_list() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLIENT LIST");
        rv
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
    pub fn client_no_evict() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLIENT NO-EVICT");
        rv
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
    pub fn client_pause(timeout: i64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLIENT PAUSE");
        rv.arg(timeout);
        rv
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
    pub fn client_reply() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLIENT REPLY");
        rv
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
    pub fn client_setname<T0: ToRedisArgs>(connection_name: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLIENT SETNAME");
        rv.arg(connection_name);
        rv
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
    pub fn client_tracking() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLIENT TRACKING");
        rv
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
    pub fn client_trackinginfo() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLIENT TRACKINGINFO");
        rv
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
    pub fn client_unblock(client_id: i64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLIENT UNBLOCK");
        rv.arg(client_id);
        rv
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
    pub fn client_unpause() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLIENT UNPAUSE");
        rv
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
    pub fn echo<T0: ToRedisArgs>(message: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ECHO");
        rv.arg(message);
        rv
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
    pub fn hello<T0: ToRedisArgs>(arguments: Option<T0>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("HELLO");
        rv.arg(arguments);
        rv
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
    pub fn ping<T0: ToRedisArgs>(message: Option<T0>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("PING");
        rv.arg(message);
        rv
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
    pub fn quit() -> Self {
        let mut rv = Cmd::new();
        rv.arg("QUIT");
        rv
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
    pub fn reset() -> Self {
        let mut rv = Cmd::new();
        rv.arg("RESET");
        rv
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
    pub fn select(index: i64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SELECT");
        rv.arg(index);
        rv
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
    pub fn acl() -> Self {
        let mut rv = Cmd::new();
        rv.arg("ACL");
        rv
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
    pub fn acl_cat<T0: ToRedisArgs>(categoryname: Option<T0>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ACL CAT");
        rv.arg(categoryname);
        rv
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
    pub fn acl_deluser<T0: ToRedisArgs>(username: &[T0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ACL DELUSER");
        rv.arg(username);
        rv
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
    pub fn acl_dryrun<T0: ToRedisArgs, T1: ToRedisArgs, T2: ToRedisArgs>(username: T0, command: T1, arg: Option<&[T2]>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ACL DRYRUN");
        rv.arg(username);
        rv.arg(command);
        rv.arg(arg);
        rv
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
    pub fn acl_genpass(bits: Option<i64>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ACL GENPASS");
        rv.arg(bits);
        rv
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
    pub fn acl_getuser<T0: ToRedisArgs>(username: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ACL GETUSER");
        rv.arg(username);
        rv
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
    pub fn acl_help() -> Self {
        let mut rv = Cmd::new();
        rv.arg("ACL HELP");
        rv
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
    pub fn acl_list() -> Self {
        let mut rv = Cmd::new();
        rv.arg("ACL LIST");
        rv
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
    pub fn acl_load() -> Self {
        let mut rv = Cmd::new();
        rv.arg("ACL LOAD");
        rv
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
    pub fn acl_log() -> Self {
        let mut rv = Cmd::new();
        rv.arg("ACL LOG");
        rv
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
    pub fn acl_save() -> Self {
        let mut rv = Cmd::new();
        rv.arg("ACL SAVE");
        rv
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
    pub fn acl_setuser<T0: ToRedisArgs, T1: ToRedisArgs>(username: T0, rule: Option<&[T1]>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("ACL SETUSER");
        rv.arg(username);
        rv.arg(rule);
        rv
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
    pub fn acl_users() -> Self {
        let mut rv = Cmd::new();
        rv.arg("ACL USERS");
        rv
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
    pub fn acl_whoami() -> Self {
        let mut rv = Cmd::new();
        rv.arg("ACL WHOAMI");
        rv
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
    pub fn bgrewriteaof() -> Self {
        let mut rv = Cmd::new();
        rv.arg("BGREWRITEAOF");
        rv
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
    pub fn bgsave() -> Self {
        let mut rv = Cmd::new();
        rv.arg("BGSAVE");
        rv
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
    pub fn command() -> Self {
        let mut rv = Cmd::new();
        rv.arg("COMMAND");
        rv
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
    pub fn command_count() -> Self {
        let mut rv = Cmd::new();
        rv.arg("COMMAND COUNT");
        rv
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
    pub fn command_docs<T0: ToRedisArgs>(command_name: Option<&[T0]>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("COMMAND DOCS");
        rv.arg(command_name);
        rv
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
    pub fn command_getkeys() -> Self {
        let mut rv = Cmd::new();
        rv.arg("COMMAND GETKEYS");
        rv
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
    pub fn command_getkeysandflags() -> Self {
        let mut rv = Cmd::new();
        rv.arg("COMMAND GETKEYSANDFLAGS");
        rv
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
    pub fn command_help() -> Self {
        let mut rv = Cmd::new();
        rv.arg("COMMAND HELP");
        rv
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
    pub fn command_info<T0: ToRedisArgs>(command_name: Option<&[T0]>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("COMMAND INFO");
        rv.arg(command_name);
        rv
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
    pub fn command_list() -> Self {
        let mut rv = Cmd::new();
        rv.arg("COMMAND LIST");
        rv
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
    pub fn config() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CONFIG");
        rv
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
    pub fn config_get<T1: ToRedisArgs>(parameter: &[T1]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("CONFIG GET");
        rv.arg(parameter);
        rv
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
    pub fn config_help() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CONFIG HELP");
        rv
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
    pub fn config_resetstat() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CONFIG RESETSTAT");
        rv
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
    pub fn config_rewrite() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CONFIG REWRITE");
        rv
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
    pub fn config_set<T1: ToRedisArgs, T2: ToRedisArgs>(parameter_value: &[(T1, T2)]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("CONFIG SET");
        rv.arg(parameter_value);
        rv
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
    pub fn dbsize() -> Self {
        let mut rv = Cmd::new();
        rv.arg("DBSIZE");
        rv
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
    pub fn debug() -> Self {
        let mut rv = Cmd::new();
        rv.arg("DEBUG");
        rv
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
    pub fn failover() -> Self {
        let mut rv = Cmd::new();
        rv.arg("FAILOVER");
        rv
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
    pub fn flushall() -> Self {
        let mut rv = Cmd::new();
        rv.arg("FLUSHALL");
        rv
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
    pub fn flushdb() -> Self {
        let mut rv = Cmd::new();
        rv.arg("FLUSHDB");
        rv
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
    pub fn info<T0: ToRedisArgs>(section: Option<&[T0]>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("INFO");
        rv.arg(section);
        rv
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
    pub fn lastsave() -> Self {
        let mut rv = Cmd::new();
        rv.arg("LASTSAVE");
        rv
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
    pub fn latency() -> Self {
        let mut rv = Cmd::new();
        rv.arg("LATENCY");
        rv
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
    pub fn latency_doctor() -> Self {
        let mut rv = Cmd::new();
        rv.arg("LATENCY DOCTOR");
        rv
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
    pub fn latency_graph<T0: ToRedisArgs>(event: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("LATENCY GRAPH");
        rv.arg(event);
        rv
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
    pub fn latency_help() -> Self {
        let mut rv = Cmd::new();
        rv.arg("LATENCY HELP");
        rv
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
    pub fn latency_histogram<T0: ToRedisArgs>(command: Option<&[T0]>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("LATENCY HISTOGRAM");
        rv.arg(command);
        rv
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
    pub fn latency_history<T0: ToRedisArgs>(event: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("LATENCY HISTORY");
        rv.arg(event);
        rv
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
    pub fn latency_latest() -> Self {
        let mut rv = Cmd::new();
        rv.arg("LATENCY LATEST");
        rv
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
    pub fn latency_reset<T0: ToRedisArgs>(event: Option<&[T0]>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("LATENCY RESET");
        rv.arg(event);
        rv
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
    pub fn lolwut() -> Self {
        let mut rv = Cmd::new();
        rv.arg("LOLWUT");
        rv
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
    pub fn memory() -> Self {
        let mut rv = Cmd::new();
        rv.arg("MEMORY");
        rv
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
    pub fn memory_doctor() -> Self {
        let mut rv = Cmd::new();
        rv.arg("MEMORY DOCTOR");
        rv
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
    pub fn memory_help() -> Self {
        let mut rv = Cmd::new();
        rv.arg("MEMORY HELP");
        rv
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
    pub fn memory_malloc_stats() -> Self {
        let mut rv = Cmd::new();
        rv.arg("MEMORY MALLOC-STATS");
        rv
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
    pub fn memory_purge() -> Self {
        let mut rv = Cmd::new();
        rv.arg("MEMORY PURGE");
        rv
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
    pub fn memory_stats() -> Self {
        let mut rv = Cmd::new();
        rv.arg("MEMORY STATS");
        rv
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
    pub fn memory_usage<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("MEMORY USAGE");
        rv.arg(key);
        rv
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
    pub fn module() -> Self {
        let mut rv = Cmd::new();
        rv.arg("MODULE");
        rv
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
    pub fn module_help() -> Self {
        let mut rv = Cmd::new();
        rv.arg("MODULE HELP");
        rv
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
    pub fn module_list() -> Self {
        let mut rv = Cmd::new();
        rv.arg("MODULE LIST");
        rv
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
    pub fn module_load<T0: ToRedisArgs, T1: ToRedisArgs>(path: T0, arg: Option<&[T1]>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("MODULE LOAD");
        rv.arg(path);
        rv.arg(arg);
        rv
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
    pub fn module_loadex<T0: ToRedisArgs>(path: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("MODULE LOADEX");
        rv.arg(path);
        rv
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
    pub fn module_unload<T0: ToRedisArgs>(name: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("MODULE UNLOAD");
        rv.arg(name);
        rv
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
    pub fn monitor() -> Self {
        let mut rv = Cmd::new();
        rv.arg("MONITOR");
        rv
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
    pub fn psync<T0: ToRedisArgs>(replicationid: T0, offset: i64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("PSYNC");
        rv.arg(replicationid);
        rv.arg(offset);
        rv
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
    pub fn replconf() -> Self {
        let mut rv = Cmd::new();
        rv.arg("REPLCONF");
        rv
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
    pub fn replicaof<T0: ToRedisArgs>(host: T0, port: i64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("REPLICAOF");
        rv.arg(host);
        rv.arg(port);
        rv
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
    pub fn restore_asking<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, ttl: i64, serialized_value: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("RESTORE-ASKING");
        rv.arg(key);
        rv.arg(ttl);
        rv.arg(serialized_value);
        rv
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
    pub fn role() -> Self {
        let mut rv = Cmd::new();
        rv.arg("ROLE");
        rv
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
    pub fn save() -> Self {
        let mut rv = Cmd::new();
        rv.arg("SAVE");
        rv
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
    pub fn shutdown() -> Self {
        let mut rv = Cmd::new();
        rv.arg("SHUTDOWN");
        rv
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
    pub fn slaveof<T0: ToRedisArgs>(host: T0, port: i64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SLAVEOF");
        rv.arg(host);
        rv.arg(port);
        rv
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
    pub fn slowlog() -> Self {
        let mut rv = Cmd::new();
        rv.arg("SLOWLOG");
        rv
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
    pub fn slowlog_get(count: Option<i64>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SLOWLOG GET");
        rv.arg(count);
        rv
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
    pub fn slowlog_help() -> Self {
        let mut rv = Cmd::new();
        rv.arg("SLOWLOG HELP");
        rv
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
    pub fn slowlog_len() -> Self {
        let mut rv = Cmd::new();
        rv.arg("SLOWLOG LEN");
        rv
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
    pub fn slowlog_reset() -> Self {
        let mut rv = Cmd::new();
        rv.arg("SLOWLOG RESET");
        rv
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
    pub fn swapdb(index1: i64, index2: i64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SWAPDB");
        rv.arg(index1);
        rv.arg(index2);
        rv
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
    pub fn sync() -> Self {
        let mut rv = Cmd::new();
        rv.arg("SYNC");
        rv
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
    pub fn time() -> Self {
        let mut rv = Cmd::new();
        rv.arg("TIME");
        rv
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
    pub fn eval<T0: ToRedisArgs, K0: ToRedisArgs, T1: ToRedisArgs>(script: T0, numkeys: i64, key: Option<&[K0]>, arg: Option<&[T1]>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("EVAL");
        rv.arg(script);
        rv.arg(numkeys);
        rv.arg(key);
        rv.arg(arg);
        rv
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
    pub fn evalsha<T0: ToRedisArgs, K0: ToRedisArgs, T1: ToRedisArgs>(sha1: T0, numkeys: i64, key: Option<&[K0]>, arg: Option<&[T1]>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("EVALSHA");
        rv.arg(sha1);
        rv.arg(numkeys);
        rv.arg(key);
        rv.arg(arg);
        rv
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
    pub fn evalsha_ro<T0: ToRedisArgs, K0: ToRedisArgs, T1: ToRedisArgs>(sha1: T0, numkeys: i64, key: &[K0], arg: &[T1]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("EVALSHA_RO");
        rv.arg(sha1);
        rv.arg(numkeys);
        rv.arg(key);
        rv.arg(arg);
        rv
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
    pub fn eval_ro<T0: ToRedisArgs, K0: ToRedisArgs, T1: ToRedisArgs>(script: T0, numkeys: i64, key: &[K0], arg: &[T1]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("EVAL_RO");
        rv.arg(script);
        rv.arg(numkeys);
        rv.arg(key);
        rv.arg(arg);
        rv
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
    pub fn fcall<T0: ToRedisArgs, K0: ToRedisArgs, T1: ToRedisArgs>(function: T0, numkeys: i64, key: &[K0], arg: &[T1]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("FCALL");
        rv.arg(function);
        rv.arg(numkeys);
        rv.arg(key);
        rv.arg(arg);
        rv
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
    pub fn fcall_ro<T0: ToRedisArgs, K0: ToRedisArgs, T1: ToRedisArgs>(function: T0, numkeys: i64, key: &[K0], arg: &[T1]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("FCALL_RO");
        rv.arg(function);
        rv.arg(numkeys);
        rv.arg(key);
        rv.arg(arg);
        rv
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
    pub fn function() -> Self {
        let mut rv = Cmd::new();
        rv.arg("FUNCTION");
        rv
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
    pub fn function_delete<T0: ToRedisArgs>(library_name: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("FUNCTION DELETE");
        rv.arg(library_name);
        rv
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
    pub fn function_dump() -> Self {
        let mut rv = Cmd::new();
        rv.arg("FUNCTION DUMP");
        rv
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
    pub fn function_flush() -> Self {
        let mut rv = Cmd::new();
        rv.arg("FUNCTION FLUSH");
        rv
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
    pub fn function_help() -> Self {
        let mut rv = Cmd::new();
        rv.arg("FUNCTION HELP");
        rv
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
    pub fn function_kill() -> Self {
        let mut rv = Cmd::new();
        rv.arg("FUNCTION KILL");
        rv
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
    pub fn function_list() -> Self {
        let mut rv = Cmd::new();
        rv.arg("FUNCTION LIST");
        rv
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
    pub fn function_load<T0: ToRedisArgs>(function_code: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("FUNCTION LOAD");
        rv.arg(function_code);
        rv
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
    pub fn function_restore<T0: ToRedisArgs>(serialized_value: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("FUNCTION RESTORE");
        rv.arg(serialized_value);
        rv
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
    pub fn function_stats() -> Self {
        let mut rv = Cmd::new();
        rv.arg("FUNCTION STATS");
        rv
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
    pub fn script() -> Self {
        let mut rv = Cmd::new();
        rv.arg("SCRIPT");
        rv
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
    pub fn script_debug() -> Self {
        let mut rv = Cmd::new();
        rv.arg("SCRIPT DEBUG");
        rv
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
    pub fn script_exists<T0: ToRedisArgs>(sha1: &[T0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SCRIPT EXISTS");
        rv.arg(sha1);
        rv
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
    pub fn script_flush() -> Self {
        let mut rv = Cmd::new();
        rv.arg("SCRIPT FLUSH");
        rv
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
    pub fn script_help() -> Self {
        let mut rv = Cmd::new();
        rv.arg("SCRIPT HELP");
        rv
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
    pub fn script_kill() -> Self {
        let mut rv = Cmd::new();
        rv.arg("SCRIPT KILL");
        rv
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
    pub fn script_load<T0: ToRedisArgs>(script: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SCRIPT LOAD");
        rv.arg(script);
        rv
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
    pub fn pfadd<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, element: Option<&[T0]>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("PFADD");
        rv.arg(key);
        rv.arg(element);
        rv
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
    pub fn pfcount<K0: ToRedisArgs>(key: &[K0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("PFCOUNT");
        rv.arg(key);
        rv
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
    pub fn pfdebug<T0: ToRedisArgs, K0: ToRedisArgs>(subcommand: T0, key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("PFDEBUG");
        rv.arg(subcommand);
        rv.arg(key);
        rv
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
    pub fn pfmerge<K0: ToRedisArgs, K1: ToRedisArgs>(destkey: K0, sourcekey: &[K1]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("PFMERGE");
        rv.arg(destkey);
        rv.arg(sourcekey);
        rv
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
    pub fn pfselftest() -> Self {
        let mut rv = Cmd::new();
        rv.arg("PFSELFTEST");
        rv
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
    pub fn asking() -> Self {
        let mut rv = Cmd::new();
        rv.arg("ASKING");
        rv
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
    pub fn cluster() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLUSTER");
        rv
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
    pub fn cluster_addslots(slot: &[i64]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLUSTER ADDSLOTS");
        rv.arg(slot);
        rv
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
    pub fn cluster_addslotsrange(start_slot_end_slot: &[(i64, i64)]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLUSTER ADDSLOTSRANGE");
        rv.arg(start_slot_end_slot);
        rv
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
    pub fn cluster_bumpepoch() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLUSTER BUMPEPOCH");
        rv
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
    pub fn cluster_count_failure_reports<T0: ToRedisArgs>(node_id: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLUSTER COUNT-FAILURE-REPORTS");
        rv.arg(node_id);
        rv
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
    pub fn cluster_countkeysinslot(slot: i64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLUSTER COUNTKEYSINSLOT");
        rv.arg(slot);
        rv
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
    pub fn cluster_delslots(slot: &[i64]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLUSTER DELSLOTS");
        rv.arg(slot);
        rv
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
    pub fn cluster_delslotsrange(start_slot_end_slot: &[(i64, i64)]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLUSTER DELSLOTSRANGE");
        rv.arg(start_slot_end_slot);
        rv
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
    pub fn cluster_failover() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLUSTER FAILOVER");
        rv
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
    pub fn cluster_flushslots() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLUSTER FLUSHSLOTS");
        rv
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
    pub fn cluster_forget<T0: ToRedisArgs>(node_id: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLUSTER FORGET");
        rv.arg(node_id);
        rv
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
    pub fn cluster_getkeysinslot(slot: i64, count: i64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLUSTER GETKEYSINSLOT");
        rv.arg(slot);
        rv.arg(count);
        rv
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
    pub fn cluster_help() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLUSTER HELP");
        rv
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
    pub fn cluster_info() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLUSTER INFO");
        rv
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
    pub fn cluster_keyslot<T0: ToRedisArgs>(key: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLUSTER KEYSLOT");
        rv.arg(key);
        rv
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
    pub fn cluster_links() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLUSTER LINKS");
        rv
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
    pub fn cluster_meet<T0: ToRedisArgs>(ip: T0, port: i64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLUSTER MEET");
        rv.arg(ip);
        rv.arg(port);
        rv
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
    pub fn cluster_myid() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLUSTER MYID");
        rv
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
    pub fn cluster_nodes() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLUSTER NODES");
        rv
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
    pub fn cluster_replicas<T0: ToRedisArgs>(node_id: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLUSTER REPLICAS");
        rv.arg(node_id);
        rv
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
    pub fn cluster_replicate<T0: ToRedisArgs>(node_id: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLUSTER REPLICATE");
        rv.arg(node_id);
        rv
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
    pub fn cluster_reset() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLUSTER RESET");
        rv
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
    pub fn cluster_saveconfig() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLUSTER SAVECONFIG");
        rv
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
    pub fn cluster_set_config_epoch(config_epoch: i64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLUSTER SET-CONFIG-EPOCH");
        rv.arg(config_epoch);
        rv
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
    pub fn cluster_setslot(slot: i64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLUSTER SETSLOT");
        rv.arg(slot);
        rv
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
    pub fn cluster_shards() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLUSTER SHARDS");
        rv
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
    pub fn cluster_slaves<T0: ToRedisArgs>(node_id: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLUSTER SLAVES");
        rv.arg(node_id);
        rv
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
    pub fn cluster_slots() -> Self {
        let mut rv = Cmd::new();
        rv.arg("CLUSTER SLOTS");
        rv
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
    pub fn readonly() -> Self {
        let mut rv = Cmd::new();
        rv.arg("READONLY");
        rv
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
    pub fn readwrite() -> Self {
        let mut rv = Cmd::new();
        rv.arg("READWRITE");
        rv
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
    pub fn geoadd<K0: ToRedisArgs, T1: ToRedisArgs>(key: K0, longitude_latitude_member: &[(f64, f64, T1)]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("GEOADD");
        rv.arg(key);
        rv.arg(longitude_latitude_member);
        rv
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
    pub fn geodist<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(key: K0, member1: T0, member2: T1) -> Self {
        let mut rv = Cmd::new();
        rv.arg("GEODIST");
        rv.arg(key);
        rv.arg(member1);
        rv.arg(member2);
        rv
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
    pub fn geohash<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, member: &[T0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("GEOHASH");
        rv.arg(key);
        rv.arg(member);
        rv
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
    pub fn geopos<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, member: &[T0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("GEOPOS");
        rv.arg(key);
        rv.arg(member);
        rv
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
    pub fn georadius<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, longitude: f64, latitude: f64, radius: f64, count: Option<T0>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("GEORADIUS");
        rv.arg(key);
        rv.arg(longitude);
        rv.arg(latitude);
        rv.arg(radius);
        rv.arg(count);
        rv
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
    pub fn georadiusbymember<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(key: K0, member: T0, radius: f64, count: Option<T1>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("GEORADIUSBYMEMBER");
        rv.arg(key);
        rv.arg(member);
        rv.arg(radius);
        rv.arg(count);
        rv
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
    pub fn georadiusbymember_ro<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(key: K0, member: T0, radius: f64, count: Option<T1>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("GEORADIUSBYMEMBER_RO");
        rv.arg(key);
        rv.arg(member);
        rv.arg(radius);
        rv.arg(count);
        rv
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
    pub fn georadius_ro<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, longitude: f64, latitude: f64, radius: f64, count: Option<T0>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("GEORADIUS_RO");
        rv.arg(key);
        rv.arg(longitude);
        rv.arg(latitude);
        rv.arg(radius);
        rv.arg(count);
        rv
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
    pub fn geosearch<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, count: Option<T0>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("GEOSEARCH");
        rv.arg(key);
        rv.arg(count);
        rv
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
    pub fn geosearchstore<K0: ToRedisArgs, K1: ToRedisArgs, T0: ToRedisArgs>(destination: K0, source: K1, count: Option<T0>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("GEOSEARCHSTORE");
        rv.arg(destination);
        rv.arg(source);
        rv.arg(count);
        rv
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
    pub fn xack<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(key: K0, group: T0, id: &[T1]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("XACK");
        rv.arg(key);
        rv.arg(group);
        rv.arg(id);
        rv
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
    pub fn xadd<K0: ToRedisArgs, T0: ToRedisArgs, T2: ToRedisArgs, T3: ToRedisArgs>(key: K0, trim: Option<T0>, field_value: &[(T2, T3)]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("XADD");
        rv.arg(key);
        rv.arg(trim);
        rv.arg(field_value);
        rv
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
    pub fn xautoclaim<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs, T2: ToRedisArgs, T3: ToRedisArgs>(key: K0, group: T0, consumer: T1, min_idle_time: T2, start: T3) -> Self {
        let mut rv = Cmd::new();
        rv.arg("XAUTOCLAIM");
        rv.arg(key);
        rv.arg(group);
        rv.arg(consumer);
        rv.arg(min_idle_time);
        rv.arg(start);
        rv
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
    pub fn xclaim<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs, T2: ToRedisArgs, T3: ToRedisArgs>(key: K0, group: T0, consumer: T1, min_idle_time: T2, id: &[T3]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("XCLAIM");
        rv.arg(key);
        rv.arg(group);
        rv.arg(consumer);
        rv.arg(min_idle_time);
        rv.arg(id);
        rv
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
    pub fn xdel<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, id: &[T0]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("XDEL");
        rv.arg(key);
        rv.arg(id);
        rv
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
    pub fn xgroup() -> Self {
        let mut rv = Cmd::new();
        rv.arg("XGROUP");
        rv
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
    pub fn xgroup_create<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, groupname: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("XGROUP CREATE");
        rv.arg(key);
        rv.arg(groupname);
        rv
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
    pub fn xgroup_createconsumer<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(key: K0, groupname: T0, consumername: T1) -> Self {
        let mut rv = Cmd::new();
        rv.arg("XGROUP CREATECONSUMER");
        rv.arg(key);
        rv.arg(groupname);
        rv.arg(consumername);
        rv
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
    pub fn xgroup_delconsumer<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(key: K0, groupname: T0, consumername: T1) -> Self {
        let mut rv = Cmd::new();
        rv.arg("XGROUP DELCONSUMER");
        rv.arg(key);
        rv.arg(groupname);
        rv.arg(consumername);
        rv
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
    pub fn xgroup_destroy<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, groupname: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("XGROUP DESTROY");
        rv.arg(key);
        rv.arg(groupname);
        rv
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
    pub fn xgroup_help() -> Self {
        let mut rv = Cmd::new();
        rv.arg("XGROUP HELP");
        rv
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
    pub fn xgroup_setid<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, groupname: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("XGROUP SETID");
        rv.arg(key);
        rv.arg(groupname);
        rv
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
    pub fn xinfo() -> Self {
        let mut rv = Cmd::new();
        rv.arg("XINFO");
        rv
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
    pub fn xinfo_consumers<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, groupname: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("XINFO CONSUMERS");
        rv.arg(key);
        rv.arg(groupname);
        rv
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
    pub fn xinfo_groups<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("XINFO GROUPS");
        rv.arg(key);
        rv
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
    pub fn xinfo_help() -> Self {
        let mut rv = Cmd::new();
        rv.arg("XINFO HELP");
        rv
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
    pub fn xinfo_stream<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("XINFO STREAM");
        rv.arg(key);
        rv
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
    pub fn xlen<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("XLEN");
        rv.arg(key);
        rv
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
    pub fn xpending<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(key: K0, group: T0, filters: Option<T1>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("XPENDING");
        rv.arg(key);
        rv.arg(group);
        rv.arg(filters);
        rv
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
    pub fn xrange<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(key: K0, start: T0, end: T1) -> Self {
        let mut rv = Cmd::new();
        rv.arg("XRANGE");
        rv.arg(key);
        rv.arg(start);
        rv.arg(end);
        rv
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
    pub fn xread() -> Self {
        let mut rv = Cmd::new();
        rv.arg("XREAD");
        rv
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
    pub fn xreadgroup() -> Self {
        let mut rv = Cmd::new();
        rv.arg("XREADGROUP");
        rv
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
    pub fn xrevrange<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(key: K0, end: T0, start: T1) -> Self {
        let mut rv = Cmd::new();
        rv.arg("XREVRANGE");
        rv.arg(key);
        rv.arg(end);
        rv.arg(start);
        rv
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
    pub fn xsetid<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, last_id: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("XSETID");
        rv.arg(key);
        rv.arg(last_id);
        rv
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
    pub fn xtrim<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, trim: T0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("XTRIM");
        rv.arg(key);
        rv.arg(trim);
        rv
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
    pub fn bitcount<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, index: Option<T0>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("BITCOUNT");
        rv.arg(key);
        rv.arg(index);
        rv
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
    pub fn bitfield<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("BITFIELD");
        rv.arg(key);
        rv
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
    pub fn bitfield_ro<K0: ToRedisArgs>(key: K0) -> Self {
        let mut rv = Cmd::new();
        rv.arg("BITFIELD_RO");
        rv.arg(key);
        rv
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
    pub fn bitop<T0: ToRedisArgs, K0: ToRedisArgs, K1: ToRedisArgs>(operation: T0, destkey: K0, key: &[K1]) -> Self {
        let mut rv = Cmd::new();
        rv.arg("BITOP");
        rv.arg(operation);
        rv.arg(destkey);
        rv.arg(key);
        rv
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
    pub fn bitpos<K0: ToRedisArgs, T0: ToRedisArgs>(key: K0, bit: i64, index: Option<T0>) -> Self {
        let mut rv = Cmd::new();
        rv.arg("BITPOS");
        rv.arg(key);
        rv.arg(bit);
        rv.arg(index);
        rv
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
    pub fn getbit<K0: ToRedisArgs>(key: K0, offset: i64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("GETBIT");
        rv.arg(key);
        rv.arg(offset);
        rv
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
    pub fn setbit<K0: ToRedisArgs>(key: K0, offset: i64, value: i64) -> Self {
        let mut rv = Cmd::new();
        rv.arg("SETBIT");
        rv.arg(key);
        rv.arg(offset);
        rv.arg(value);
        rv
    }

}
