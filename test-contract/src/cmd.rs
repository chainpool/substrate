use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "substrate-cli",
    about = "Substrate CLI",
    author = "chainpool",
    version = "0.1.0"
)]
pub struct Substrate {
    #[structopt(short = "u", long = "url", default_value = "http://127.0.0.1")]
    /// Specify the address of node
    pub url: String,
    #[structopt(short = "p", long = "port", default_value = "9944")]
    /// Specify the port number
    pub port: String,
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt, Debug, Clone)]
pub enum Command {
    #[structopt(name = "flipper")]
    Flipper(Flipper),
}

#[derive(StructOpt, Debug, Clone)]
pub enum Flipper {
    #[structopt(name = "put_code")]
    PutCode,
    #[structopt(name = "instantiate")]
    Instantiate,
    #[structopt(name = "get")]
    Get,
    #[structopt(name = "flip")]
    Flip,
}
