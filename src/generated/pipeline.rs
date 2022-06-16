use crate::types::{FromRedisValue, NumericBehavior, RedisResult, ToRedisArgs, RedisWrite, Expiry};
use crate::connection::{Connection, ConnectionLike, Msg};
use crate::pipeline::Pipeline;
use crate::cmd::Cmd;

/// Implements common redis commands for pipelines.  Unlike the regular
/// commands trait, this returns the pipeline rather than a result
/// directly.  Other than that it works the same however.
impl Pipeline {
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
    pub fn copy<K0: ToRedisArgs, K1: ToRedisArgs>(&mut self, source: K0, destination: K1) -> &mut Self {
        self.add_command(Cmd::copy(source, destination))
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
    pub fn del<'a, K0: ToRedisArgs>(&mut self, key: &'a [K0]) -> &mut Self {
        self.add_command(Cmd::del(key))
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
    pub fn dump<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::dump(key))
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
    pub fn exists<'a, K0: ToRedisArgs>(&mut self, key: &'a [K0]) -> &mut Self {
        self.add_command(Cmd::exists(key))
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
    pub fn expire<K0: ToRedisArgs>(&mut self, key: K0, seconds: i64) -> &mut Self {
        self.add_command(Cmd::expire(key, seconds))
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
    pub fn expireat<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::expireat(key))
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
    pub fn expiretime<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::expiretime(key))
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
    pub fn keys<K0: ToRedisArgs>(&mut self, pattern: K0) -> &mut Self {
        self.add_command(Cmd::keys(pattern))
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
    pub fn migrate<T0: ToRedisArgs>(&mut self, host: T0, port: i64, destination_db: i64, timeout: i64) -> &mut Self {
        self.add_command(Cmd::migrate(host, port, destination_db, timeout))
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
    pub fn move_key<K0: ToRedisArgs>(&mut self, key: K0, db: i64) -> &mut Self {
        self.add_command(Cmd::move_key(key, db))
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
    pub fn object_encoding<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::object_encoding(key))
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
    pub fn object_freq<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::object_freq(key))
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
    pub fn object_help(&mut self) -> &mut Self {
        self.add_command(Cmd::object_help())
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
    pub fn object_idletime<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::object_idletime(key))
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
    pub fn object_refcount<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::object_refcount(key))
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
    pub fn persist<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::persist(key))
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
    pub fn pexpire<K0: ToRedisArgs>(&mut self, key: K0, milliseconds: i64) -> &mut Self {
        self.add_command(Cmd::pexpire(key, milliseconds))
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
    pub fn pexpireat<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::pexpireat(key))
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
    pub fn pexpiretime<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::pexpiretime(key))
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
    pub fn pttl<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::pttl(key))
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
    pub fn randomkey(&mut self) -> &mut Self {
        self.add_command(Cmd::randomkey())
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
    pub fn rename<K0: ToRedisArgs, K1: ToRedisArgs>(&mut self, key: K0, newkey: K1) -> &mut Self {
        self.add_command(Cmd::rename(key, newkey))
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
    pub fn renamenx<K0: ToRedisArgs, K1: ToRedisArgs>(&mut self, key: K0, newkey: K1) -> &mut Self {
        self.add_command(Cmd::renamenx(key, newkey))
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
    pub fn restore<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, ttl: i64, serialized_value: T0) -> &mut Self {
        self.add_command(Cmd::restore(key, ttl, serialized_value))
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
    pub fn sort<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::sort(key))
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
    pub fn sort_ro<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::sort_ro(key))
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
    pub fn touch<'a, K0: ToRedisArgs>(&mut self, key: &'a [K0]) -> &mut Self {
        self.add_command(Cmd::touch(key))
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
    pub fn ttl<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::ttl(key))
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
    pub fn r#type<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::r#type(key))
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
    pub fn unlink<'a, K0: ToRedisArgs>(&mut self, key: &'a [K0]) -> &mut Self {
        self.add_command(Cmd::unlink(key))
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
    pub fn wait(&mut self, numreplicas: i64, timeout: i64) -> &mut Self {
        self.add_command(Cmd::wait(numreplicas, timeout))
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
    pub fn append<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, value: T0) -> &mut Self {
        self.add_command(Cmd::append(key, value))
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
    pub fn decr<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::decr(key))
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
    pub fn decrby<K0: ToRedisArgs>(&mut self, key: K0, decrement: i64) -> &mut Self {
        self.add_command(Cmd::decrby(key, decrement))
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
    pub fn get<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::get(key))
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
    pub fn getdel<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::getdel(key))
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
    pub fn get_del<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::get_del(key))
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
    pub fn getex<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::getex(key))
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
    pub fn getrange<K0: ToRedisArgs>(&mut self, key: K0, start: i64, end: i64) -> &mut Self {
        self.add_command(Cmd::getrange(key, start, end))
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
    pub fn getset<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, value: T0) -> &mut Self {
        self.add_command(Cmd::getset(key, value))
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
    pub fn incr<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::incr(key))
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
    pub fn incrby<K0: ToRedisArgs>(&mut self, key: K0, increment: i64) -> &mut Self {
        self.add_command(Cmd::incrby(key, increment))
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
    pub fn incrbyfloat<K0: ToRedisArgs>(&mut self, key: K0, increment: f64) -> &mut Self {
        self.add_command(Cmd::incrbyfloat(key, increment))
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
    pub fn lcs<K0: ToRedisArgs, K1: ToRedisArgs>(&mut self, key1: K0, key2: K1) -> &mut Self {
        self.add_command(Cmd::lcs(key1, key2))
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
    pub fn mget<'a, K0: ToRedisArgs>(&mut self, key: &'a [K0]) -> &mut Self {
        self.add_command(Cmd::mget(key))
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
    pub fn mset<'a, T0: ToRedisArgs>(&mut self, key_value: &'a [T0]) -> &mut Self {
        self.add_command(Cmd::mset(key_value))
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
    pub fn msetnx<'a, T0: ToRedisArgs>(&mut self, key_value: &'a [T0]) -> &mut Self {
        self.add_command(Cmd::msetnx(key_value))
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
    pub fn psetex<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, milliseconds: i64, value: T0) -> &mut Self {
        self.add_command(Cmd::psetex(key, milliseconds, value))
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
    pub fn set<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, value: T0) -> &mut Self {
        self.add_command(Cmd::set(key, value))
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
    pub fn setex<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, seconds: i64, value: T0) -> &mut Self {
        self.add_command(Cmd::setex(key, seconds, value))
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
    pub fn setnx<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, value: T0) -> &mut Self {
        self.add_command(Cmd::setnx(key, value))
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
    pub fn setrange<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, offset: i64, value: T0) -> &mut Self {
        self.add_command(Cmd::setrange(key, offset, value))
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
    pub fn strlen<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::strlen(key))
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
    pub fn substr<K0: ToRedisArgs>(&mut self, key: K0, start: i64, end: i64) -> &mut Self {
        self.add_command(Cmd::substr(key, start, end))
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
    pub fn blmove<K0: ToRedisArgs, K1: ToRedisArgs>(&mut self, source: K0, destination: K1, timeout: f64) -> &mut Self {
        self.add_command(Cmd::blmove(source, destination, timeout))
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
    pub fn blmpop<'a, K0: ToRedisArgs>(&mut self, timeout: f64, numkeys: i64, key: &'a [K0]) -> &mut Self {
        self.add_command(Cmd::blmpop(timeout, numkeys, key))
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
    pub fn blpop<'a, K0: ToRedisArgs>(&mut self, key: &'a [K0], timeout: f64) -> &mut Self {
        self.add_command(Cmd::blpop(key, timeout))
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
    pub fn brpop<'a, K0: ToRedisArgs>(&mut self, key: &'a [K0], timeout: f64) -> &mut Self {
        self.add_command(Cmd::brpop(key, timeout))
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
    pub fn brpoplpush<K0: ToRedisArgs, K1: ToRedisArgs>(&mut self, source: K0, destination: K1, timeout: f64) -> &mut Self {
        self.add_command(Cmd::brpoplpush(source, destination, timeout))
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
    pub fn lindex<K0: ToRedisArgs>(&mut self, key: K0, index: i64) -> &mut Self {
        self.add_command(Cmd::lindex(key, index))
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
    pub fn linsert<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(&mut self, key: K0, pivot: T0, element: T1) -> &mut Self {
        self.add_command(Cmd::linsert(key, pivot, element))
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
    pub fn llen<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::llen(key))
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
    pub fn lmove<K0: ToRedisArgs, K1: ToRedisArgs>(&mut self, source: K0, destination: K1) -> &mut Self {
        self.add_command(Cmd::lmove(source, destination))
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
    pub fn lmpop<'a, K0: ToRedisArgs>(&mut self, numkeys: i64, key: &'a [K0]) -> &mut Self {
        self.add_command(Cmd::lmpop(numkeys, key))
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
    pub fn lpop<K0: ToRedisArgs>(&mut self, key: K0, count: Option<i64>) -> &mut Self {
        self.add_command(Cmd::lpop(key, count))
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
    pub fn lpos<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, element: T0) -> &mut Self {
        self.add_command(Cmd::lpos(key, element))
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
    pub fn lpush<'a, K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, element: &'a [T0]) -> &mut Self {
        self.add_command(Cmd::lpush(key, element))
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
    pub fn lpushx<'a, K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, element: &'a [T0]) -> &mut Self {
        self.add_command(Cmd::lpushx(key, element))
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
    pub fn lrange<K0: ToRedisArgs>(&mut self, key: K0, start: i64, stop: i64) -> &mut Self {
        self.add_command(Cmd::lrange(key, start, stop))
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
    pub fn lrem<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, count: i64, element: T0) -> &mut Self {
        self.add_command(Cmd::lrem(key, count, element))
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
    pub fn lset<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, index: i64, element: T0) -> &mut Self {
        self.add_command(Cmd::lset(key, index, element))
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
    pub fn ltrim<K0: ToRedisArgs>(&mut self, key: K0, start: i64, stop: i64) -> &mut Self {
        self.add_command(Cmd::ltrim(key, start, stop))
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
    pub fn rpop<K0: ToRedisArgs>(&mut self, key: K0, count: Option<i64>) -> &mut Self {
        self.add_command(Cmd::rpop(key, count))
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
    pub fn rpoplpush<K0: ToRedisArgs, K1: ToRedisArgs>(&mut self, source: K0, destination: K1) -> &mut Self {
        self.add_command(Cmd::rpoplpush(source, destination))
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
    pub fn rpush<'a, K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, element: &'a [T0]) -> &mut Self {
        self.add_command(Cmd::rpush(key, element))
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
    pub fn rpushx<'a, K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, element: &'a [T0]) -> &mut Self {
        self.add_command(Cmd::rpushx(key, element))
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
    pub fn sadd<'a, K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, member: &'a [T0]) -> &mut Self {
        self.add_command(Cmd::sadd(key, member))
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
    pub fn scard<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::scard(key))
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
    pub fn sdiff<'a, K0: ToRedisArgs>(&mut self, key: &'a [K0]) -> &mut Self {
        self.add_command(Cmd::sdiff(key))
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
    pub fn sdiffstore<'a, K0: ToRedisArgs, K1: ToRedisArgs>(&mut self, destination: K0, key: &'a [K1]) -> &mut Self {
        self.add_command(Cmd::sdiffstore(destination, key))
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
    pub fn sinter<'a, K0: ToRedisArgs>(&mut self, key: &'a [K0]) -> &mut Self {
        self.add_command(Cmd::sinter(key))
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
    pub fn sintercard<'a, K0: ToRedisArgs>(&mut self, numkeys: i64, key: &'a [K0]) -> &mut Self {
        self.add_command(Cmd::sintercard(numkeys, key))
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
    pub fn sinterstore<'a, K0: ToRedisArgs, K1: ToRedisArgs>(&mut self, destination: K0, key: &'a [K1]) -> &mut Self {
        self.add_command(Cmd::sinterstore(destination, key))
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
    pub fn sismember<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, member: T0) -> &mut Self {
        self.add_command(Cmd::sismember(key, member))
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
    pub fn smembers<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::smembers(key))
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
    pub fn smismember<'a, K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, member: &'a [T0]) -> &mut Self {
        self.add_command(Cmd::smismember(key, member))
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
    pub fn smove<K0: ToRedisArgs, K1: ToRedisArgs, T0: ToRedisArgs>(&mut self, source: K0, destination: K1, member: T0) -> &mut Self {
        self.add_command(Cmd::smove(source, destination, member))
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
    pub fn spop<K0: ToRedisArgs>(&mut self, key: K0, count: Option<i64>) -> &mut Self {
        self.add_command(Cmd::spop(key, count))
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
    pub fn srandmember<K0: ToRedisArgs>(&mut self, key: K0, count: Option<i64>) -> &mut Self {
        self.add_command(Cmd::srandmember(key, count))
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
    pub fn srem<'a, K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, member: &'a [T0]) -> &mut Self {
        self.add_command(Cmd::srem(key, member))
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
    pub fn sunion<'a, K0: ToRedisArgs>(&mut self, key: &'a [K0]) -> &mut Self {
        self.add_command(Cmd::sunion(key))
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
    pub fn sunionstore<'a, K0: ToRedisArgs, K1: ToRedisArgs>(&mut self, destination: K0, key: &'a [K1]) -> &mut Self {
        self.add_command(Cmd::sunionstore(destination, key))
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
    pub fn bzmpop<'a, K0: ToRedisArgs>(&mut self, timeout: f64, numkeys: i64, key: &'a [K0]) -> &mut Self {
        self.add_command(Cmd::bzmpop(timeout, numkeys, key))
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
    pub fn bzpopmax<'a, K0: ToRedisArgs>(&mut self, key: &'a [K0], timeout: f64) -> &mut Self {
        self.add_command(Cmd::bzpopmax(key, timeout))
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
    pub fn bzpopmin<'a, K0: ToRedisArgs>(&mut self, key: &'a [K0], timeout: f64) -> &mut Self {
        self.add_command(Cmd::bzpopmin(key, timeout))
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
    pub fn zadd<'a, K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, score_member: &'a [T0]) -> &mut Self {
        self.add_command(Cmd::zadd(key, score_member))
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
    pub fn zcard<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::zcard(key))
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
    pub fn zcount<K0: ToRedisArgs>(&mut self, key: K0, min: f64, max: f64) -> &mut Self {
        self.add_command(Cmd::zcount(key, min, max))
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
    pub fn zdiff<'a, K0: ToRedisArgs>(&mut self, numkeys: i64, key: &'a [K0]) -> &mut Self {
        self.add_command(Cmd::zdiff(numkeys, key))
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
    pub fn zdiffstore<'a, K0: ToRedisArgs, K1: ToRedisArgs>(&mut self, destination: K0, numkeys: i64, key: &'a [K1]) -> &mut Self {
        self.add_command(Cmd::zdiffstore(destination, numkeys, key))
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
    pub fn zincrby<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, increment: i64, member: T0) -> &mut Self {
        self.add_command(Cmd::zincrby(key, increment, member))
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
    pub fn zinter<'a, K0: ToRedisArgs>(&mut self, numkeys: i64, key: &'a [K0]) -> &mut Self {
        self.add_command(Cmd::zinter(numkeys, key))
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
    pub fn zintercard<'a, K0: ToRedisArgs>(&mut self, numkeys: i64, key: &'a [K0]) -> &mut Self {
        self.add_command(Cmd::zintercard(numkeys, key))
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
    pub fn zinterstore<'a, K0: ToRedisArgs, K1: ToRedisArgs>(&mut self, destination: K0, numkeys: i64, key: &'a [K1]) -> &mut Self {
        self.add_command(Cmd::zinterstore(destination, numkeys, key))
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
    pub fn zlexcount<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(&mut self, key: K0, min: T0, max: T1) -> &mut Self {
        self.add_command(Cmd::zlexcount(key, min, max))
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
    pub fn zmpop<'a, K0: ToRedisArgs>(&mut self, numkeys: i64, key: &'a [K0]) -> &mut Self {
        self.add_command(Cmd::zmpop(numkeys, key))
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
    pub fn zmscore<'a, K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, member: &'a [T0]) -> &mut Self {
        self.add_command(Cmd::zmscore(key, member))
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
    pub fn zpopmax<K0: ToRedisArgs>(&mut self, key: K0, count: Option<i64>) -> &mut Self {
        self.add_command(Cmd::zpopmax(key, count))
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
    pub fn zpopmin<K0: ToRedisArgs>(&mut self, key: K0, count: Option<i64>) -> &mut Self {
        self.add_command(Cmd::zpopmin(key, count))
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
    pub fn zrandmember<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, options: Option<T0>) -> &mut Self {
        self.add_command(Cmd::zrandmember(key, options))
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
    pub fn zrange<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(&mut self, key: K0, min: T0, max: T1) -> &mut Self {
        self.add_command(Cmd::zrange(key, min, max))
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
    pub fn zrangebylex<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(&mut self, key: K0, min: T0, max: T1) -> &mut Self {
        self.add_command(Cmd::zrangebylex(key, min, max))
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
    pub fn zrangebyscore<K0: ToRedisArgs>(&mut self, key: K0, min: f64, max: f64) -> &mut Self {
        self.add_command(Cmd::zrangebyscore(key, min, max))
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
    pub fn zrangestore<K0: ToRedisArgs, K1: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(&mut self, dst: K0, src: K1, min: T0, max: T1) -> &mut Self {
        self.add_command(Cmd::zrangestore(dst, src, min, max))
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
    pub fn zrank<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, member: T0) -> &mut Self {
        self.add_command(Cmd::zrank(key, member))
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
    pub fn zrem<'a, K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, member: &'a [T0]) -> &mut Self {
        self.add_command(Cmd::zrem(key, member))
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
    pub fn zremrangebylex<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(&mut self, key: K0, min: T0, max: T1) -> &mut Self {
        self.add_command(Cmd::zremrangebylex(key, min, max))
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
    pub fn zrembylex<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(&mut self, key: K0, min: T0, max: T1) -> &mut Self {
        self.add_command(Cmd::zrembylex(key, min, max))
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
    pub fn zremrangebyrank<K0: ToRedisArgs>(&mut self, key: K0, start: i64, stop: i64) -> &mut Self {
        self.add_command(Cmd::zremrangebyrank(key, start, stop))
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
    pub fn zremrangebyscore<K0: ToRedisArgs>(&mut self, key: K0, min: f64, max: f64) -> &mut Self {
        self.add_command(Cmd::zremrangebyscore(key, min, max))
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
    pub fn zrevrange<K0: ToRedisArgs>(&mut self, key: K0, start: i64, stop: i64) -> &mut Self {
        self.add_command(Cmd::zrevrange(key, start, stop))
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
    pub fn zrevrangebylex<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(&mut self, key: K0, max: T0, min: T1) -> &mut Self {
        self.add_command(Cmd::zrevrangebylex(key, max, min))
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
    pub fn zrevrangebyscore<K0: ToRedisArgs>(&mut self, key: K0, max: f64, min: f64) -> &mut Self {
        self.add_command(Cmd::zrevrangebyscore(key, max, min))
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
    pub fn zrevrank<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, member: T0) -> &mut Self {
        self.add_command(Cmd::zrevrank(key, member))
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
    pub fn zscore<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, member: T0) -> &mut Self {
        self.add_command(Cmd::zscore(key, member))
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
    pub fn zunion<'a, K0: ToRedisArgs>(&mut self, numkeys: i64, key: &'a [K0]) -> &mut Self {
        self.add_command(Cmd::zunion(numkeys, key))
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
    pub fn zunionstore<'a, K0: ToRedisArgs, K1: ToRedisArgs>(&mut self, destination: K0, numkeys: i64, key: &'a [K1]) -> &mut Self {
        self.add_command(Cmd::zunionstore(destination, numkeys, key))
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
    pub fn hdel<'a, K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, field: &'a [T0]) -> &mut Self {
        self.add_command(Cmd::hdel(key, field))
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
    pub fn hexists<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, field: T0) -> &mut Self {
        self.add_command(Cmd::hexists(key, field))
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
    pub fn hget<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, field: T0) -> &mut Self {
        self.add_command(Cmd::hget(key, field))
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
    pub fn hgetall<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::hgetall(key))
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
    pub fn hincrby<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, field: T0, increment: i64) -> &mut Self {
        self.add_command(Cmd::hincrby(key, field, increment))
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
    pub fn hincrbyfloat<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, field: T0, increment: f64) -> &mut Self {
        self.add_command(Cmd::hincrbyfloat(key, field, increment))
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
    pub fn hkeys<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::hkeys(key))
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
    pub fn hlen<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::hlen(key))
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
    pub fn hmget<'a, K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, field: &'a [T0]) -> &mut Self {
        self.add_command(Cmd::hmget(key, field))
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
    pub fn hmset<'a, K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, field_value: &'a [T0]) -> &mut Self {
        self.add_command(Cmd::hmset(key, field_value))
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
    pub fn hrandfield<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, options: Option<T0>) -> &mut Self {
        self.add_command(Cmd::hrandfield(key, options))
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
    pub fn hset<'a, K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, field_value: &'a [T0]) -> &mut Self {
        self.add_command(Cmd::hset(key, field_value))
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
    pub fn hsetnx<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(&mut self, key: K0, field: T0, value: T1) -> &mut Self {
        self.add_command(Cmd::hsetnx(key, field, value))
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
    pub fn hstrlen<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, field: T0) -> &mut Self {
        self.add_command(Cmd::hstrlen(key, field))
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
    pub fn hvals<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::hvals(key))
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
    pub fn psubscribe<'a, T0: ToRedisArgs>(&mut self, pattern: &'a [T0]) -> &mut Self {
        self.add_command(Cmd::psubscribe(pattern))
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
    pub fn publish<T0: ToRedisArgs, T1: ToRedisArgs>(&mut self, channel: T0, message: T1) -> &mut Self {
        self.add_command(Cmd::publish(channel, message))
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
    pub fn pubsub(&mut self) -> &mut Self {
        self.add_command(Cmd::pubsub())
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
    pub fn pubsub_channels<K0: ToRedisArgs>(&mut self, pattern: Option<K0>) -> &mut Self {
        self.add_command(Cmd::pubsub_channels(pattern))
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
    pub fn pubsub_help(&mut self) -> &mut Self {
        self.add_command(Cmd::pubsub_help())
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
    pub fn pubsub_numpat(&mut self) -> &mut Self {
        self.add_command(Cmd::pubsub_numpat())
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
    pub fn pubsub_numsub<'a, T0: ToRedisArgs>(&mut self, channel: Option<&'a [T0]>) -> &mut Self {
        self.add_command(Cmd::pubsub_numsub(channel))
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
    pub fn pubsub_shardchannels<K0: ToRedisArgs>(&mut self, pattern: Option<K0>) -> &mut Self {
        self.add_command(Cmd::pubsub_shardchannels(pattern))
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
    pub fn pubsub_shardnumsub<'a, T0: ToRedisArgs>(&mut self, shardchannel: Option<&'a [T0]>) -> &mut Self {
        self.add_command(Cmd::pubsub_shardnumsub(shardchannel))
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
    pub fn punsubscribe<'a, K0: ToRedisArgs>(&mut self, pattern: Option<&'a [K0]>) -> &mut Self {
        self.add_command(Cmd::punsubscribe(pattern))
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
    pub fn spublish<T0: ToRedisArgs, T1: ToRedisArgs>(&mut self, shardchannel: T0, message: T1) -> &mut Self {
        self.add_command(Cmd::spublish(shardchannel, message))
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
    pub fn ssubscribe<'a, T0: ToRedisArgs>(&mut self, shardchannel: &'a [T0]) -> &mut Self {
        self.add_command(Cmd::ssubscribe(shardchannel))
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
    pub fn subscribe<'a, T0: ToRedisArgs>(&mut self, channel: &'a [T0]) -> &mut Self {
        self.add_command(Cmd::subscribe(channel))
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
    pub fn sunsubscribe<'a, T0: ToRedisArgs>(&mut self, shardchannel: Option<&'a [T0]>) -> &mut Self {
        self.add_command(Cmd::sunsubscribe(shardchannel))
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
    pub fn unsubscribe<'a, T0: ToRedisArgs>(&mut self, channel: Option<&'a [T0]>) -> &mut Self {
        self.add_command(Cmd::unsubscribe(channel))
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
    pub fn discard(&mut self) -> &mut Self {
        self.add_command(Cmd::discard())
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
    pub fn exec(&mut self) -> &mut Self {
        self.add_command(Cmd::exec())
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
    pub fn multi(&mut self) -> &mut Self {
        self.add_command(Cmd::multi())
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
    pub fn unwatch(&mut self) -> &mut Self {
        self.add_command(Cmd::unwatch())
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
    pub fn watch<'a, K0: ToRedisArgs>(&mut self, key: &'a [K0]) -> &mut Self {
        self.add_command(Cmd::watch(key))
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
    pub fn auth<T0: ToRedisArgs, T1: ToRedisArgs>(&mut self, username: Option<T0>, password: T1) -> &mut Self {
        self.add_command(Cmd::auth(username, password))
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
    pub fn client(&mut self) -> &mut Self {
        self.add_command(Cmd::client())
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
    pub fn client_caching(&mut self) -> &mut Self {
        self.add_command(Cmd::client_caching())
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
    pub fn client_getname(&mut self) -> &mut Self {
        self.add_command(Cmd::client_getname())
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
    pub fn client_getredir(&mut self) -> &mut Self {
        self.add_command(Cmd::client_getredir())
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
    pub fn client_help(&mut self) -> &mut Self {
        self.add_command(Cmd::client_help())
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
    pub fn client_id(&mut self) -> &mut Self {
        self.add_command(Cmd::client_id())
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
    pub fn client_info(&mut self) -> &mut Self {
        self.add_command(Cmd::client_info())
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
    pub fn client_list(&mut self) -> &mut Self {
        self.add_command(Cmd::client_list())
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
    pub fn client_no_evict(&mut self) -> &mut Self {
        self.add_command(Cmd::client_no_evict())
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
    pub fn client_pause(&mut self, timeout: i64) -> &mut Self {
        self.add_command(Cmd::client_pause(timeout))
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
    pub fn client_reply(&mut self) -> &mut Self {
        self.add_command(Cmd::client_reply())
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
    pub fn client_setname<T0: ToRedisArgs>(&mut self, connection_name: T0) -> &mut Self {
        self.add_command(Cmd::client_setname(connection_name))
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
    pub fn client_tracking(&mut self) -> &mut Self {
        self.add_command(Cmd::client_tracking())
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
    pub fn client_trackinginfo(&mut self) -> &mut Self {
        self.add_command(Cmd::client_trackinginfo())
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
    pub fn client_unblock(&mut self, client_id: i64) -> &mut Self {
        self.add_command(Cmd::client_unblock(client_id))
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
    pub fn client_unpause(&mut self) -> &mut Self {
        self.add_command(Cmd::client_unpause())
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
    pub fn echo<T0: ToRedisArgs>(&mut self, message: T0) -> &mut Self {
        self.add_command(Cmd::echo(message))
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
    pub fn hello<T0: ToRedisArgs>(&mut self, arguments: Option<T0>) -> &mut Self {
        self.add_command(Cmd::hello(arguments))
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
    pub fn ping<T0: ToRedisArgs>(&mut self, message: Option<T0>) -> &mut Self {
        self.add_command(Cmd::ping(message))
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
    pub fn quit(&mut self) -> &mut Self {
        self.add_command(Cmd::quit())
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
    pub fn reset(&mut self) -> &mut Self {
        self.add_command(Cmd::reset())
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
    pub fn select(&mut self, index: i64) -> &mut Self {
        self.add_command(Cmd::select(index))
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
    pub fn acl(&mut self) -> &mut Self {
        self.add_command(Cmd::acl())
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
    pub fn acl_cat<T0: ToRedisArgs>(&mut self, categoryname: Option<T0>) -> &mut Self {
        self.add_command(Cmd::acl_cat(categoryname))
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
    pub fn acl_deluser<'a, T0: ToRedisArgs>(&mut self, username: &'a [T0]) -> &mut Self {
        self.add_command(Cmd::acl_deluser(username))
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
    pub fn acl_dryrun<'a, T0: ToRedisArgs, T1: ToRedisArgs, T2: ToRedisArgs>(&mut self, username: T0, command: T1, arg: Option<&'a [T2]>) -> &mut Self {
        self.add_command(Cmd::acl_dryrun(username, command, arg))
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
    pub fn acl_genpass(&mut self, bits: Option<i64>) -> &mut Self {
        self.add_command(Cmd::acl_genpass(bits))
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
    pub fn acl_getuser<T0: ToRedisArgs>(&mut self, username: T0) -> &mut Self {
        self.add_command(Cmd::acl_getuser(username))
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
    pub fn acl_help(&mut self) -> &mut Self {
        self.add_command(Cmd::acl_help())
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
    pub fn acl_list(&mut self) -> &mut Self {
        self.add_command(Cmd::acl_list())
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
    pub fn acl_load(&mut self) -> &mut Self {
        self.add_command(Cmd::acl_load())
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
    pub fn acl_log(&mut self) -> &mut Self {
        self.add_command(Cmd::acl_log())
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
    pub fn acl_save(&mut self) -> &mut Self {
        self.add_command(Cmd::acl_save())
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
    pub fn acl_setuser<'a, T0: ToRedisArgs, T1: ToRedisArgs>(&mut self, username: T0, rule: Option<&'a [T1]>) -> &mut Self {
        self.add_command(Cmd::acl_setuser(username, rule))
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
    pub fn acl_users(&mut self) -> &mut Self {
        self.add_command(Cmd::acl_users())
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
    pub fn acl_whoami(&mut self) -> &mut Self {
        self.add_command(Cmd::acl_whoami())
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
    pub fn bgrewriteaof(&mut self) -> &mut Self {
        self.add_command(Cmd::bgrewriteaof())
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
    pub fn bgsave(&mut self) -> &mut Self {
        self.add_command(Cmd::bgsave())
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
    pub fn command(&mut self) -> &mut Self {
        self.add_command(Cmd::command())
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
    pub fn command_count(&mut self) -> &mut Self {
        self.add_command(Cmd::command_count())
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
    pub fn command_docs<'a, T0: ToRedisArgs>(&mut self, command_name: Option<&'a [T0]>) -> &mut Self {
        self.add_command(Cmd::command_docs(command_name))
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
    pub fn command_getkeys(&mut self) -> &mut Self {
        self.add_command(Cmd::command_getkeys())
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
    pub fn command_getkeysandflags(&mut self) -> &mut Self {
        self.add_command(Cmd::command_getkeysandflags())
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
    pub fn command_help(&mut self) -> &mut Self {
        self.add_command(Cmd::command_help())
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
    pub fn command_info<'a, T0: ToRedisArgs>(&mut self, command_name: Option<&'a [T0]>) -> &mut Self {
        self.add_command(Cmd::command_info(command_name))
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
    pub fn command_list(&mut self) -> &mut Self {
        self.add_command(Cmd::command_list())
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
    pub fn config(&mut self) -> &mut Self {
        self.add_command(Cmd::config())
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
    pub fn config_get<'a, T0: ToRedisArgs>(&mut self, parameter: &'a [T0]) -> &mut Self {
        self.add_command(Cmd::config_get(parameter))
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
    pub fn config_help(&mut self) -> &mut Self {
        self.add_command(Cmd::config_help())
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
    pub fn config_resetstat(&mut self) -> &mut Self {
        self.add_command(Cmd::config_resetstat())
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
    pub fn config_rewrite(&mut self) -> &mut Self {
        self.add_command(Cmd::config_rewrite())
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
    pub fn config_set<'a, T0: ToRedisArgs>(&mut self, parameter_value: &'a [T0]) -> &mut Self {
        self.add_command(Cmd::config_set(parameter_value))
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
    pub fn dbsize(&mut self) -> &mut Self {
        self.add_command(Cmd::dbsize())
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
    pub fn debug(&mut self) -> &mut Self {
        self.add_command(Cmd::debug())
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
    pub fn failover(&mut self) -> &mut Self {
        self.add_command(Cmd::failover())
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
    pub fn flushall(&mut self) -> &mut Self {
        self.add_command(Cmd::flushall())
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
    pub fn flushdb(&mut self) -> &mut Self {
        self.add_command(Cmd::flushdb())
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
    pub fn info<'a, T0: ToRedisArgs>(&mut self, section: Option<&'a [T0]>) -> &mut Self {
        self.add_command(Cmd::info(section))
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
    pub fn lastsave(&mut self) -> &mut Self {
        self.add_command(Cmd::lastsave())
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
    pub fn latency(&mut self) -> &mut Self {
        self.add_command(Cmd::latency())
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
    pub fn latency_doctor(&mut self) -> &mut Self {
        self.add_command(Cmd::latency_doctor())
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
    pub fn latency_graph<T0: ToRedisArgs>(&mut self, event: T0) -> &mut Self {
        self.add_command(Cmd::latency_graph(event))
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
    pub fn latency_help(&mut self) -> &mut Self {
        self.add_command(Cmd::latency_help())
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
    pub fn latency_histogram<'a, T0: ToRedisArgs>(&mut self, command: Option<&'a [T0]>) -> &mut Self {
        self.add_command(Cmd::latency_histogram(command))
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
    pub fn latency_history<T0: ToRedisArgs>(&mut self, event: T0) -> &mut Self {
        self.add_command(Cmd::latency_history(event))
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
    pub fn latency_latest(&mut self) -> &mut Self {
        self.add_command(Cmd::latency_latest())
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
    pub fn latency_reset<'a, T0: ToRedisArgs>(&mut self, event: Option<&'a [T0]>) -> &mut Self {
        self.add_command(Cmd::latency_reset(event))
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
    pub fn lolwut(&mut self) -> &mut Self {
        self.add_command(Cmd::lolwut())
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
    pub fn memory(&mut self) -> &mut Self {
        self.add_command(Cmd::memory())
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
    pub fn memory_doctor(&mut self) -> &mut Self {
        self.add_command(Cmd::memory_doctor())
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
    pub fn memory_help(&mut self) -> &mut Self {
        self.add_command(Cmd::memory_help())
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
    pub fn memory_malloc_stats(&mut self) -> &mut Self {
        self.add_command(Cmd::memory_malloc_stats())
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
    pub fn memory_purge(&mut self) -> &mut Self {
        self.add_command(Cmd::memory_purge())
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
    pub fn memory_stats(&mut self) -> &mut Self {
        self.add_command(Cmd::memory_stats())
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
    pub fn memory_usage<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::memory_usage(key))
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
    pub fn module(&mut self) -> &mut Self {
        self.add_command(Cmd::module())
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
    pub fn module_help(&mut self) -> &mut Self {
        self.add_command(Cmd::module_help())
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
    pub fn module_list(&mut self) -> &mut Self {
        self.add_command(Cmd::module_list())
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
    pub fn module_load<'a, T0: ToRedisArgs, T1: ToRedisArgs>(&mut self, path: T0, arg: Option<&'a [T1]>) -> &mut Self {
        self.add_command(Cmd::module_load(path, arg))
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
    pub fn module_loadex<T0: ToRedisArgs>(&mut self, path: T0) -> &mut Self {
        self.add_command(Cmd::module_loadex(path))
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
    pub fn module_unload<T0: ToRedisArgs>(&mut self, name: T0) -> &mut Self {
        self.add_command(Cmd::module_unload(name))
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
    pub fn monitor(&mut self) -> &mut Self {
        self.add_command(Cmd::monitor())
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
    pub fn psync<T0: ToRedisArgs>(&mut self, replicationid: T0, offset: i64) -> &mut Self {
        self.add_command(Cmd::psync(replicationid, offset))
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
    pub fn replconf(&mut self) -> &mut Self {
        self.add_command(Cmd::replconf())
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
    pub fn replicaof<T0: ToRedisArgs>(&mut self, host: T0, port: i64) -> &mut Self {
        self.add_command(Cmd::replicaof(host, port))
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
    pub fn restore_asking<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, ttl: i64, serialized_value: T0) -> &mut Self {
        self.add_command(Cmd::restore_asking(key, ttl, serialized_value))
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
    pub fn role(&mut self) -> &mut Self {
        self.add_command(Cmd::role())
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
    pub fn save(&mut self) -> &mut Self {
        self.add_command(Cmd::save())
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
    pub fn shutdown(&mut self) -> &mut Self {
        self.add_command(Cmd::shutdown())
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
    pub fn slaveof<T0: ToRedisArgs>(&mut self, host: T0, port: i64) -> &mut Self {
        self.add_command(Cmd::slaveof(host, port))
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
    pub fn slowlog(&mut self) -> &mut Self {
        self.add_command(Cmd::slowlog())
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
    pub fn slowlog_get(&mut self, count: Option<i64>) -> &mut Self {
        self.add_command(Cmd::slowlog_get(count))
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
    pub fn slowlog_help(&mut self) -> &mut Self {
        self.add_command(Cmd::slowlog_help())
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
    pub fn slowlog_len(&mut self) -> &mut Self {
        self.add_command(Cmd::slowlog_len())
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
    pub fn slowlog_reset(&mut self) -> &mut Self {
        self.add_command(Cmd::slowlog_reset())
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
    pub fn swapdb(&mut self, index1: i64, index2: i64) -> &mut Self {
        self.add_command(Cmd::swapdb(index1, index2))
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
    pub fn sync(&mut self) -> &mut Self {
        self.add_command(Cmd::sync())
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
    pub fn time(&mut self) -> &mut Self {
        self.add_command(Cmd::time())
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
    pub fn eval<'a, T0: ToRedisArgs, K0: ToRedisArgs, T1: ToRedisArgs>(&mut self, script: T0, numkeys: i64, key: Option<&'a [K0]>, arg: Option<&'a [T1]>) -> &mut Self {
        self.add_command(Cmd::eval(script, numkeys, key, arg))
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
    pub fn evalsha<'a, T0: ToRedisArgs, K0: ToRedisArgs, T1: ToRedisArgs>(&mut self, sha1: T0, numkeys: i64, key: Option<&'a [K0]>, arg: Option<&'a [T1]>) -> &mut Self {
        self.add_command(Cmd::evalsha(sha1, numkeys, key, arg))
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
    pub fn evalsha_ro<'a, T0: ToRedisArgs, K0: ToRedisArgs, T1: ToRedisArgs>(&mut self, sha1: T0, numkeys: i64, key: &'a [K0], arg: &'a [T1]) -> &mut Self {
        self.add_command(Cmd::evalsha_ro(sha1, numkeys, key, arg))
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
    pub fn eval_ro<'a, T0: ToRedisArgs, K0: ToRedisArgs, T1: ToRedisArgs>(&mut self, script: T0, numkeys: i64, key: &'a [K0], arg: &'a [T1]) -> &mut Self {
        self.add_command(Cmd::eval_ro(script, numkeys, key, arg))
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
    pub fn fcall<'a, T0: ToRedisArgs, K0: ToRedisArgs, T1: ToRedisArgs>(&mut self, function: T0, numkeys: i64, key: &'a [K0], arg: &'a [T1]) -> &mut Self {
        self.add_command(Cmd::fcall(function, numkeys, key, arg))
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
    pub fn fcall_ro<'a, T0: ToRedisArgs, K0: ToRedisArgs, T1: ToRedisArgs>(&mut self, function: T0, numkeys: i64, key: &'a [K0], arg: &'a [T1]) -> &mut Self {
        self.add_command(Cmd::fcall_ro(function, numkeys, key, arg))
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
    pub fn function(&mut self) -> &mut Self {
        self.add_command(Cmd::function())
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
    pub fn function_delete<T0: ToRedisArgs>(&mut self, library_name: T0) -> &mut Self {
        self.add_command(Cmd::function_delete(library_name))
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
    pub fn function_dump(&mut self) -> &mut Self {
        self.add_command(Cmd::function_dump())
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
    pub fn function_flush(&mut self) -> &mut Self {
        self.add_command(Cmd::function_flush())
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
    pub fn function_help(&mut self) -> &mut Self {
        self.add_command(Cmd::function_help())
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
    pub fn function_kill(&mut self) -> &mut Self {
        self.add_command(Cmd::function_kill())
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
    pub fn function_list(&mut self) -> &mut Self {
        self.add_command(Cmd::function_list())
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
    pub fn function_load<T0: ToRedisArgs>(&mut self, function_code: T0) -> &mut Self {
        self.add_command(Cmd::function_load(function_code))
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
    pub fn function_restore<T0: ToRedisArgs>(&mut self, serialized_value: T0) -> &mut Self {
        self.add_command(Cmd::function_restore(serialized_value))
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
    pub fn function_stats(&mut self) -> &mut Self {
        self.add_command(Cmd::function_stats())
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
    pub fn script(&mut self) -> &mut Self {
        self.add_command(Cmd::script())
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
    pub fn script_debug(&mut self) -> &mut Self {
        self.add_command(Cmd::script_debug())
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
    pub fn script_exists<'a, T0: ToRedisArgs>(&mut self, sha1: &'a [T0]) -> &mut Self {
        self.add_command(Cmd::script_exists(sha1))
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
    pub fn script_flush(&mut self) -> &mut Self {
        self.add_command(Cmd::script_flush())
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
    pub fn script_help(&mut self) -> &mut Self {
        self.add_command(Cmd::script_help())
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
    pub fn script_kill(&mut self) -> &mut Self {
        self.add_command(Cmd::script_kill())
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
    pub fn script_load<T0: ToRedisArgs>(&mut self, script: T0) -> &mut Self {
        self.add_command(Cmd::script_load(script))
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
    pub fn pfadd<'a, K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, element: Option<&'a [T0]>) -> &mut Self {
        self.add_command(Cmd::pfadd(key, element))
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
    pub fn pfcount<'a, K0: ToRedisArgs>(&mut self, key: &'a [K0]) -> &mut Self {
        self.add_command(Cmd::pfcount(key))
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
    pub fn pfdebug<T0: ToRedisArgs, K0: ToRedisArgs>(&mut self, subcommand: T0, key: K0) -> &mut Self {
        self.add_command(Cmd::pfdebug(subcommand, key))
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
    pub fn pfmerge<'a, K0: ToRedisArgs, K1: ToRedisArgs>(&mut self, destkey: K0, sourcekey: &'a [K1]) -> &mut Self {
        self.add_command(Cmd::pfmerge(destkey, sourcekey))
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
    pub fn pfselftest(&mut self) -> &mut Self {
        self.add_command(Cmd::pfselftest())
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
    pub fn asking(&mut self) -> &mut Self {
        self.add_command(Cmd::asking())
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
    pub fn cluster(&mut self) -> &mut Self {
        self.add_command(Cmd::cluster())
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
    pub fn cluster_addslots<'a>(&mut self, slot: &'a [i64]) -> &mut Self {
        self.add_command(Cmd::cluster_addslots(slot))
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
    pub fn cluster_addslotsrange<'a, T0: ToRedisArgs>(&mut self, start_slot_end_slot: &'a [T0]) -> &mut Self {
        self.add_command(Cmd::cluster_addslotsrange(start_slot_end_slot))
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
    pub fn cluster_bumpepoch(&mut self) -> &mut Self {
        self.add_command(Cmd::cluster_bumpepoch())
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
    pub fn cluster_count_failure_reports<T0: ToRedisArgs>(&mut self, node_id: T0) -> &mut Self {
        self.add_command(Cmd::cluster_count_failure_reports(node_id))
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
    pub fn cluster_countkeysinslot(&mut self, slot: i64) -> &mut Self {
        self.add_command(Cmd::cluster_countkeysinslot(slot))
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
    pub fn cluster_delslots<'a>(&mut self, slot: &'a [i64]) -> &mut Self {
        self.add_command(Cmd::cluster_delslots(slot))
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
    pub fn cluster_delslotsrange<'a, T0: ToRedisArgs>(&mut self, start_slot_end_slot: &'a [T0]) -> &mut Self {
        self.add_command(Cmd::cluster_delslotsrange(start_slot_end_slot))
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
    pub fn cluster_failover(&mut self) -> &mut Self {
        self.add_command(Cmd::cluster_failover())
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
    pub fn cluster_flushslots(&mut self) -> &mut Self {
        self.add_command(Cmd::cluster_flushslots())
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
    pub fn cluster_forget<T0: ToRedisArgs>(&mut self, node_id: T0) -> &mut Self {
        self.add_command(Cmd::cluster_forget(node_id))
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
    pub fn cluster_getkeysinslot(&mut self, slot: i64, count: i64) -> &mut Self {
        self.add_command(Cmd::cluster_getkeysinslot(slot, count))
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
    pub fn cluster_help(&mut self) -> &mut Self {
        self.add_command(Cmd::cluster_help())
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
    pub fn cluster_info(&mut self) -> &mut Self {
        self.add_command(Cmd::cluster_info())
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
    pub fn cluster_keyslot<T0: ToRedisArgs>(&mut self, key: T0) -> &mut Self {
        self.add_command(Cmd::cluster_keyslot(key))
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
    pub fn cluster_links(&mut self) -> &mut Self {
        self.add_command(Cmd::cluster_links())
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
    pub fn cluster_meet<T0: ToRedisArgs>(&mut self, ip: T0, port: i64) -> &mut Self {
        self.add_command(Cmd::cluster_meet(ip, port))
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
    pub fn cluster_myid(&mut self) -> &mut Self {
        self.add_command(Cmd::cluster_myid())
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
    pub fn cluster_nodes(&mut self) -> &mut Self {
        self.add_command(Cmd::cluster_nodes())
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
    pub fn cluster_replicas<T0: ToRedisArgs>(&mut self, node_id: T0) -> &mut Self {
        self.add_command(Cmd::cluster_replicas(node_id))
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
    pub fn cluster_replicate<T0: ToRedisArgs>(&mut self, node_id: T0) -> &mut Self {
        self.add_command(Cmd::cluster_replicate(node_id))
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
    pub fn cluster_reset(&mut self) -> &mut Self {
        self.add_command(Cmd::cluster_reset())
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
    pub fn cluster_saveconfig(&mut self) -> &mut Self {
        self.add_command(Cmd::cluster_saveconfig())
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
    pub fn cluster_set_config_epoch(&mut self, config_epoch: i64) -> &mut Self {
        self.add_command(Cmd::cluster_set_config_epoch(config_epoch))
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
    pub fn cluster_setslot(&mut self, slot: i64) -> &mut Self {
        self.add_command(Cmd::cluster_setslot(slot))
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
    pub fn cluster_shards(&mut self) -> &mut Self {
        self.add_command(Cmd::cluster_shards())
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
    pub fn cluster_slaves<T0: ToRedisArgs>(&mut self, node_id: T0) -> &mut Self {
        self.add_command(Cmd::cluster_slaves(node_id))
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
    pub fn cluster_slots(&mut self) -> &mut Self {
        self.add_command(Cmd::cluster_slots())
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
    pub fn readonly(&mut self) -> &mut Self {
        self.add_command(Cmd::readonly())
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
    pub fn readwrite(&mut self) -> &mut Self {
        self.add_command(Cmd::readwrite())
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
    pub fn geoadd<'a, K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, longitude_latitude_member: &'a [T0]) -> &mut Self {
        self.add_command(Cmd::geoadd(key, longitude_latitude_member))
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
    pub fn geodist<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(&mut self, key: K0, member1: T0, member2: T1) -> &mut Self {
        self.add_command(Cmd::geodist(key, member1, member2))
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
    pub fn geohash<'a, K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, member: &'a [T0]) -> &mut Self {
        self.add_command(Cmd::geohash(key, member))
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
    pub fn geopos<'a, K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, member: &'a [T0]) -> &mut Self {
        self.add_command(Cmd::geopos(key, member))
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
    pub fn georadius<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, longitude: f64, latitude: f64, radius: f64, count: Option<T0>) -> &mut Self {
        self.add_command(Cmd::georadius(key, longitude, latitude, radius, count))
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
    pub fn georadiusbymember<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(&mut self, key: K0, member: T0, radius: f64, count: Option<T1>) -> &mut Self {
        self.add_command(Cmd::georadiusbymember(key, member, radius, count))
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
    pub fn georadiusbymember_ro<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(&mut self, key: K0, member: T0, radius: f64, count: Option<T1>) -> &mut Self {
        self.add_command(Cmd::georadiusbymember_ro(key, member, radius, count))
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
    pub fn georadius_ro<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, longitude: f64, latitude: f64, radius: f64, count: Option<T0>) -> &mut Self {
        self.add_command(Cmd::georadius_ro(key, longitude, latitude, radius, count))
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
    pub fn geosearch<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, count: Option<T0>) -> &mut Self {
        self.add_command(Cmd::geosearch(key, count))
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
    pub fn geosearchstore<K0: ToRedisArgs, K1: ToRedisArgs, T0: ToRedisArgs>(&mut self, destination: K0, source: K1, count: Option<T0>) -> &mut Self {
        self.add_command(Cmd::geosearchstore(destination, source, count))
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
    pub fn xack<'a, K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(&mut self, key: K0, group: T0, id: &'a [T1]) -> &mut Self {
        self.add_command(Cmd::xack(key, group, id))
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
    pub fn xadd<'a, K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(&mut self, key: K0, trim: Option<T0>, field_value: &'a [T1]) -> &mut Self {
        self.add_command(Cmd::xadd(key, trim, field_value))
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
    pub fn xautoclaim<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs, T2: ToRedisArgs, T3: ToRedisArgs>(&mut self, key: K0, group: T0, consumer: T1, min_idle_time: T2, start: T3) -> &mut Self {
        self.add_command(Cmd::xautoclaim(key, group, consumer, min_idle_time, start))
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
    pub fn xclaim<'a, K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs, T2: ToRedisArgs, T3: ToRedisArgs>(&mut self, key: K0, group: T0, consumer: T1, min_idle_time: T2, id: &'a [T3]) -> &mut Self {
        self.add_command(Cmd::xclaim(key, group, consumer, min_idle_time, id))
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
    pub fn xdel<'a, K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, id: &'a [T0]) -> &mut Self {
        self.add_command(Cmd::xdel(key, id))
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
    pub fn xgroup(&mut self) -> &mut Self {
        self.add_command(Cmd::xgroup())
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
    pub fn xgroup_create<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, groupname: T0) -> &mut Self {
        self.add_command(Cmd::xgroup_create(key, groupname))
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
    pub fn xgroup_createconsumer<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(&mut self, key: K0, groupname: T0, consumername: T1) -> &mut Self {
        self.add_command(Cmd::xgroup_createconsumer(key, groupname, consumername))
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
    pub fn xgroup_delconsumer<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(&mut self, key: K0, groupname: T0, consumername: T1) -> &mut Self {
        self.add_command(Cmd::xgroup_delconsumer(key, groupname, consumername))
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
    pub fn xgroup_destroy<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, groupname: T0) -> &mut Self {
        self.add_command(Cmd::xgroup_destroy(key, groupname))
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
    pub fn xgroup_help(&mut self) -> &mut Self {
        self.add_command(Cmd::xgroup_help())
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
    pub fn xgroup_setid<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, groupname: T0) -> &mut Self {
        self.add_command(Cmd::xgroup_setid(key, groupname))
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
    pub fn xinfo(&mut self) -> &mut Self {
        self.add_command(Cmd::xinfo())
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
    pub fn xinfo_consumers<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, groupname: T0) -> &mut Self {
        self.add_command(Cmd::xinfo_consumers(key, groupname))
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
    pub fn xinfo_groups<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::xinfo_groups(key))
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
    pub fn xinfo_help(&mut self) -> &mut Self {
        self.add_command(Cmd::xinfo_help())
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
    pub fn xinfo_stream<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::xinfo_stream(key))
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
    pub fn xlen<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::xlen(key))
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
    pub fn xpending<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(&mut self, key: K0, group: T0, filters: Option<T1>) -> &mut Self {
        self.add_command(Cmd::xpending(key, group, filters))
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
    pub fn xrange<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(&mut self, key: K0, start: T0, end: T1) -> &mut Self {
        self.add_command(Cmd::xrange(key, start, end))
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
    pub fn xread(&mut self) -> &mut Self {
        self.add_command(Cmd::xread())
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
    pub fn xreadgroup(&mut self) -> &mut Self {
        self.add_command(Cmd::xreadgroup())
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
    pub fn xrevrange<K0: ToRedisArgs, T0: ToRedisArgs, T1: ToRedisArgs>(&mut self, key: K0, end: T0, start: T1) -> &mut Self {
        self.add_command(Cmd::xrevrange(key, end, start))
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
    pub fn xsetid<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, last_id: T0) -> &mut Self {
        self.add_command(Cmd::xsetid(key, last_id))
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
    pub fn xtrim<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, trim: T0) -> &mut Self {
        self.add_command(Cmd::xtrim(key, trim))
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
    pub fn bitcount<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, index: Option<T0>) -> &mut Self {
        self.add_command(Cmd::bitcount(key, index))
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
    pub fn bitfield<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::bitfield(key))
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
    pub fn bitfield_ro<K0: ToRedisArgs>(&mut self, key: K0) -> &mut Self {
        self.add_command(Cmd::bitfield_ro(key))
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
    pub fn bitop<'a, T0: ToRedisArgs, K0: ToRedisArgs, K1: ToRedisArgs>(&mut self, operation: T0, destkey: K0, key: &'a [K1]) -> &mut Self {
        self.add_command(Cmd::bitop(operation, destkey, key))
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
    pub fn bitpos<K0: ToRedisArgs, T0: ToRedisArgs>(&mut self, key: K0, bit: i64, index: Option<T0>) -> &mut Self {
        self.add_command(Cmd::bitpos(key, bit, index))
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
    pub fn getbit<K0: ToRedisArgs>(&mut self, key: K0, offset: i64) -> &mut Self {
        self.add_command(Cmd::getbit(key, offset))
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
    pub fn setbit<K0: ToRedisArgs>(&mut self, key: K0, offset: i64, value: i64) -> &mut Self {
        self.add_command(Cmd::setbit(key, offset, value))
    }

}
