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
    #[structopt(name = "erc20")]
    ERC20(ERC20),
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

#[derive(StructOpt, Debug, Clone)]
pub enum ERC20 {
    #[structopt(name = "put_code")]
    PutCode,
    #[structopt(name = "instantiate")]
    Instantiate,
    #[structopt(name = "total_supply")]
    TotalSupply,
    #[structopt(name = "balance_of")]
    BalanceOf,
    #[structopt(name = "transfer")]
    Transfer,
}
