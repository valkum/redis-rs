#![cfg_attr(rustfmt, rustfmt_skip)]
//! These are enums and structs based on commands.json
//!
//! For each oneof attribute there is a enum based on the token or the attribute name.
//! For each block attribute there is a struct based on the token or the attribute name.
//! Also included are wrapper types for arguments that have token.
//! ```
use crate::types::{ToRedisArgs, RedisWrite};
/// Redis Type: Source COPY::Source
pub struct Source(String);

impl crate::types::ToRedisArgs for Source {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Destination COPY::Destination
pub struct Destination(String);

impl crate::types::ToRedisArgs for Destination {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: DB COPY::Db
pub struct Db(i64);

impl crate::types::ToRedisArgs for Db {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "DB".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Block: Replace
pub struct Replace {
}

impl crate::types::ToRedisArgs for Replace {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "REPLACE".write_redis_args(out);
    }
}
/// Redis Type: Key DEL::Key
pub struct Key(String);

impl crate::types::ToRedisArgs for Key {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Seconds EXPIRE::Seconds
pub struct Seconds(i64);

impl crate::types::ToRedisArgs for Seconds {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Condition
pub enum Condition {
    /// NX
    Nx,
    /// XX
    Xx,
    /// GT
    Gt,
    /// LT
    Lt,
}

impl crate::types::ToRedisArgs for Condition {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        match self {
            Condition::Nx => "NX".write_redis_args(out),
            Condition::Xx => "XX".write_redis_args(out),
            Condition::Gt => "GT".write_redis_args(out),
            Condition::Lt => "LT".write_redis_args(out),
        }
    }
}
/// Redis Type: Pattern KEYS::Pattern
pub struct Pattern(String);

impl crate::types::ToRedisArgs for Pattern {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Host MIGRATE::Host
pub struct Host(String);

impl crate::types::ToRedisArgs for Host {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Port MIGRATE::Port
pub struct Port(i64);

impl crate::types::ToRedisArgs for Port {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: KeyOrEmptyString
pub enum KeyOrEmptyString {
    /// Unknown
    Key(String),
    /// 
    EmptyString,
}

impl crate::types::ToRedisArgs for KeyOrEmptyString {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        match self {
            KeyOrEmptyString::Key(inner) => {
                inner.write_redis_args(out);
            },
            KeyOrEmptyString::EmptyString => "".write_redis_args(out),
        }
    }
}
/// Redis Type: DestinationDb MIGRATE::DestinationDb
pub struct DestinationDb(i64);

impl crate::types::ToRedisArgs for DestinationDb {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Timeout MIGRATE::Timeout
pub struct Timeout(i64);

impl crate::types::ToRedisArgs for Timeout {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Block: CopyArg
pub struct CopyArg {
}

impl crate::types::ToRedisArgs for CopyArg {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "COPY".write_redis_args(out);
    }
}
/// Redis Type: Authentication
pub enum Authentication {
    /// AUTH
    Auth(String),
    /// AUTH2
    Auth2 {username: String, password: String},
}

impl crate::types::ToRedisArgs for Authentication {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        match self {
            Authentication::Auth(inner) => {
                "AUTH".write_redis_args(out);
                inner.write_redis_args(out);
            },
            Authentication::Auth2{username, password} => {
                "AUTH2".write_redis_args(out);
                username.write_redis_args(out);
                password.write_redis_args(out);
            },
        }
    }
}
/// Redis Type: KEYS MIGRATE::Keys
pub struct Keys(String);

impl crate::types::ToRedisArgs for Keys {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "KEYS".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Milliseconds PEXPIRE::Milliseconds
pub struct Milliseconds(i64);

impl crate::types::ToRedisArgs for Milliseconds {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Newkey RENAME::Newkey
pub struct Newkey(String);

impl crate::types::ToRedisArgs for Newkey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Ttl RESTORE::Ttl
pub struct Ttl(i64);

impl crate::types::ToRedisArgs for Ttl {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: SerializedValue RESTORE::SerializedValue
pub struct SerializedValue(String);

impl crate::types::ToRedisArgs for SerializedValue {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Block: Absttl
pub struct Absttl {
}

impl crate::types::ToRedisArgs for Absttl {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "ABSTTL".write_redis_args(out);
    }
}
/// Redis Type: IDLETIME RESTORE::Idletime
pub struct Idletime(i64);

impl crate::types::ToRedisArgs for Idletime {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "IDLETIME".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: FREQ RESTORE::Freq
pub struct Freq(i64);

impl crate::types::ToRedisArgs for Freq {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "FREQ".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Cursor SCAN::Cursor
pub struct Cursor(i64);

impl crate::types::ToRedisArgs for Cursor {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: MATCH SCAN::Match
pub struct Match(String);

impl crate::types::ToRedisArgs for Match {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "MATCH".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: COUNT SCAN::Count
pub struct Count(i64);

impl crate::types::ToRedisArgs for Count {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "COUNT".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: TYPE SCAN::Type
pub struct Type(String);

impl crate::types::ToRedisArgs for Type {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "TYPE".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: BY SORT::By
pub struct By(String);

impl crate::types::ToRedisArgs for By {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "BY".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Block: Limit
pub struct Limit {
    /// offset
    pub offset: i64,
    /// count
    pub count: i64,
}

impl crate::types::ToRedisArgs for Limit {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "LIMIT".write_redis_args(out);
        self.offset.write_redis_args(out);
        self.count.write_redis_args(out);
    }
}
/// Redis Type: GET SORT::Get
pub struct Get(String);

impl crate::types::ToRedisArgs for Get {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "GET".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Order
pub enum Order {
    /// ASC
    Asc,
    /// DESC
    Desc,
}

impl crate::types::ToRedisArgs for Order {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        match self {
            Order::Asc => "ASC".write_redis_args(out),
            Order::Desc => "DESC".write_redis_args(out),
        }
    }
}
/// Redis Block: Alpha
pub struct Alpha {
}

impl crate::types::ToRedisArgs for Alpha {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "ALPHA".write_redis_args(out);
    }
}
/// Redis Type: STORE SORT::Store
pub struct Store(String);

impl crate::types::ToRedisArgs for Store {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "STORE".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Numreplicas WAIT::Numreplicas
pub struct Numreplicas(i64);

impl crate::types::ToRedisArgs for Numreplicas {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Value APPEND::Value
pub struct Value(String);

impl crate::types::ToRedisArgs for Value {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Decrement DECRBY::Decrement
pub struct Decrement(i64);

impl crate::types::ToRedisArgs for Decrement {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Expiration
pub enum Expiration {
    /// EX
    Ex(i64),
    /// PX
    Px(i64),
    /// PERSIST
    Persist,
}

impl crate::types::ToRedisArgs for Expiration {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        match self {
            Expiration::Ex(inner) => {
                "EX".write_redis_args(out);
                inner.write_redis_args(out);
            },
            Expiration::Px(inner) => {
                "PX".write_redis_args(out);
                inner.write_redis_args(out);
            },
            Expiration::Persist => "PERSIST".write_redis_args(out),
        }
    }
}
/// Redis Type: Start GETRANGE::Start
pub struct Start(i64);

impl crate::types::ToRedisArgs for Start {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: End GETRANGE::End
pub struct End(i64);

impl crate::types::ToRedisArgs for End {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Increment INCRBY::Increment
pub struct Increment(i64);

impl crate::types::ToRedisArgs for Increment {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Key1 LCS::Key1
pub struct Key1(String);

impl crate::types::ToRedisArgs for Key1 {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Key2 LCS::Key2
pub struct Key2(String);

impl crate::types::ToRedisArgs for Key2 {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Block: Len
pub struct Len {
}

impl crate::types::ToRedisArgs for Len {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "LEN".write_redis_args(out);
    }
}
/// Redis Block: Idx
pub struct Idx {
}

impl crate::types::ToRedisArgs for Idx {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "IDX".write_redis_args(out);
    }
}
/// Redis Type: MINMATCHLEN LCS::Minmatchlen
pub struct Minmatchlen(i64);

impl crate::types::ToRedisArgs for Minmatchlen {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "MINMATCHLEN".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Block: Withmatchlen
pub struct Withmatchlen {
}

impl crate::types::ToRedisArgs for Withmatchlen {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "WITHMATCHLEN".write_redis_args(out);
    }
}
/// Redis Block: KeyValue
pub struct KeyValue {
    /// key
    pub key: String,
    /// value
    pub value: String,
}

impl crate::types::ToRedisArgs for KeyValue {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.key.write_redis_args(out);
        self.value.write_redis_args(out);
    }
}
/// Redis Type: Offset SETRANGE::Offset
pub struct Offset(i64);

impl crate::types::ToRedisArgs for Offset {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Wherefrom
pub enum Wherefrom {
    /// LEFT
    Left,
    /// RIGHT
    Right,
}

impl crate::types::ToRedisArgs for Wherefrom {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        match self {
            Wherefrom::Left => "LEFT".write_redis_args(out),
            Wherefrom::Right => "RIGHT".write_redis_args(out),
        }
    }
}
/// Redis Type: Whereto
pub enum Whereto {
    /// LEFT
    Left,
    /// RIGHT
    Right,
}

impl crate::types::ToRedisArgs for Whereto {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        match self {
            Whereto::Left => "LEFT".write_redis_args(out),
            Whereto::Right => "RIGHT".write_redis_args(out),
        }
    }
}
/// Redis Type: Numkeys BLMPOP::Numkeys
pub struct Numkeys(i64);

impl crate::types::ToRedisArgs for Numkeys {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Where
pub enum Where {
    /// LEFT
    Left,
    /// RIGHT
    Right,
}

impl crate::types::ToRedisArgs for Where {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        match self {
            Where::Left => "LEFT".write_redis_args(out),
            Where::Right => "RIGHT".write_redis_args(out),
        }
    }
}
/// Redis Type: Index LINDEX::Index
pub struct Index(i64);

impl crate::types::ToRedisArgs for Index {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Pivot LINSERT::Pivot
pub struct Pivot(String);

impl crate::types::ToRedisArgs for Pivot {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Element LINSERT::Element
pub struct Element(String);

impl crate::types::ToRedisArgs for Element {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: RANK LPOS::Rank
pub struct Rank(i64);

impl crate::types::ToRedisArgs for Rank {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "RANK".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: MAXLEN LPOS::Maxlen
pub struct Maxlen(i64);

impl crate::types::ToRedisArgs for Maxlen {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "MAXLEN".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Stop LRANGE::Stop
pub struct Stop(i64);

impl crate::types::ToRedisArgs for Stop {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Member SADD::Member
pub struct Member(String);

impl crate::types::ToRedisArgs for Member {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Comparison
pub enum Comparison {
    /// GT
    Gt,
    /// LT
    Lt,
}

impl crate::types::ToRedisArgs for Comparison {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        match self {
            Comparison::Gt => "GT".write_redis_args(out),
            Comparison::Lt => "LT".write_redis_args(out),
        }
    }
}
/// Redis Block: Ch
pub struct Ch {
}

impl crate::types::ToRedisArgs for Ch {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "CH".write_redis_args(out);
    }
}
/// Redis Block: Incr
pub struct Incr {
}

impl crate::types::ToRedisArgs for Incr {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "INCR".write_redis_args(out);
    }
}
/// Redis Block: ScoreMember
pub struct ScoreMember {
    /// score
    pub score: f64,
    /// member
    pub member: String,
}

impl crate::types::ToRedisArgs for ScoreMember {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.score.write_redis_args(out);
        self.member.write_redis_args(out);
    }
}
/// Redis Type: Min ZCOUNT::Min
pub struct Min(f64);

impl crate::types::ToRedisArgs for Min {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Max ZCOUNT::Max
pub struct Max(f64);

impl crate::types::ToRedisArgs for Max {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Block: Withscores
pub struct Withscores {
}

impl crate::types::ToRedisArgs for Withscores {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "WITHSCORES".write_redis_args(out);
    }
}
/// Redis Type: WEIGHTS ZINTER::Weights
pub struct Weights(i64);

impl crate::types::ToRedisArgs for Weights {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "WEIGHTS".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Aggregate
pub enum Aggregate {
    /// SUM
    Sum,
    /// MIN
    Min,
    /// MAX
    Max,
}

impl crate::types::ToRedisArgs for Aggregate {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "AGGREGATE".write_redis_args(out);
        match self {
            Aggregate::Sum => "SUM".write_redis_args(out),
            Aggregate::Min => "MIN".write_redis_args(out),
            Aggregate::Max => "MAX".write_redis_args(out),
        }
    }
}
/// Redis Block: Options
pub struct Options {
    /// count
    pub count: i64,
    /// withscores
    pub withscores: bool,
}

impl crate::types::ToRedisArgs for Options {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.count.write_redis_args(out);
        if self.withscores {
            "WITHSCORES".write_redis_args(out);
        }
    }
}
/// Redis Type: Sortby
pub enum Sortby {
    /// BYSCORE
    Byscore,
    /// BYLEX
    Bylex,
}

impl crate::types::ToRedisArgs for Sortby {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        match self {
            Sortby::Byscore => "BYSCORE".write_redis_args(out),
            Sortby::Bylex => "BYLEX".write_redis_args(out),
        }
    }
}
/// Redis Block: Rev
pub struct Rev {
}

impl crate::types::ToRedisArgs for Rev {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "REV".write_redis_args(out);
    }
}
/// Redis Type: Dst ZRANGESTORE::Dst
pub struct Dst(String);

impl crate::types::ToRedisArgs for Dst {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Src ZRANGESTORE::Src
pub struct Src(String);

impl crate::types::ToRedisArgs for Src {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Field HDEL::Field
pub struct Field(String);

impl crate::types::ToRedisArgs for Field {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Block: FieldValue
pub struct FieldValue {
    /// field
    pub field: String,
    /// value
    pub value: String,
}

impl crate::types::ToRedisArgs for FieldValue {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.field.write_redis_args(out);
        self.value.write_redis_args(out);
    }
}
/// Redis Type: Channel PUBLISH::Channel
pub struct Channel(String);

impl crate::types::ToRedisArgs for Channel {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Message PUBLISH::Message
pub struct Message(String);

impl crate::types::ToRedisArgs for Message {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Shardchannel PUBSUB SHARDNUMSUB::Shardchannel
pub struct Shardchannel(String);

impl crate::types::ToRedisArgs for Shardchannel {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Username AUTH::Username
pub struct Username(String);

impl crate::types::ToRedisArgs for Username {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Password AUTH::Password
pub struct Password(String);

impl crate::types::ToRedisArgs for Password {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Mode
pub enum Mode {
    /// YES
    Yes,
    /// NO
    No,
}

impl crate::types::ToRedisArgs for Mode {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        match self {
            Mode::Yes => "YES".write_redis_args(out),
            Mode::No => "NO".write_redis_args(out),
        }
    }
}
/// Redis Type: IpPort CLIENT KILL::IpPort
pub struct IpPort(String);

impl crate::types::ToRedisArgs for IpPort {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: ID CLIENT KILL::Id
pub struct Id(i64);

impl crate::types::ToRedisArgs for Id {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "ID".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: USER CLIENT KILL::User
pub struct User(String);

impl crate::types::ToRedisArgs for User {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "USER".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: ADDR CLIENT KILL::Addr
pub struct Addr(String);

impl crate::types::ToRedisArgs for Addr {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "ADDR".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: LADDR CLIENT KILL::Laddr
pub struct Laddr(String);

impl crate::types::ToRedisArgs for Laddr {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "LADDR".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: SKIPME CLIENT KILL::Skipme
pub struct Skipme(String);

impl crate::types::ToRedisArgs for Skipme {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "SKIPME".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Enabled
pub enum Enabled {
    /// ON
    On,
    /// OFF
    Off,
}

impl crate::types::ToRedisArgs for Enabled {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        match self {
            Enabled::On => "ON".write_redis_args(out),
            Enabled::Off => "OFF".write_redis_args(out),
        }
    }
}
/// Redis Type: OnOffSkip
pub enum OnOffSkip {
    /// ON
    On,
    /// OFF
    Off,
    /// SKIP
    Skip,
}

impl crate::types::ToRedisArgs for OnOffSkip {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        match self {
            OnOffSkip::On => "ON".write_redis_args(out),
            OnOffSkip::Off => "OFF".write_redis_args(out),
            OnOffSkip::Skip => "SKIP".write_redis_args(out),
        }
    }
}
/// Redis Type: ConnectionName CLIENT SETNAME::ConnectionName
pub struct ConnectionName(String);

impl crate::types::ToRedisArgs for ConnectionName {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Status
pub enum Status {
    /// ON
    On,
    /// OFF
    Off,
}

impl crate::types::ToRedisArgs for Status {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        match self {
            Status::On => "ON".write_redis_args(out),
            Status::Off => "OFF".write_redis_args(out),
        }
    }
}
/// Redis Type: REDIRECT CLIENT TRACKING::Redirect
pub struct Redirect(i64);

impl crate::types::ToRedisArgs for Redirect {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "REDIRECT".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: PREFIX CLIENT TRACKING::Prefix
pub struct Prefix(String);

impl crate::types::ToRedisArgs for Prefix {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "PREFIX".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Block: Bcast
pub struct Bcast {
}

impl crate::types::ToRedisArgs for Bcast {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "BCAST".write_redis_args(out);
    }
}
/// Redis Block: Optin
pub struct Optin {
}

impl crate::types::ToRedisArgs for Optin {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "OPTIN".write_redis_args(out);
    }
}
/// Redis Block: Optout
pub struct Optout {
}

impl crate::types::ToRedisArgs for Optout {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "OPTOUT".write_redis_args(out);
    }
}
/// Redis Block: Noloop
pub struct Noloop {
}

impl crate::types::ToRedisArgs for Noloop {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "NOLOOP".write_redis_args(out);
    }
}
/// Redis Type: ClientId CLIENT UNBLOCK::ClientId
pub struct ClientId(i64);

impl crate::types::ToRedisArgs for ClientId {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: TimeoutError
pub enum TimeoutError {
    /// TIMEOUT
    Timeout,
    /// ERROR
    Error,
}

impl crate::types::ToRedisArgs for TimeoutError {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        match self {
            TimeoutError::Timeout => "TIMEOUT".write_redis_args(out),
            TimeoutError::Error => "ERROR".write_redis_args(out),
        }
    }
}
/// Redis Block: Arguments
pub struct Arguments {
    /// protover
    pub protover: i64,
    /// username_password
    pub username_password: crate::generated::types::Auth,
    /// clientname
    pub clientname: crate::generated::types::Setname,
}

impl crate::types::ToRedisArgs for Arguments {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.protover.write_redis_args(out);
        self.username_password.write_redis_args(out);
        self.clientname.write_redis_args(out);
    }
}
/// Redis Type: SETNAME HELLO::arguments::Setname
pub struct Setname(String);

impl crate::types::ToRedisArgs for Setname {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "SETNAME".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Block: Auth
pub struct Auth {
    /// username
    pub username: String,
    /// password
    pub password: String,
}

impl crate::types::ToRedisArgs for Auth {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "AUTH".write_redis_args(out);
        self.username.write_redis_args(out);
        self.password.write_redis_args(out);
    }
}
/// Redis Type: Categoryname ACL CAT::Categoryname
pub struct Categoryname(String);

impl crate::types::ToRedisArgs for Categoryname {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Command ACL DRYRUN::Command
pub struct Command(String);

impl crate::types::ToRedisArgs for Command {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Arg ACL DRYRUN::Arg
pub struct Arg(String);

impl crate::types::ToRedisArgs for Arg {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Bits ACL GENPASS::Bits
pub struct Bits(i64);

impl crate::types::ToRedisArgs for Bits {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Operation
pub enum Operation {
    /// Unknown
    Count(i64),
    /// RESET
    Reset,
}

impl crate::types::ToRedisArgs for Operation {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        match self {
            Operation::Count(inner) => {
                inner.write_redis_args(out);
            },
            Operation::Reset => "RESET".write_redis_args(out),
        }
    }
}
/// Redis Type: Rule ACL SETUSER::Rule
pub struct Rule(String);

impl crate::types::ToRedisArgs for Rule {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Block: Schedule
pub struct Schedule {
}

impl crate::types::ToRedisArgs for Schedule {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "SCHEDULE".write_redis_args(out);
    }
}
/// Redis Type: CommandName COMMAND DOCS::CommandName
pub struct CommandName(String);

impl crate::types::ToRedisArgs for CommandName {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Filterby
pub enum Filterby {
    /// MODULE
    Module(String),
    /// ACLCAT
    Aclcat(String),
    /// PATTERN
    Pattern(String),
}

impl crate::types::ToRedisArgs for Filterby {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "FILTERBY".write_redis_args(out);
        match self {
            Filterby::Module(inner) => {
                "MODULE".write_redis_args(out);
                inner.write_redis_args(out);
            },
            Filterby::Aclcat(inner) => {
                "ACLCAT".write_redis_args(out);
                inner.write_redis_args(out);
            },
            Filterby::Pattern(inner) => {
                "PATTERN".write_redis_args(out);
                inner.write_redis_args(out);
            },
        }
    }
}
/// Redis Block: Parameter
pub struct Parameter {
    /// parameter
    pub parameter: String,
}

impl crate::types::ToRedisArgs for Parameter {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.parameter.write_redis_args(out);
    }
}
/// Redis Block: ParameterValue
pub struct ParameterValue {
    /// parameter
    pub parameter: String,
    /// value
    pub value: String,
}

impl crate::types::ToRedisArgs for ParameterValue {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.parameter.write_redis_args(out);
        self.value.write_redis_args(out);
    }
}
/// Redis Block: To
pub struct To {
    /// host
    pub host: String,
    /// port
    pub port: i64,
    /// force
    pub force: bool,
}

impl crate::types::ToRedisArgs for To {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "TO".write_redis_args(out);
        self.host.write_redis_args(out);
        self.port.write_redis_args(out);
        if self.force {
            "FORCE".write_redis_args(out);
        }
    }
}
/// Redis Block: Abort
pub struct Abort {
}

impl crate::types::ToRedisArgs for Abort {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "ABORT".write_redis_args(out);
    }
}
/// Redis Type: Async
pub enum Async {
    /// ASYNC
    Async,
    /// SYNC
    Sync,
}

impl crate::types::ToRedisArgs for Async {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        match self {
            Async::Async => "ASYNC".write_redis_args(out),
            Async::Sync => "SYNC".write_redis_args(out),
        }
    }
}
/// Redis Type: Section INFO::Section
pub struct Section(String);

impl crate::types::ToRedisArgs for Section {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Event LATENCY GRAPH::Event
pub struct Event(String);

impl crate::types::ToRedisArgs for Event {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: VERSION LOLWUT::Version
pub struct Version(i64);

impl crate::types::ToRedisArgs for Version {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "VERSION".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: SAMPLES MEMORY USAGE::Samples
pub struct Samples(i64);

impl crate::types::ToRedisArgs for Samples {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "SAMPLES".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Path MODULE LOAD::Path
pub struct Path(String);

impl crate::types::ToRedisArgs for Path {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Block: Config
pub struct Config {
    /// name
    pub name: String,
    /// value
    pub value: String,
}

impl crate::types::ToRedisArgs for Config {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "CONFIG".write_redis_args(out);
        self.name.write_redis_args(out);
        self.value.write_redis_args(out);
    }
}
/// Redis Block: Args
pub struct Args {
    /// arg
    pub arg: String,
}

impl crate::types::ToRedisArgs for Args {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "ARGS".write_redis_args(out);
        self.arg.write_redis_args(out);
    }
}
/// Redis Type: Name MODULE UNLOAD::Name
pub struct Name(String);

impl crate::types::ToRedisArgs for Name {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Replicationid PSYNC::Replicationid
pub struct Replicationid(String);

impl crate::types::ToRedisArgs for Replicationid {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: NosaveSave
pub enum NosaveSave {
    /// NOSAVE
    Nosave,
    /// SAVE
    Save,
}

impl crate::types::ToRedisArgs for NosaveSave {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        match self {
            NosaveSave::Nosave => "NOSAVE".write_redis_args(out),
            NosaveSave::Save => "SAVE".write_redis_args(out),
        }
    }
}
/// Redis Block: Now
pub struct Now {
}

impl crate::types::ToRedisArgs for Now {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "NOW".write_redis_args(out);
    }
}
/// Redis Block: Force
pub struct Force {
}

impl crate::types::ToRedisArgs for Force {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "FORCE".write_redis_args(out);
    }
}
/// Redis Type: Index1 SWAPDB::Index1
pub struct Index1(i64);

impl crate::types::ToRedisArgs for Index1 {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Index2 SWAPDB::Index2
pub struct Index2(i64);

impl crate::types::ToRedisArgs for Index2 {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Script EVAL::Script
pub struct Script(String);

impl crate::types::ToRedisArgs for Script {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Sha1 EVALSHA::Sha1
pub struct Sha1(String);

impl crate::types::ToRedisArgs for Sha1 {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Function FCALL::Function
pub struct Function(String);

impl crate::types::ToRedisArgs for Function {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: LibraryName FUNCTION DELETE::LibraryName
pub struct LibraryName(String);

impl crate::types::ToRedisArgs for LibraryName {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: LIBRARYNAME FUNCTION LIST::Libraryname
pub struct Libraryname(String);

impl crate::types::ToRedisArgs for Libraryname {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "LIBRARYNAME".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Block: Withcode
pub struct Withcode {
}

impl crate::types::ToRedisArgs for Withcode {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "WITHCODE".write_redis_args(out);
    }
}
/// Redis Type: FunctionCode FUNCTION LOAD::FunctionCode
pub struct FunctionCode(String);

impl crate::types::ToRedisArgs for FunctionCode {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Policy
pub enum Policy {
    /// FLUSH
    Flush,
    /// APPEND
    Append,
    /// REPLACE
    Replace,
}

impl crate::types::ToRedisArgs for Policy {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        match self {
            Policy::Flush => "FLUSH".write_redis_args(out),
            Policy::Append => "APPEND".write_redis_args(out),
            Policy::Replace => "REPLACE".write_redis_args(out),
        }
    }
}
/// Redis Type: Subcommand PFDEBUG::Subcommand
pub struct Subcommand(String);

impl crate::types::ToRedisArgs for Subcommand {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Destkey PFMERGE::Destkey
pub struct Destkey(String);

impl crate::types::ToRedisArgs for Destkey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Sourcekey PFMERGE::Sourcekey
pub struct Sourcekey(String);

impl crate::types::ToRedisArgs for Sourcekey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Slot CLUSTER ADDSLOTS::Slot
pub struct Slot(i64);

impl crate::types::ToRedisArgs for Slot {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Block: StartSlotEndSlot
pub struct StartSlotEndSlot {
    /// start_slot
    pub start_slot: i64,
    /// end_slot
    pub end_slot: i64,
}

impl crate::types::ToRedisArgs for StartSlotEndSlot {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.start_slot.write_redis_args(out);
        self.end_slot.write_redis_args(out);
    }
}
/// Redis Type: NodeId CLUSTER COUNT-FAILURE-REPORTS::NodeId
pub struct NodeId(String);

impl crate::types::ToRedisArgs for NodeId {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Ip CLUSTER MEET::Ip
pub struct Ip(String);

impl crate::types::ToRedisArgs for Ip {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: ClusterBusPort CLUSTER MEET::ClusterBusPort
pub struct ClusterBusPort(i64);

impl crate::types::ToRedisArgs for ClusterBusPort {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: HardSoft
pub enum HardSoft {
    /// HARD
    Hard,
    /// SOFT
    Soft,
}

impl crate::types::ToRedisArgs for HardSoft {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        match self {
            HardSoft::Hard => "HARD".write_redis_args(out),
            HardSoft::Soft => "SOFT".write_redis_args(out),
        }
    }
}
/// Redis Type: ConfigEpoch CLUSTER SET-CONFIG-EPOCH::ConfigEpoch
pub struct ConfigEpoch(i64);

impl crate::types::ToRedisArgs for ConfigEpoch {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Block: LongitudeLatitudeMember
pub struct LongitudeLatitudeMember {
    /// longitude
    pub longitude: f64,
    /// latitude
    pub latitude: f64,
    /// member
    pub member: String,
}

impl crate::types::ToRedisArgs for LongitudeLatitudeMember {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.longitude.write_redis_args(out);
        self.latitude.write_redis_args(out);
        self.member.write_redis_args(out);
    }
}
/// Redis Type: Member1 GEODIST::Member1
pub struct Member1(String);

impl crate::types::ToRedisArgs for Member1 {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Member2 GEODIST::Member2
pub struct Member2(String);

impl crate::types::ToRedisArgs for Member2 {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Unit
pub enum Unit {
    /// M
    M,
    /// KM
    Km,
    /// FT
    Ft,
    /// MI
    Mi,
}

impl crate::types::ToRedisArgs for Unit {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        match self {
            Unit::M => "M".write_redis_args(out),
            Unit::Km => "KM".write_redis_args(out),
            Unit::Ft => "FT".write_redis_args(out),
            Unit::Mi => "MI".write_redis_args(out),
        }
    }
}
/// Redis Type: Longitude GEORADIUS::Longitude
pub struct Longitude(f64);

impl crate::types::ToRedisArgs for Longitude {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Latitude GEORADIUS::Latitude
pub struct Latitude(f64);

impl crate::types::ToRedisArgs for Latitude {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Radius GEORADIUS::Radius
pub struct Radius(f64);

impl crate::types::ToRedisArgs for Radius {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Block: Withcoord
pub struct Withcoord {
}

impl crate::types::ToRedisArgs for Withcoord {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "WITHCOORD".write_redis_args(out);
    }
}
/// Redis Block: Withdist
pub struct Withdist {
}

impl crate::types::ToRedisArgs for Withdist {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "WITHDIST".write_redis_args(out);
    }
}
/// Redis Block: Withhash
pub struct Withhash {
}

impl crate::types::ToRedisArgs for Withhash {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "WITHHASH".write_redis_args(out);
    }
}
/// Redis Type: STOREDIST GEORADIUS::Storedist
pub struct Storedist(String);

impl crate::types::ToRedisArgs for Storedist {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "STOREDIST".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: From
pub enum From {
    /// FROMMEMBER
    Frommember(String),
    /// FROMLONLAT
    Fromlonlat {longitude: f64, latitude: f64},
}

impl crate::types::ToRedisArgs for From {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        match self {
            From::Frommember(inner) => {
                "FROMMEMBER".write_redis_args(out);
                inner.write_redis_args(out);
            },
            From::Fromlonlat{longitude, latitude} => {
                "FROMLONLAT".write_redis_args(out);
                longitude.write_redis_args(out);
                latitude.write_redis_args(out);
            },
        }
    }
}
/// Redis Type: Group XACK::Group
pub struct Group(String);

impl crate::types::ToRedisArgs for Group {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Block: Nomkstream
pub struct Nomkstream {
}

impl crate::types::ToRedisArgs for Nomkstream {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "NOMKSTREAM".write_redis_args(out);
    }
}
/// Redis Block: Trim
pub struct Trim {
    /// strategy
    pub strategy: crate::generated::types::Strategy,
    /// operator
    pub operator: crate::generated::types::Operator,
    /// threshold
    pub threshold: String,
    /// count
    pub count: crate::generated::types::sintercard::Limit,
}

impl crate::types::ToRedisArgs for Trim {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.strategy.write_redis_args(out);
        self.operator.write_redis_args(out);
        self.threshold.write_redis_args(out);
        self.count.write_redis_args(out);
    }
}
/// Redis Type: Operator
pub enum Operator {
    /// =
    Equals,
    /// ~
    Approx,
}

impl crate::types::ToRedisArgs for Operator {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        match self {
            Operator::Equals => "=".write_redis_args(out),
            Operator::Approx => "~".write_redis_args(out),
        }
    }
}
/// Redis Type: Strategy
pub enum Strategy {
    /// MAXLEN
    Maxlen,
    /// MINID
    Minid,
}

impl crate::types::ToRedisArgs for Strategy {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        match self {
            Strategy::Maxlen => "MAXLEN".write_redis_args(out),
            Strategy::Minid => "MINID".write_redis_args(out),
        }
    }
}
/// Redis Type: IdOrAuto
pub enum IdOrAuto {
    /// *
    Star,
    /// Unknown
    Id(String),
}

impl crate::types::ToRedisArgs for IdOrAuto {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        match self {
            IdOrAuto::Star => "*".write_redis_args(out),
            IdOrAuto::Id(inner) => {
                inner.write_redis_args(out);
            },
        }
    }
}
/// Redis Type: Consumer XAUTOCLAIM::Consumer
pub struct Consumer(String);

impl crate::types::ToRedisArgs for Consumer {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: MinIdleTime XAUTOCLAIM::MinIdleTime
pub struct MinIdleTime(String);

impl crate::types::ToRedisArgs for MinIdleTime {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Block: Justid
pub struct Justid {
}

impl crate::types::ToRedisArgs for Justid {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "JUSTID".write_redis_args(out);
    }
}
/// Redis Type: IDLE XCLAIM::Idle
pub struct Idle(i64);

impl crate::types::ToRedisArgs for Idle {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "IDLE".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: RETRYCOUNT XCLAIM::Retrycount
pub struct Retrycount(i64);

impl crate::types::ToRedisArgs for Retrycount {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "RETRYCOUNT".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Groupname XGROUP CREATE::Groupname
pub struct Groupname(String);

impl crate::types::ToRedisArgs for Groupname {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Block: Mkstream
pub struct Mkstream {
}

impl crate::types::ToRedisArgs for Mkstream {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "MKSTREAM".write_redis_args(out);
    }
}
/// Redis Type: ENTRIESREAD XGROUP CREATE::Entriesread
pub struct Entriesread(i64);

impl crate::types::ToRedisArgs for Entriesread {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "ENTRIESREAD".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Consumername XGROUP CREATECONSUMER::Consumername
pub struct Consumername(String);

impl crate::types::ToRedisArgs for Consumername {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Block: Full
pub struct Full {
    /// count
    pub count: crate::generated::types::Count,
}

impl crate::types::ToRedisArgs for Full {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "FULL".write_redis_args(out);
        self.count.write_redis_args(out);
    }
}
/// Redis Block: Filters
pub struct Filters {
    /// min_idle_time
    pub min_idle_time: crate::generated::types::Idle,
    /// start
    pub start: String,
    /// end
    pub end: String,
    /// count
    pub count: i64,
    /// consumer
    pub consumer: String,
}

impl crate::types::ToRedisArgs for Filters {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.min_idle_time.write_redis_args(out);
        self.start.write_redis_args(out);
        self.end.write_redis_args(out);
        self.count.write_redis_args(out);
        self.consumer.write_redis_args(out);
    }
}
/// Redis Type: BLOCK XREAD::Block
pub struct Block(i64);

impl crate::types::ToRedisArgs for Block {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "BLOCK".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Block: Streams
pub struct Streams {
    /// key
    pub key: String,
    /// id
    pub id: String,
}

impl crate::types::ToRedisArgs for Streams {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "STREAMS".write_redis_args(out);
        self.key.write_redis_args(out);
        self.id.write_redis_args(out);
    }
}
/// Redis Block: Noack
pub struct Noack {
}

impl crate::types::ToRedisArgs for Noack {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "NOACK".write_redis_args(out);
    }
}
/// Redis Type: LastId XSETID::LastId
pub struct LastId(String);

impl crate::types::ToRedisArgs for LastId {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Type: ENTRIESADDED XSETID::Entriesadded
pub struct Entriesadded(i64);

impl crate::types::ToRedisArgs for Entriesadded {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "ENTRIESADDED".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: MAXDELETEDID XSETID::Maxdeletedid
pub struct Maxdeletedid(String);

impl crate::types::ToRedisArgs for Maxdeletedid {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "MAXDELETEDID".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: IndexUnit
pub enum IndexUnit {
    /// BYTE
    Byte,
    /// BIT
    Bit,
}

impl crate::types::ToRedisArgs for IndexUnit {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        match self {
            IndexUnit::Byte => "BYTE".write_redis_args(out),
            IndexUnit::Bit => "BIT".write_redis_args(out),
        }
    }
}
/// Redis Type: WriteOperation
pub enum WriteOperation {
    /// SET
    Set {encoding: String, offset: i64, value: i64},
    /// INCRBY
    Incrby {encoding: String, offset: i64, increment: i64},
}

impl crate::types::ToRedisArgs for WriteOperation {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        match self {
            WriteOperation::Set{encoding, offset, value} => {
                "SET".write_redis_args(out);
                encoding.write_redis_args(out);
                offset.write_redis_args(out);
                value.write_redis_args(out);
            },
            WriteOperation::Incrby{encoding, offset, increment} => {
                "INCRBY".write_redis_args(out);
                encoding.write_redis_args(out);
                offset.write_redis_args(out);
                increment.write_redis_args(out);
            },
        }
    }
}
/// Redis Type: Overflow
pub enum Overflow {
    /// WRAP
    Wrap,
    /// SAT
    Sat,
    /// FAIL
    Fail,
}

impl crate::types::ToRedisArgs for Overflow {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        "OVERFLOW".write_redis_args(out);
        match self {
            Overflow::Wrap => "WRAP".write_redis_args(out),
            Overflow::Sat => "SAT".write_redis_args(out),
            Overflow::Fail => "FAIL".write_redis_args(out),
        }
    }
}
/// Redis Type: Bit BITPOS::Bit
pub struct Bit(i64);

impl crate::types::ToRedisArgs for Bit {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.0.write_redis_args(out);
    }
}
/// Redis Block: EndIndex
pub struct EndIndex {
    /// end
    pub end: i64,
    /// index_unit
    pub index_unit: crate::generated::types::IndexUnit,
}

impl crate::types::ToRedisArgs for EndIndex {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + crate::types::RedisWrite,
    {
        self.end.write_redis_args(out);
        self.index_unit.write_redis_args(out);
    }
}
pub mod set {
    /// Redis Type: Condition
    pub enum Condition {
        /// NX
        Nx,
        /// XX
        Xx,
    }

    impl crate::types::ToRedisArgs for Condition {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            match self {
                Condition::Nx => "NX".write_redis_args(out),
                Condition::Xx => "XX".write_redis_args(out),
            }
        }
    }
    /// Redis Block: Get
    pub struct Get {
    }

    impl crate::types::ToRedisArgs for Get {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            "GET".write_redis_args(out);
        }
    }
    /// Redis Type: Expiration
    pub enum Expiration {
        /// EX
        Ex(i64),
        /// PX
        Px(i64),
        /// KEEPTTL
        Keepttl,
    }

    impl crate::types::ToRedisArgs for Expiration {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            match self {
                Expiration::Ex(inner) => {
                    "EX".write_redis_args(out);
                    inner.write_redis_args(out);
                },
                Expiration::Px(inner) => {
                    "PX".write_redis_args(out);
                    inner.write_redis_args(out);
                },
                Expiration::Keepttl => "KEEPTTL".write_redis_args(out),
            }
        }
    }
}
pub mod r#move {
    /// Redis Type: Db MOVE::Db
    pub struct Db(i64);

    impl crate::types::ToRedisArgs for Db {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            self.0.write_redis_args(out);
        }
    }
}
pub mod client_pause {
    /// Redis Type: Mode
    pub enum Mode {
        /// WRITE
        Write,
        /// ALL
        All,
    }

    impl crate::types::ToRedisArgs for Mode {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            match self {
                Mode::Write => "WRITE".write_redis_args(out),
                Mode::All => "ALL".write_redis_args(out),
            }
        }
    }
}
pub mod geosearch {
    /// Redis Type: By
    pub enum By {
        /// Unknown
        Circle {radius: f64, unit: crate::generated::types::Unit},
        /// Unknown
        Box {width: f64, height: f64, unit: crate::generated::types::Unit},
    }

    impl crate::types::ToRedisArgs for By {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            match self {
                By::Circle{radius, unit} => {
                    radius.write_redis_args(out);
                    unit.write_redis_args(out);
                },
                By::Box{width, height, unit} => {
                    width.write_redis_args(out);
                    height.write_redis_args(out);
                    unit.write_redis_args(out);
                },
            }
        }
    }
}
pub mod client_kill {
    /// Redis Type: Type
    pub enum Type {
        /// NORMAL
        Normal,
        /// MASTER
        Master,
        /// SLAVE
        Slave,
        /// REPLICA
        Replica,
        /// PUBSUB
        Pubsub,
    }

    impl crate::types::ToRedisArgs for Type {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            "TYPE".write_redis_args(out);
            match self {
                Type::Normal => "NORMAL".write_redis_args(out),
                Type::Master => "MASTER".write_redis_args(out),
                Type::Slave => "SLAVE".write_redis_args(out),
                Type::Replica => "REPLICA".write_redis_args(out),
                Type::Pubsub => "PUBSUB".write_redis_args(out),
            }
        }
    }
}
pub mod lpop {
    /// Redis Type: Count LPOP::Count
    pub struct Count(i64);

    impl crate::types::ToRedisArgs for Count {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            self.0.write_redis_args(out);
        }
    }
}
pub mod xgroup_create {
    /// Redis Type: Id
    pub enum Id {
        /// Unknown
        Id(String),
        /// $
        LastId,
    }

    impl crate::types::ToRedisArgs for Id {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            match self {
                Id::Id(inner) => {
                    inner.write_redis_args(out);
                },
                Id::LastId => "$".write_redis_args(out),
            }
        }
    }
}
pub mod script_debug {
    /// Redis Type: Mode
    pub enum Mode {
        /// YES
        Yes,
        /// SYNC
        Sync,
        /// NO
        No,
    }

    impl crate::types::ToRedisArgs for Mode {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            match self {
                Mode::Yes => "YES".write_redis_args(out),
                Mode::Sync => "SYNC".write_redis_args(out),
                Mode::No => "NO".write_redis_args(out),
            }
        }
    }
}
pub mod client_list {
    /// Redis Type: Type
    pub enum Type {
        /// NORMAL
        Normal,
        /// MASTER
        Master,
        /// REPLICA
        Replica,
        /// PUBSUB
        Pubsub,
    }

    impl crate::types::ToRedisArgs for Type {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            "TYPE".write_redis_args(out);
            match self {
                Type::Normal => "NORMAL".write_redis_args(out),
                Type::Master => "MASTER".write_redis_args(out),
                Type::Replica => "REPLICA".write_redis_args(out),
                Type::Pubsub => "PUBSUB".write_redis_args(out),
            }
        }
    }
    /// Redis Block: Id
    pub struct Id {
        /// client_id
        pub client_id: i64,
    }

    impl crate::types::ToRedisArgs for Id {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            "ID".write_redis_args(out);
            self.client_id.write_redis_args(out);
        }
    }
}
pub mod bitop {
    /// Redis Type: Operation BITOP::Operation
    pub struct Operation(String);

    impl crate::types::ToRedisArgs for Operation {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            self.0.write_redis_args(out);
        }
    }
}
pub mod blmove {
    /// Redis Type: Timeout BLMOVE::Timeout
    pub struct Timeout(f64);

    impl crate::types::ToRedisArgs for Timeout {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            self.0.write_redis_args(out);
        }
    }
}
pub mod hrandfield {
    /// Redis Block: Options
    pub struct Options {
        /// count
        pub count: i64,
        /// withvalues
        pub withvalues: bool,
    }

    impl crate::types::ToRedisArgs for Options {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            self.count.write_redis_args(out);
            if self.withvalues {
                "WITHVALUES".write_redis_args(out);
            }
        }
    }
}
pub mod zrange {
    /// Redis Type: Start ZRANGE::Start
    pub struct Start(String);

    impl crate::types::ToRedisArgs for Start {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            self.0.write_redis_args(out);
        }
    }
    /// Redis Type: Stop ZRANGE::Stop
    pub struct Stop(String);

    impl crate::types::ToRedisArgs for Stop {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            self.0.write_redis_args(out);
        }
    }
}
pub mod incrbyfloat {
    /// Redis Type: Increment INCRBYFLOAT::Increment
    pub struct Increment(f64);

    impl crate::types::ToRedisArgs for Increment {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            self.0.write_redis_args(out);
        }
    }
}
pub mod psubscribe {
    /// Redis Block: Pattern
    pub struct Pattern {
        /// pattern
        pub pattern: String,
    }

    impl crate::types::ToRedisArgs for Pattern {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            self.pattern.write_redis_args(out);
        }
    }
}
pub mod bitfield {
    /// Redis Type: Operation
    pub enum Operation {
        /// GET
        Get {encoding: String, offset: i64},
        /// Unknown
        Write {wrap_sat_fail: crate::generated::types::Overflow, write_operation: crate::generated::types::WriteOperation},
    }

    impl crate::types::ToRedisArgs for Operation {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            match self {
                Operation::Get{encoding, offset} => {
                    "GET".write_redis_args(out);
                    encoding.write_redis_args(out);
                    offset.write_redis_args(out);
                },
                Operation::Write{wrap_sat_fail, write_operation} => {
                    wrap_sat_fail.write_redis_args(out);
                    write_operation.write_redis_args(out);
                },
            }
        }
    }
}
pub mod xack {
    /// Redis Type: Id XACK::Id
    pub struct Id(String);

    impl crate::types::ToRedisArgs for Id {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            self.0.write_redis_args(out);
        }
    }
}
pub mod xreadgroup {
    /// Redis Block: Group
    pub struct Group {
        /// group
        pub group: String,
        /// consumer
        pub consumer: String,
    }

    impl crate::types::ToRedisArgs for Group {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            "GROUP".write_redis_args(out);
            self.group.write_redis_args(out);
            self.consumer.write_redis_args(out);
        }
    }
}
pub mod bitfield_ro {
    /// Redis Block: Get
    pub struct Get {
        /// encoding
        pub encoding: String,
        /// offset
        pub offset: i64,
    }

    impl crate::types::ToRedisArgs for Get {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            "GET".write_redis_args(out);
            self.encoding.write_redis_args(out);
            self.offset.write_redis_args(out);
        }
    }
}
pub mod zlexcount {
    /// Redis Type: Min ZLEXCOUNT::Min
    pub struct Min(String);

    impl crate::types::ToRedisArgs for Min {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            self.0.write_redis_args(out);
        }
    }
    /// Redis Type: Max ZLEXCOUNT::Max
    pub struct Max(String);

    impl crate::types::ToRedisArgs for Max {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            self.0.write_redis_args(out);
        }
    }
}
pub mod bitcount {
    /// Redis Block: Index
    pub struct Index {
        /// start
        pub start: i64,
        /// end
        pub end: i64,
        /// index_unit
        pub index_unit: crate::generated::types::IndexUnit,
    }

    impl crate::types::ToRedisArgs for Index {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            self.start.write_redis_args(out);
            self.end.write_redis_args(out);
            self.index_unit.write_redis_args(out);
        }
    }
}
pub mod cluster_failover {
    /// Redis Type: Options
    pub enum Options {
        /// FORCE
        Force,
        /// TAKEOVER
        Takeover,
    }

    impl crate::types::ToRedisArgs for Options {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            match self {
                Options::Force => "FORCE".write_redis_args(out),
                Options::Takeover => "TAKEOVER".write_redis_args(out),
            }
        }
    }
}
pub mod setbit {
    /// Redis Type: Value SETBIT::Value
    pub struct Value(i64);

    impl crate::types::ToRedisArgs for Value {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            self.0.write_redis_args(out);
        }
    }
}
pub mod bitpos {
    /// Redis Block: Index
    pub struct Index {
        /// start
        pub start: i64,
        /// end_index
        pub end_index: crate::generated::types::EndIndex,
    }

    impl crate::types::ToRedisArgs for Index {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            self.start.write_redis_args(out);
            self.end_index.write_redis_args(out);
        }
    }
}
pub mod failover {
    /// Redis Type: TIMEOUT FAILOVER::Timeout
    pub struct Timeout(i64);

    impl crate::types::ToRedisArgs for Timeout {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            "TIMEOUT".write_redis_args(out);
            self.0.write_redis_args(out);
        }
    }
}
pub mod georadius {
    /// Redis Block: Count
    pub struct Count {
        /// count
        pub count: crate::generated::types::Count,
        /// any
        pub any: bool,
    }

    impl crate::types::ToRedisArgs for Count {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            self.count.write_redis_args(out);
            if self.any {
                "ANY".write_redis_args(out);
            }
        }
    }
}
pub mod sintercard {
    /// Redis Type: LIMIT SINTERCARD::Limit
    pub struct Limit(i64);

    impl crate::types::ToRedisArgs for Limit {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            "LIMIT".write_redis_args(out);
            self.0.write_redis_args(out);
        }
    }
}
pub mod geosearchstore {
    /// Redis Block: Storedist
    pub struct Storedist {
    }

    impl crate::types::ToRedisArgs for Storedist {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            "STOREDIST".write_redis_args(out);
        }
    }
}
pub mod linsert {
    /// Redis Type: Where
    pub enum Where {
        /// BEFORE
        Before,
        /// AFTER
        After,
    }

    impl crate::types::ToRedisArgs for Where {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            match self {
                Where::Before => "BEFORE".write_redis_args(out),
                Where::After => "AFTER".write_redis_args(out),
            }
        }
    }
}
pub mod xrange {
    /// Redis Type: End XRANGE::End
    pub struct End(String);

    impl crate::types::ToRedisArgs for End {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            self.0.write_redis_args(out);
        }
    }
}
pub mod bzmpop {
    /// Redis Type: Where
    pub enum Where {
        /// MIN
        Min,
        /// MAX
        Max,
    }

    impl crate::types::ToRedisArgs for Where {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            match self {
                Where::Min => "MIN".write_redis_args(out),
                Where::Max => "MAX".write_redis_args(out),
            }
        }
    }
}
pub mod cluster_setslot {
    /// Redis Type: Subcommand
    pub enum Subcommand {
        /// IMPORTING
        Importing(String),
        /// MIGRATING
        Migrating(String),
        /// NODE
        Node(String),
        /// STABLE
        Stable,
    }

    impl crate::types::ToRedisArgs for Subcommand {
        fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + crate::types::RedisWrite,
        {
            match self {
                Subcommand::Importing(inner) => {
                    "IMPORTING".write_redis_args(out);
                    inner.write_redis_args(out);
                },
                Subcommand::Migrating(inner) => {
                    "MIGRATING".write_redis_args(out);
                    inner.write_redis_args(out);
                },
                Subcommand::Node(inner) => {
                    "NODE".write_redis_args(out);
                    inner.write_redis_args(out);
                },
                Subcommand::Stable => "STABLE".write_redis_args(out),
            }
        }
    }
}
