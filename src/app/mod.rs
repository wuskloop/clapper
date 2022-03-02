pub mod app;
mod cmd;

type Action = fn() -> anyhow::Result<()>;

pub struct Argument {
    name: String,
    usage: String,
    value: Value,
    hidden: bool,
    required: bool,
    aliases: Vec<String>,
    env_vars: Vec<String>,
    description: String,
}

pub enum Value{
    Bool(bool),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    String(String)
}

pub struct Author {
    name: String,
    email: Option<String>,
}