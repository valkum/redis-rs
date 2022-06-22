use crate::types::{FromRedisValue, NumericBehavior, RedisResult, ToRedisArgs, RedisWrite, Expiry};
use crate::connection::{Connection, ConnectionLike, Msg};
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
pub enum Condition {
    Nx,
    Xx,
    Gt,
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
pub enum KeyOrEmptyString {
    Key(String),
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
pub enum Authentication {
    Auth(String),
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
pub struct Limit {
    pub offset: i64,
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
pub enum Order {
    Asc,
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
pub enum Expiration {
    Ex(i64),
    Px(i64),
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
pub enum Wherefrom {
    Left,
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
pub enum Whereto {
    Left,
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
pub enum Where {
    Left,
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
pub enum Comparison {
    Gt,
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
pub enum Aggregate {
    Sum,
    Min,
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
pub enum Sortby {
    Byscore,
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
pub enum Mode {
    Yes,
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
pub enum Enabled {
    On,
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
pub enum OnOffSkip {
    On,
    Off,
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
pub enum Status {
    On,
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
pub enum TimeoutError {
    Timeout,
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
pub enum Operation {
    Count(i64),
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
pub enum Filterby {
    Module(String),
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
pub struct To {
    pub host: String,
    pub port: i64,
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
pub enum Async {
    Async,
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
pub struct Config {
    pub name: String,
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
pub struct Args {
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
pub enum NosaveSave {
    Nosave,
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
pub enum Policy {
    Flush,
    Append,
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
pub enum Options {
    Force,
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
pub enum HardSoft {
    Hard,
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
pub enum Subcommand {
    Importing(String),
    Migrating(String),
    Node(String),
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
pub enum Unit {
    M,
    Km,
    Ft,
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
pub enum From {
    Frommember(String),
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
pub enum By {
    Circle {radius: f64},
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
pub enum IdOrAuto {
    Star,
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
pub struct Full {
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
pub struct Streams {
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
pub struct Group {
    pub group: String,
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
pub struct Get {
    pub encoding: String,
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
