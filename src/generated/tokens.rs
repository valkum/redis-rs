use crate::types::{FromRedisValue, NumericBehavior, RedisResult, ToRedisArgs, RedisWrite, Expiry};
use crate::connection::{Connection, ConnectionLike, Msg};
/// Redis Type: DB
pub struct Db(i64);

impl ToRedisArgs for Db {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        "DB".write_redis_args(out);
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

impl ToRedisArgs for Condition {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        match self {
            Condition::Nx => "NX".write_redis_args(out),
            Condition::Xx => "XX".write_redis_args(out),
            Condition::Gt => "GT".write_redis_args(out),
            Condition::Lt => "LT".write_redis_args(out),
        }
    }
}
/// Redis Type: KeyOrEmptyString
pub enum KeyOrEmptyString {
    /// Unknown
    Key(String),
    /// 
    EmptyString,
}

impl ToRedisArgs for KeyOrEmptyString {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        match self {
            KeyOrEmptyString::Key(inner) => {
                inner.write_redis_args(out);
            },
            KeyOrEmptyString::EmptyString => "".write_redis_args(out),
        }
    }
}
/// Redis Type: Authentication
pub enum Authentication {
    /// AUTH
    Auth(String),
    /// AUTH2
    Auth2 {username: String, password: String},
}

impl ToRedisArgs for Authentication {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
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
/// Redis Type: IDLETIME
pub struct Idletime(i64);

impl ToRedisArgs for Idletime {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        "IDLETIME".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: FREQ
pub struct Freq(i64);

impl ToRedisArgs for Freq {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        "FREQ".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: COUNT
pub struct Count(i64);

impl ToRedisArgs for Count {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        "COUNT".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: TYPE
pub struct Type(String);

impl ToRedisArgs for Type {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        "TYPE".write_redis_args(out);
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

impl ToRedisArgs for Limit {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        self.offset.write_redis_args(out);
        self.count.write_redis_args(out);
    }
}
/// Redis Type: Order
pub enum Order {
    /// ASC
    Asc,
    /// DESC
    Desc,
}

impl ToRedisArgs for Order {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        match self {
            Order::Asc => "ASC".write_redis_args(out),
            Order::Desc => "DESC".write_redis_args(out),
        }
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

impl ToRedisArgs for Expiration {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
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
/// Redis Type: MINMATCHLEN
pub struct Minmatchlen(i64);

impl ToRedisArgs for Minmatchlen {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        "MINMATCHLEN".write_redis_args(out);
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

impl ToRedisArgs for Wherefrom {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
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

impl ToRedisArgs for Whereto {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        match self {
            Whereto::Left => "LEFT".write_redis_args(out),
            Whereto::Right => "RIGHT".write_redis_args(out),
        }
    }
}
/// Redis Type: Where
pub enum Where {
    /// LEFT
    Left,
    /// RIGHT
    Right,
}

impl ToRedisArgs for Where {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        match self {
            Where::Left => "LEFT".write_redis_args(out),
            Where::Right => "RIGHT".write_redis_args(out),
        }
    }
}
/// Redis Type: RANK
pub struct Rank(i64);

impl ToRedisArgs for Rank {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        "RANK".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: MAXLEN
pub struct Maxlen(i64);

impl ToRedisArgs for Maxlen {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        "MAXLEN".write_redis_args(out);
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

impl ToRedisArgs for Comparison {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        match self {
            Comparison::Gt => "GT".write_redis_args(out),
            Comparison::Lt => "LT".write_redis_args(out),
        }
    }
}
/// Redis Type: WEIGHTS
pub struct Weights(i64);

impl ToRedisArgs for Weights {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
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

impl ToRedisArgs for Aggregate {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        match self {
            Aggregate::Sum => "SUM".write_redis_args(out),
            Aggregate::Min => "MIN".write_redis_args(out),
            Aggregate::Max => "MAX".write_redis_args(out),
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

impl ToRedisArgs for Sortby {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        match self {
            Sortby::Byscore => "BYSCORE".write_redis_args(out),
            Sortby::Bylex => "BYLEX".write_redis_args(out),
        }
    }
}
/// Redis Type: Mode
pub enum Mode {
    /// YES
    Yes,
    /// NO
    No,
}

impl ToRedisArgs for Mode {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        match self {
            Mode::Yes => "YES".write_redis_args(out),
            Mode::No => "NO".write_redis_args(out),
        }
    }
}
/// Redis Type: ID
pub struct Id(i64);

impl ToRedisArgs for Id {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        "ID".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: USER
pub struct User(String);

impl ToRedisArgs for User {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        "USER".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: ADDR
pub struct Addr(String);

impl ToRedisArgs for Addr {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        "ADDR".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: LADDR
pub struct Laddr(String);

impl ToRedisArgs for Laddr {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        "LADDR".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: SKIPME
pub struct Skipme(String);

impl ToRedisArgs for Skipme {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
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

impl ToRedisArgs for Enabled {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
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

impl ToRedisArgs for OnOffSkip {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        match self {
            OnOffSkip::On => "ON".write_redis_args(out),
            OnOffSkip::Off => "OFF".write_redis_args(out),
            OnOffSkip::Skip => "SKIP".write_redis_args(out),
        }
    }
}
/// Redis Type: Status
pub enum Status {
    /// ON
    On,
    /// OFF
    Off,
}

impl ToRedisArgs for Status {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        match self {
            Status::On => "ON".write_redis_args(out),
            Status::Off => "OFF".write_redis_args(out),
        }
    }
}
/// Redis Type: REDIRECT
pub struct Redirect(i64);

impl ToRedisArgs for Redirect {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        "REDIRECT".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: PREFIX
pub struct Prefix(String);

impl ToRedisArgs for Prefix {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        "PREFIX".write_redis_args(out);
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

impl ToRedisArgs for TimeoutError {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        match self {
            TimeoutError::Timeout => "TIMEOUT".write_redis_args(out),
            TimeoutError::Error => "ERROR".write_redis_args(out),
        }
    }
}
/// Redis Type: Operation
pub enum Operation {
    /// Unknown
    Count(i64),
    /// RESET
    Reset,
}

impl ToRedisArgs for Operation {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        match self {
            Operation::Count(inner) => {
                inner.write_redis_args(out);
            },
            Operation::Reset => "RESET".write_redis_args(out),
        }
    }
}
/// Redis Type: Filterby
pub enum Filterby {
    /// MODULE
    Module(String),
    /// ACLCAT
    Aclcat(String),
}

impl ToRedisArgs for Filterby {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        match self {
            Filterby::Module(inner) => {
                "MODULE".write_redis_args(out);
                inner.write_redis_args(out);
            },
            Filterby::Aclcat(inner) => {
                "ACLCAT".write_redis_args(out);
                inner.write_redis_args(out);
            },
        }
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

impl ToRedisArgs for To {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        self.host.write_redis_args(out);
        self.port.write_redis_args(out);
        if self.force {
            "FORCE".write_redis_args(out);
        }
    }
}
/// Redis Type: TIMEOUT
pub struct Timeout(i64);

impl ToRedisArgs for Timeout {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        "TIMEOUT".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: Async
pub enum Async {
    /// ASYNC
    Async,
    /// SYNC
    Sync,
}

impl ToRedisArgs for Async {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        match self {
            Async::Async => "ASYNC".write_redis_args(out),
            Async::Sync => "SYNC".write_redis_args(out),
        }
    }
}
/// Redis Type: VERSION
pub struct Version(i64);

impl ToRedisArgs for Version {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        "VERSION".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: SAMPLES
pub struct Samples(i64);

impl ToRedisArgs for Samples {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        "SAMPLES".write_redis_args(out);
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

impl ToRedisArgs for Config {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        self.name.write_redis_args(out);
        self.value.write_redis_args(out);
    }
}
/// Redis Block: Args
pub struct Args {
    /// arg
    pub arg: String,
}

impl ToRedisArgs for Args {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        self.arg.write_redis_args(out);
    }
}
/// Redis Type: NosaveSave
pub enum NosaveSave {
    /// NOSAVE
    Nosave,
    /// SAVE
    Save,
}

impl ToRedisArgs for NosaveSave {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        match self {
            NosaveSave::Nosave => "NOSAVE".write_redis_args(out),
            NosaveSave::Save => "SAVE".write_redis_args(out),
        }
    }
}
/// Redis Type: LIBRARYNAME
pub struct Libraryname(String);

impl ToRedisArgs for Libraryname {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        "LIBRARYNAME".write_redis_args(out);
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

impl ToRedisArgs for Policy {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        match self {
            Policy::Flush => "FLUSH".write_redis_args(out),
            Policy::Append => "APPEND".write_redis_args(out),
            Policy::Replace => "REPLACE".write_redis_args(out),
        }
    }
}
/// Redis Type: Options
pub enum Options {
    /// FORCE
    Force,
    /// TAKEOVER
    Takeover,
}

impl ToRedisArgs for Options {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        match self {
            Options::Force => "FORCE".write_redis_args(out),
            Options::Takeover => "TAKEOVER".write_redis_args(out),
        }
    }
}
/// Redis Type: HardSoft
pub enum HardSoft {
    /// HARD
    Hard,
    /// SOFT
    Soft,
}

impl ToRedisArgs for HardSoft {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        match self {
            HardSoft::Hard => "HARD".write_redis_args(out),
            HardSoft::Soft => "SOFT".write_redis_args(out),
        }
    }
}
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

impl ToRedisArgs for Subcommand {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
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

impl ToRedisArgs for Unit {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        match self {
            Unit::M => "M".write_redis_args(out),
            Unit::Km => "KM".write_redis_args(out),
            Unit::Ft => "FT".write_redis_args(out),
            Unit::Mi => "MI".write_redis_args(out),
        }
    }
}
/// Redis Type: From
pub enum From {
    /// FROMMEMBER
    Frommember(String),
    /// FROMLONLAT
    Fromlonlat {longitude: f64, latitude: f64},
}

impl ToRedisArgs for From {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
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
/// Redis Type: By
pub enum By {
    /// Unknown
    Circle {radius: f64},
    /// Unknown
    Box {width: f64, height: f64},
}

impl ToRedisArgs for By {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        match self {
            By::Circle{radius} => {
                radius.write_redis_args(out);
            },
            By::Box{width, height} => {
                width.write_redis_args(out);
                height.write_redis_args(out);
            },
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

impl ToRedisArgs for IdOrAuto {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        match self {
            IdOrAuto::Star => "*".write_redis_args(out),
            IdOrAuto::Id(inner) => {
                inner.write_redis_args(out);
            },
        }
    }
}
/// Redis Type: IDLE
pub struct Idle(i64);

impl ToRedisArgs for Idle {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        "IDLE".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: RETRYCOUNT
pub struct Retrycount(i64);

impl ToRedisArgs for Retrycount {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        "RETRYCOUNT".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: ENTRIESREAD
pub struct Entriesread(i64);

impl ToRedisArgs for Entriesread {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        "ENTRIESREAD".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Block: Full
pub struct Full {
    /// count
    pub count: Count,
}

impl ToRedisArgs for Full {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        self.count.write_redis_args(out);
    }
}
/// Redis Type: BLOCK
pub struct Block(i64);

impl ToRedisArgs for Block {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        "BLOCK".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Block: Streams
pub struct Streams {
    /// id
    pub id: String,
}

impl ToRedisArgs for Streams {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        self.id.write_redis_args(out);
    }
}
/// Redis Block: Group
pub struct Group {
    /// group
    pub group: String,
    /// consumer
    pub consumer: String,
}

impl ToRedisArgs for Group {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        self.group.write_redis_args(out);
        self.consumer.write_redis_args(out);
    }
}
/// Redis Type: ENTRIESADDED
pub struct Entriesadded(i64);

impl ToRedisArgs for Entriesadded {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        "ENTRIESADDED".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Type: MAXDELETEDID
pub struct Maxdeletedid(String);

impl ToRedisArgs for Maxdeletedid {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        "MAXDELETEDID".write_redis_args(out);
        self.0.write_redis_args(out);
    }
}
/// Redis Block: Get
pub struct Get {
    /// encoding
    pub encoding: String,
    /// offset
    pub offset: i64,
}

impl ToRedisArgs for Get {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        self.encoding.write_redis_args(out);
        self.offset.write_redis_args(out);
    }
}
